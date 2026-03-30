//! SQLite implementation of the train-formation repository.
//!
//! This module provides [`SqlxTrainFormationRepository`] which persists
//! [`TrainFormation`] aggregates and their composition elements using
//! `sqlx` transactions.

use crate::core::domain::domain_error::DomainError;
use crate::core::domain::metadata::Metadata;
use crate::trains::domain::formation::formation_element::FormationElement;
use crate::trains::domain::formation::train_formation::TrainFormation;
use crate::trains::infrastructure::entities::{
    FormationCategoryRow, FormationElementDetailRow, FormationElementRow, PrototypeRow,
    TrainFormationRow, TrainFormationSummaryRow,
};
use crate::trains::infrastructure::mappers::{
    self, FormationCategoryView, FormationElementView, PrototypeGroupView, PrototypeView,
    TrainFormationDetail, TrainFormationSummary, TrainFormationView,
};
use chrono::Utc;
use sqlx::SqliteConnection;

/// Parameters for [`SqlxTrainFormationRepository::save_prototype`].
pub struct SavePrototypeParams<'a> {
    pub id: &'a str,
    pub railway_company_id: &'a str,
    pub series_code: &'a str,
    pub car_type: &'a str,
    pub service_level: Option<&'a str>,
    pub category: &'a str,
    pub is_motorized: bool,
    pub default_is_dummy: bool,
    pub notes: Option<&'a str>,
}

/// Repository for [`TrainFormation`] backed by an SQLite transaction.
pub struct SqlxTrainFormationRepository<'conn> {
    conn: &'conn mut SqliteConnection,
}

impl<'conn> SqlxTrainFormationRepository<'conn> {
    /// Create a new repository bound to an existing transaction.
    pub fn new(conn: &'conn mut SqliteConnection) -> Self {
        Self { conn }
    }

    // ── Formation CRUD ────────────────────────────────────────────────────────

    /// Upsert a [`TrainFormation`] — INSERT on new, UPDATE on existing.
    pub async fn save(&mut self, formation: &TrainFormation) -> Result<(), DomainError> {
        let now = Utc::now().to_rfc3339();
        let existing: Option<i64> =
            sqlx::query_scalar("SELECT version FROM train_formations WHERE id = ?")
                .bind(&formation.id)
                .fetch_optional(&mut *self.conn)
                .await
                .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        if existing.is_some() {
            sqlx::query(
                r#"UPDATE train_formations
                   SET name = ?, category_id = ?, start_year = ?, end_year = ?,
                       epoch = ?, notes = ?, updated_at = ?,
                       version = version + 1
                   WHERE id = ?"#,
            )
            .bind(&formation.name)
            .bind(&formation.category_id)
            .bind(formation.start_year)
            .bind(formation.end_year)
            .bind(&formation.epoch)
            .bind(&formation.notes)
            .bind(&now)
            .bind(&formation.id)
            .execute(&mut *self.conn)
            .await
            .map_err(|e| {
                if e.to_string().contains("UNIQUE") {
                    DomainError::BusinessRule(format!(
                        "A formation named '{}' already exists",
                        formation.name
                    ))
                } else {
                    DomainError::Infrastructure(e.to_string())
                }
            })?;
        } else {
            sqlx::query(
                r#"INSERT INTO train_formations
                   (id, name, category_id, start_year, end_year, epoch, notes,
                    created_at, updated_at, version)
                   VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 0)"#,
            )
            .bind(&formation.id)
            .bind(&formation.name)
            .bind(&formation.category_id)
            .bind(formation.start_year)
            .bind(formation.end_year)
            .bind(&formation.epoch)
            .bind(&formation.notes)
            .bind(&now)
            .bind(&now)
            .execute(&mut *self.conn)
            .await
            .map_err(|e| {
                if e.to_string().contains("UNIQUE") {
                    DomainError::BusinessRule(format!(
                        "A formation named '{}' already exists",
                        formation.name
                    ))
                } else {
                    DomainError::Infrastructure(e.to_string())
                }
            })?;
        }
        Ok(())
    }

    /// Fetch a formation row + its elements by ID.
    pub async fn find_by_id_raw(&mut self, id: &str) -> Result<TrainFormation, DomainError> {
        let row: Option<TrainFormationRow> = sqlx::query_as(
            "SELECT id, name, category_id, start_year, end_year, epoch, notes,
                    created_at, updated_at, version
             FROM train_formations WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&mut *self.conn)
        .await
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        let row = row.ok_or_else(|| DomainError::NotFound {
            resource: "TrainFormation".into(),
            identifier: id.into(),
        })?;

        let elements: Vec<FormationElementRow> = sqlx::query_as(
            r#"SELECT id, formation_id, prototype_id, owned_rolling_stock_id,
                      snapshot_series_code, snapshot_company_name,
                      position_order, traction_override, created_at, updated_at
               FROM formation_elements
               WHERE formation_id = ?
               ORDER BY position_order ASC"#,
        )
        .bind(id)
        .fetch_all(&mut *self.conn)
        .await
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        let domain_elements: Vec<FormationElement> = elements
            .into_iter()
            .map(|e| FormationElement {
                id: e.id,
                prototype_id: e.prototype_id,
                owned_rolling_stock_id: e.owned_rolling_stock_id,
                position_order: e.position_order,
                traction_override: e.traction_override,
            })
            .collect();

        Ok(TrainFormation {
            id: row.id,
            name: row.name,
            category_id: row.category_id,
            start_year: row.start_year,
            end_year: row.end_year,
            epoch: row.epoch,
            notes: row.notes,
            elements: domain_elements,
            pending_events: Vec::new(),
            metadata: Metadata::default(),
        })
    }

    /// Delete a formation (elements cascade).
    pub async fn delete(&mut self, id: &str) -> Result<(), DomainError> {
        let rows_affected = sqlx::query("DELETE FROM train_formations WHERE id = ?")
            .bind(id)
            .execute(&mut *self.conn)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?
            .rows_affected();

        if rows_affected == 0 {
            return Err(DomainError::NotFound {
                resource: "TrainFormation".into(),
                identifier: id.into(),
            });
        }
        Ok(())
    }

    // ── View queries ──────────────────────────────────────────────────────────

    /// Return the full [`TrainFormationDetail`] with joined element data.
    pub async fn get_detail(&mut self, id: &str) -> Result<TrainFormationDetail, DomainError> {
        let row: Option<TrainFormationRow> = sqlx::query_as(
            "SELECT id, name, category_id, start_year, end_year, epoch, notes,
                    created_at, updated_at, version
             FROM train_formations WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&mut *self.conn)
        .await
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        let row = row.ok_or_else(|| DomainError::NotFound {
            resource: "TrainFormation".into(),
            identifier: id.into(),
        })?;

        let category = self.get_category_view(row.category_id.as_deref()).await?;

        let element_rows: Vec<FormationElementDetailRow> = sqlx::query_as(
            r#"SELECT
                   fe.id,
                   fe.formation_id,
                   fe.prototype_id,
                   fe.owned_rolling_stock_id,
                   fe.snapshot_series_code,
                   fe.snapshot_company_name,
                   fe.position_order,
                   fe.traction_override,
                   p.railway_company_id AS proto_railway_company_id,
                   rc.name AS proto_company_name,
                   p.series_code AS proto_series_code,
                   p.car_type AS proto_car_type,
                   p.service_level AS proto_service_level,
                   p.category AS proto_category,
                   p.is_motorized AS proto_is_motorized,
                   p.default_is_dummy AS proto_default_is_dummy,
                   p.is_custom AS proto_is_custom,
                   (SELECT COUNT(*) FROM owned_rolling_stocks ors
                    WHERE ors.prototype_id = fe.prototype_id) AS owned_count_for_prototype
               FROM formation_elements fe
               LEFT JOIN prototypes p ON p.id = fe.prototype_id
               LEFT JOIN railway_companies rc ON rc.id = p.railway_company_id
               WHERE fe.formation_id = ?
               ORDER BY fe.position_order ASC"#,
        )
        .bind(id)
        .fetch_all(&mut *self.conn)
        .await
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        let elements: Vec<FormationElementView> = element_rows
            .into_iter()
            .map(mappers::element_detail_row_to_view)
            .collect();

        let has_traction = elements.iter().any(|e| e.is_traction_slot);

        Ok(TrainFormationDetail {
            id: row.id,
            name: row.name,
            category,
            start_year: row.start_year,
            end_year: row.end_year,
            epoch: row.epoch,
            notes: row.notes,
            elements,
            has_traction,
        })
    }

    /// Return a list of formation summaries.
    pub async fn list_summaries(&mut self) -> Result<Vec<TrainFormationSummary>, DomainError> {
        let rows: Vec<TrainFormationSummaryRow> = sqlx::query_as(
            r#"SELECT
                   tf.id,
                   tf.name,
                   tf.category_id,
                   fc.name AS category_name,
                   tf.start_year,
                   tf.end_year,
                   tf.epoch,
                   COUNT(fe.id) AS element_count,
                   COUNT(fe.owned_rolling_stock_id) AS owned_count,
                   tf.version
               FROM train_formations tf
               LEFT JOIN formation_categories fc ON fc.id = tf.category_id
               LEFT JOIN formation_elements fe ON fe.formation_id = tf.id
               GROUP BY tf.id
               ORDER BY tf.name ASC"#,
        )
        .fetch_all(&mut *self.conn)
        .await
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        let mut summaries = Vec::with_capacity(rows.len());
        for row in rows {
            // Compute has_traction via the element join data
            let has_traction = self.compute_has_traction(row.id.as_str()).await?;
            let category = if let Some(ref cat_id) = row.category_id {
                row.category_name
                    .as_ref()
                    .map(|name| FormationCategoryView {
                        id: cat_id.clone(),
                        name: name.clone(),
                        is_custom: false,
                    })
            } else {
                None
            };
            summaries.push(mappers::summary_row_to_view(row, category, has_traction));
        }
        Ok(summaries)
    }

    /// Return a [`TrainFormationView`] (post-write response).
    pub async fn get_view(&mut self, id: &str) -> Result<TrainFormationView, DomainError> {
        let row: TrainFormationRow = sqlx::query_as(
            "SELECT id, name, category_id, start_year, end_year, epoch, notes,
                    created_at, updated_at, version
             FROM train_formations WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&mut *self.conn)
        .await
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?
        .ok_or_else(|| DomainError::NotFound {
            resource: "TrainFormation".into(),
            identifier: id.into(),
        })?;

        let category = self.get_category_view(row.category_id.as_deref()).await?;
        let element_count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM formation_elements WHERE formation_id = ?")
                .bind(id)
                .fetch_one(&mut *self.conn)
                .await
                .map_err(|e| DomainError::Infrastructure(e.to_string()))?;
        let has_traction = self.compute_has_traction(id).await?;
        Ok(mappers::formation_row_to_view(
            row,
            category,
            element_count,
            has_traction,
        ))
    }

    // ── Element CRUD ──────────────────────────────────────────────────────────

    /// Append a new element to a formation (position = current max + 1).
    pub async fn add_element(
        &mut self,
        formation_id: &str,
        element: &FormationElement,
    ) -> Result<(), DomainError> {
        // Compute next position
        let max_pos: Option<i32> = sqlx::query_scalar(
            "SELECT MAX(position_order) FROM formation_elements WHERE formation_id = ?",
        )
        .bind(formation_id)
        .fetch_optional(&mut *self.conn)
        .await
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?
        .flatten();

        let position = max_pos.map(|p| p + 1).unwrap_or(0);
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            r#"INSERT INTO formation_elements
               (id, formation_id, prototype_id, owned_rolling_stock_id,
                snapshot_series_code, snapshot_company_name,
                position_order, traction_override, created_at, updated_at)
               VALUES (?, ?, ?, ?, NULL, NULL, ?, ?, ?, ?)"#,
        )
        .bind(&element.id)
        .bind(formation_id)
        .bind(&element.prototype_id)
        .bind(&element.owned_rolling_stock_id)
        .bind(position)
        .bind(element.traction_override)
        .bind(&now)
        .bind(&now)
        .execute(&mut *self.conn)
        .await
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        Ok(())
    }

    /// Remove an element and shift subsequent elements' `position_order` down.
    pub async fn remove_element_and_shift(&mut self, element_id: &str) -> Result<(), DomainError> {
        // Fetch the position of the element being removed
        let pos: Option<i32> =
            sqlx::query_scalar("SELECT position_order FROM formation_elements WHERE id = ?")
                .bind(element_id)
                .fetch_optional(&mut *self.conn)
                .await
                .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        let pos = pos.ok_or_else(|| DomainError::NotFound {
            resource: "FormationElement".into(),
            identifier: element_id.into(),
        })?;

        let formation_id: String =
            sqlx::query_scalar("SELECT formation_id FROM formation_elements WHERE id = ?")
                .bind(element_id)
                .fetch_one(&mut *self.conn)
                .await
                .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        sqlx::query("DELETE FROM formation_elements WHERE id = ?")
            .bind(element_id)
            .execute(&mut *self.conn)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        // Shift all subsequent elements down by 1
        sqlx::query(
            "UPDATE formation_elements SET position_order = position_order - 1
             WHERE formation_id = ? AND position_order > ?",
        )
        .bind(&formation_id)
        .bind(pos)
        .execute(&mut *self.conn)
        .await
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        Ok(())
    }

    /// Atomically bulk-update `position_order` for all elements in a formation.
    pub async fn bulk_reorder(
        &mut self,
        formation_id: &str,
        ordered_ids: &[String],
    ) -> Result<(), DomainError> {
        for (idx, element_id) in ordered_ids.iter().enumerate() {
            sqlx::query(
                "UPDATE formation_elements SET position_order = ? WHERE id = ? AND formation_id = ?",
            )
            .bind(idx as i32)
            .bind(element_id)
            .bind(formation_id)
            .execute(&mut *self.conn)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?;
        }
        Ok(())
    }

    /// Assign or unassign an owned model to/from an element.
    ///
    /// When assigning, persists `snapshot_series_code` + `snapshot_company_name`
    /// from the prototype/company at assignment time (FR-020).
    /// When unassigning (`owned_id = None`), snapshots are cleared.
    pub async fn assign_rolling_stock(
        &mut self,
        element_id: &str,
        owned_id: Option<&str>,
    ) -> Result<FormationElementView, DomainError> {
        let now = Utc::now().to_rfc3339();

        if let Some(ref oid) = owned_id {
            // Fetch prototype snapshot data
            let snapshot: Option<(String, String)> = sqlx::query_as(
                r#"SELECT p.series_code, rc.name
                   FROM formation_elements fe
                   JOIN prototypes p ON p.id = fe.prototype_id
                   JOIN railway_companies rc ON rc.id = p.railway_company_id
                   WHERE fe.id = ?"#,
            )
            .bind(element_id)
            .fetch_optional(&mut *self.conn)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

            let (series_code, company_name) = snapshot.unwrap_or_default();

            sqlx::query(
                r#"UPDATE formation_elements
                   SET owned_rolling_stock_id = ?,
                       snapshot_series_code = ?,
                       snapshot_company_name = ?,
                       updated_at = ?
                   WHERE id = ?"#,
            )
            .bind(oid)
            .bind(&series_code)
            .bind(&company_name)
            .bind(&now)
            .bind(element_id)
            .execute(&mut *self.conn)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?;
        } else {
            sqlx::query(
                r#"UPDATE formation_elements
                   SET owned_rolling_stock_id = NULL,
                       snapshot_series_code = NULL,
                       snapshot_company_name = NULL,
                       updated_at = ?
                   WHERE id = ?"#,
            )
            .bind(&now)
            .bind(element_id)
            .execute(&mut *self.conn)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?;
        }

        self.get_element_view(element_id).await
    }

    /// Update the `traction_override` for an element.
    pub async fn set_traction_override(
        &mut self,
        element_id: &str,
        traction_override: i32,
    ) -> Result<FormationElementView, DomainError> {
        let now = Utc::now().to_rfc3339();
        let rows = sqlx::query(
            "UPDATE formation_elements SET traction_override = ?, updated_at = ? WHERE id = ?",
        )
        .bind(traction_override)
        .bind(&now)
        .bind(element_id)
        .execute(&mut *self.conn)
        .await
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?
        .rows_affected();

        if rows == 0 {
            return Err(DomainError::NotFound {
                resource: "FormationElement".into(),
                identifier: element_id.into(),
            });
        }
        self.get_element_view(element_id).await
    }

    /// Fetch a single element view (used as return value for write operations).
    pub async fn get_element_view(
        &mut self,
        element_id: &str,
    ) -> Result<FormationElementView, DomainError> {
        let row: Option<FormationElementDetailRow> = sqlx::query_as(
            r#"SELECT
                   fe.id,
                   fe.formation_id,
                   fe.prototype_id,
                   fe.owned_rolling_stock_id,
                   fe.snapshot_series_code,
                   fe.snapshot_company_name,
                   fe.position_order,
                   fe.traction_override,
                   p.railway_company_id AS proto_railway_company_id,
                   rc.name AS proto_company_name,
                   p.series_code AS proto_series_code,
                   p.car_type AS proto_car_type,
                   p.service_level AS proto_service_level,
                   p.category AS proto_category,
                   p.is_motorized AS proto_is_motorized,
                   p.default_is_dummy AS proto_default_is_dummy,
                   p.is_custom AS proto_is_custom,
                   (SELECT COUNT(*) FROM owned_rolling_stocks ors
                    WHERE ors.prototype_id = fe.prototype_id) AS owned_count_for_prototype
               FROM formation_elements fe
               LEFT JOIN prototypes p ON p.id = fe.prototype_id
               LEFT JOIN railway_companies rc ON rc.id = p.railway_company_id
               WHERE fe.id = ?"#,
        )
        .bind(element_id)
        .fetch_optional(&mut *self.conn)
        .await
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        let row = row.ok_or_else(|| DomainError::NotFound {
            resource: "FormationElement".into(),
            identifier: element_id.into(),
        })?;

        Ok(mappers::element_detail_row_to_view(row))
    }

    // ── Prototype queries ─────────────────────────────────────────────────────

    /// Search prototypes, optionally filtered by a query string.
    pub async fn search_prototypes(
        &mut self,
        query: Option<&str>,
    ) -> Result<Vec<PrototypeGroupView>, DomainError> {
        let rows: Vec<PrototypeRow> = if let Some(q) = query.filter(|s| !s.trim().is_empty()) {
            let pattern = format!("%{}%", q.to_lowercase());
            sqlx::query_as(
                r#"SELECT id, railway_company_id, series_code, car_type, service_level,
                          category, is_motorized, default_is_dummy, is_custom, notes, version
                   FROM prototypes
                   WHERE LOWER(series_code) LIKE ? OR LOWER(car_type) LIKE ?
                   ORDER BY railway_company_id, series_code"#,
            )
            .bind(&pattern)
            .bind(&pattern)
            .fetch_all(&mut *self.conn)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?
        } else {
            sqlx::query_as(
                r#"SELECT id, railway_company_id, series_code, car_type, service_level,
                          category, is_motorized, default_is_dummy, is_custom, notes, version
                   FROM prototypes
                   ORDER BY railway_company_id, series_code"#,
            )
            .fetch_all(&mut *self.conn)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?
        };

        // Group by railway company preserving ORDER BY order
        let mut group_order: Vec<String> = Vec::new();
        let mut groups: std::collections::HashMap<String, (String, Vec<PrototypeView>)> =
            std::collections::HashMap::new();
        for row in rows {
            let company_id = row.railway_company_id.clone();
            let company_name = self.get_company_name(&company_id).await?;
            let view = mappers::prototype_row_to_view(row, company_name.clone());
            if !groups.contains_key(&company_id) {
                group_order.push(company_id.clone());
                groups.insert(company_id.clone(), (company_name, Vec::new()));
            }
            if let Some(entry) = groups.get_mut(&company_id) {
                entry.1.push(view);
            }
        }

        Ok(group_order
            .into_iter()
            .filter_map(|id| {
                groups.remove(&id).map(|(name, protos)| PrototypeGroupView {
                    railway_company_id: id,
                    company_name: name,
                    prototypes: protos,
                })
            })
            .collect())
    }

    /// Save a custom prototype.
    pub async fn save_prototype(
        &mut self,
        params: SavePrototypeParams<'_>,
    ) -> Result<PrototypeView, DomainError> {
        let now = Utc::now().to_rfc3339();
        let company_name = self.get_company_name(params.railway_company_id).await?;

        sqlx::query(
            r#"INSERT INTO prototypes
               (id, railway_company_id, series_code, car_type, service_level,
                category, is_motorized, default_is_dummy, is_custom,
                notes, created_at, updated_at, version)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, 1, ?, ?, ?, 0)"#,
        )
        .bind(params.id)
        .bind(params.railway_company_id)
        .bind(params.series_code)
        .bind(params.car_type)
        .bind(params.service_level)
        .bind(params.category)
        .bind(i64::from(params.is_motorized))
        .bind(i64::from(params.default_is_dummy))
        .bind(params.notes)
        .bind(&now)
        .bind(&now)
        .execute(&mut *self.conn)
        .await
        .map_err(|e| {
            if e.to_string().contains("UNIQUE") {
                DomainError::BusinessRule(format!(
                    "A prototype with series code '{}' already exists for this company",
                    params.series_code
                ))
            } else {
                DomainError::Infrastructure(e.to_string())
            }
        })?;

        Ok(PrototypeView {
            id: params.id.to_string(),
            railway_company_id: params.railway_company_id.to_string(),
            company_name,
            series_code: params.series_code.to_string(),
            car_type: params.car_type.to_string(),
            service_level: params.service_level.map(|s| s.to_string()),
            category: params.category.to_string(),
            is_motorized: params.is_motorized,
            default_is_dummy: params.default_is_dummy,
            is_custom: true,
        })
    }

    // ── Category queries ──────────────────────────────────────────────────────

    /// Return all formation categories.
    pub async fn list_categories(&mut self) -> Result<Vec<FormationCategoryView>, DomainError> {
        let rows: Vec<FormationCategoryRow> = sqlx::query_as(
            "SELECT id, name, is_custom FROM formation_categories ORDER BY name ASC",
        )
        .fetch_all(&mut *self.conn)
        .await
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(mappers::category_row_to_view)
            .collect())
    }

    /// Create a custom formation category.
    pub async fn create_category(
        &mut self,
        id: &str,
        name: &str,
    ) -> Result<FormationCategoryView, DomainError> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "INSERT INTO formation_categories (id, name, is_custom, created_at) VALUES (?, ?, 1, ?)",
        )
        .bind(id)
        .bind(name)
        .bind(&now)
        .execute(&mut *self.conn)
        .await
        .map_err(|e| {
            if e.to_string().contains("UNIQUE") {
                DomainError::BusinessRule(format!("Category '{name}' already exists"))
            } else {
                DomainError::Infrastructure(e.to_string())
            }
        })?;

        Ok(FormationCategoryView {
            id: id.to_string(),
            name: name.to_string(),
            is_custom: true,
        })
    }

    // ── Private helpers ───────────────────────────────────────────────────────

    async fn get_category_view(
        &mut self,
        category_id: Option<&str>,
    ) -> Result<Option<FormationCategoryView>, DomainError> {
        let Some(id) = category_id else {
            return Ok(None);
        };
        let row: Option<FormationCategoryRow> =
            sqlx::query_as("SELECT id, name, is_custom FROM formation_categories WHERE id = ?")
                .bind(id)
                .fetch_optional(&mut *self.conn)
                .await
                .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        Ok(row.map(mappers::category_row_to_view))
    }

    async fn get_company_name(&mut self, company_id: &str) -> Result<String, DomainError> {
        let name: Option<String> =
            sqlx::query_scalar("SELECT name FROM railway_companies WHERE id = ?")
                .bind(company_id)
                .fetch_optional(&mut *self.conn)
                .await
                .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        name.ok_or_else(|| DomainError::NotFound {
            resource: "RailwayCompany".into(),
            identifier: company_id.into(),
        })
    }

    /// Check whether a formation has at least one effective traction slot.
    async fn compute_has_traction(&mut self, formation_id: &str) -> Result<bool, DomainError> {
        // A slot is a traction slot when:
        //   (is_motorized=1 AND default_is_dummy=0 AND traction_override != -1)
        //   OR traction_override = 1
        let count: i64 = sqlx::query_scalar(
            r#"SELECT COUNT(*)
               FROM formation_elements fe
               JOIN prototypes p ON p.id = fe.prototype_id
               WHERE fe.formation_id = ?
                 AND (
                   (p.is_motorized = 1 AND p.default_is_dummy = 0 AND fe.traction_override != -1)
                   OR fe.traction_override = 1
                 )"#,
        )
        .bind(formation_id)
        .fetch_one(&mut *self.conn)
        .await
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        Ok(count > 0)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// T080 Integration tests — train_formation_repo & prototype_repo
// ─────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::{SavePrototypeParams, SqlxTrainFormationRepository};
    use crate::core::domain::domain_error::DomainError;
    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
    use crate::trains::domain::formation::formation_element::FormationElement;
    use crate::trains::domain::formation::train_formation::TrainFormation;
    use chrono::Utc;
    use sqlx::SqlitePool;

    // ── Test helpers ──────────────────────────────────────────────────────────

    /// Insert the three railway companies referenced by the default prototype seeds.
    async fn insert_test_companies(pool: &SqlitePool) {
        for (id, name, reg, code) in [
            ("trn:railway-company:fs", "FS", "Ferrovie dello Stato", "IT"),
            (
                "trn:railway-company:sbb-cff-ffs",
                "SBB",
                "SBB CFF FFS",
                "CH",
            ),
            ("trn:railway-company:db", "DB", "Deutsche Bahn", "DE"),
        ] {
            sqlx::query(
                "INSERT OR IGNORE INTO railway_companies
                 (id, name, registered_company_name, country_code, status, operating_since)
                 VALUES (?, ?, ?, ?, 'ACTIVE', '1900-01-01')",
            )
            .bind(id)
            .bind(name)
            .bind(reg)
            .bind(code)
            .execute(pool)
            .await
            .unwrap_or_else(|e| panic!("insert {name} railway company: {e}"));
        }
    }

    /// Insert a motorised (non-dummy) locomotive prototype.
    async fn insert_locomotive(pool: &SqlitePool, id: &str, company_id: &str, series: &str) {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "INSERT INTO prototypes
             (id, railway_company_id, series_code, car_type, service_level,
              category, is_motorized, default_is_dummy, is_custom,
              created_at, updated_at, version)
             VALUES (?, ?, ?, 'Locomotive', NULL, 'Locomotive', 1, 0, 0, ?, ?, 0)",
        )
        .bind(id)
        .bind(company_id)
        .bind(series)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await
        .unwrap_or_else(|e| panic!("insert locomotive {id}: {e}"));
    }

    // ── train_formation_repo tests ────────────────────────────────────────────

    /// Insert a record and fetch it back; all fields must round-trip correctly.
    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_formation_roundtrip(pool: SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&pool).await.expect("create uow");
        let mut repo = SqlxTrainFormationRepository::new(&mut uow.tx);

        let formation = TrainFormation::create("tf-rt-1".into(), "Gottardo 1974".into())
            .expect("construct formation");
        repo.save(&formation).await.expect("save");

        let fetched = repo.find_by_id_raw("tf-rt-1").await.expect("find");
        assert_eq!(fetched.id, "tf-rt-1");
        assert_eq!(fetched.name, "Gottardo 1974");
        assert!(fetched.elements.is_empty());
    }

    /// Inserting two formations with the same name must return a `BusinessRule` error.
    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_formation_duplicate_name(pool: SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&pool).await.expect("create uow");
        let mut repo = SqlxTrainFormationRepository::new(&mut uow.tx);

        let f1 = TrainFormation::create("tf-dup-1".into(), "Duplicate Name".into()).expect("first");
        repo.save(&f1).await.expect("save first");

        let f2 =
            TrainFormation::create("tf-dup-2".into(), "Duplicate Name".into()).expect("second");
        let result = repo.save(&f2).await;

        assert!(result.is_err(), "duplicate name must fail");
        assert!(
            matches!(result.unwrap_err(), DomainError::BusinessRule(_)),
            "expected BusinessRule error"
        );
    }

    /// Updating metadata must persist changed fields and increment `version`.
    #[sqlx::test(migrations = "./migrations")]
    async fn test_update_formation_metadata(pool: SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&pool).await.expect("create uow");
        {
            let mut repo = SqlxTrainFormationRepository::new(&mut uow.tx);

            let mut formation =
                TrainFormation::create("tf-upd-1".into(), "Original Name".into()).expect("create");
            repo.save(&formation).await.expect("save initial");

            formation.rename("Updated Name".into()).expect("rename");
            formation
                .update_metadata(
                    None,
                    Some(1975),
                    Some(1985),
                    Some("IV".into()),
                    Some("Test notes".into()),
                )
                .expect("update metadata");
            repo.save(&formation).await.expect("save update");
        }
        uow.commit().await.expect("commit");

        // Verify updated fields and version increment via pool
        let name: String = sqlx::query_scalar("SELECT name FROM train_formations WHERE id = ?")
            .bind("tf-upd-1")
            .fetch_one(&pool)
            .await
            .expect("name");
        let epoch: Option<String> =
            sqlx::query_scalar("SELECT epoch FROM train_formations WHERE id = ?")
                .bind("tf-upd-1")
                .fetch_one(&pool)
                .await
                .expect("epoch");
        let version: i64 = sqlx::query_scalar("SELECT version FROM train_formations WHERE id = ?")
            .bind("tf-upd-1")
            .fetch_one(&pool)
            .await
            .expect("version");

        assert_eq!(name, "Updated Name");
        assert_eq!(epoch, Some("IV".into()));
        assert_eq!(version, 1, "version must be incremented from 0 to 1");
    }

    /// Deleting a formation must cascade-delete all its elements.
    #[sqlx::test(migrations = "./migrations")]
    async fn test_delete_formation_cascades_elements(pool: SqlitePool) {
        insert_test_companies(&pool).await;
        insert_locomotive(&pool, "proto-del-loco", "trn:railway-company:fs", "E.444").await;

        let mut uow = SqliteUnitOfWork::new(&pool).await.expect("create uow");
        {
            let mut repo = SqlxTrainFormationRepository::new(&mut uow.tx);

            let formation = TrainFormation::create("tf-del-1".into(), "Formation To Delete".into())
                .expect("create");
            repo.save(&formation).await.expect("save formation");

            for (eid, pos) in [("el-del-1", 0i32), ("el-del-2", 1i32)] {
                let el = FormationElement {
                    id: eid.to_string(),
                    prototype_id: "proto-del-loco".into(),
                    owned_rolling_stock_id: None,
                    position_order: pos,
                    traction_override: 0,
                };
                repo.add_element("tf-del-1", &el)
                    .await
                    .expect("add element");
            }

            repo.delete("tf-del-1").await.expect("delete formation");
        }
        uow.commit().await.expect("commit");

        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM formation_elements WHERE formation_id = ?")
                .bind("tf-del-1")
                .fetch_one(&pool)
                .await
                .expect("count elements");
        assert_eq!(
            count, 0,
            "elements must be cascade-deleted with their formation"
        );
    }

    /// A formation summary must report `has_traction = true` when it has a locomotive element.
    #[sqlx::test(migrations = "./migrations")]
    async fn test_list_formations_returns_traction_flag(pool: SqlitePool) {
        insert_test_companies(&pool).await;
        insert_locomotive(&pool, "proto-trac-loco", "trn:railway-company:fs", "E.646").await;

        let mut uow = SqliteUnitOfWork::new(&pool).await.expect("create uow");
        let mut repo = SqlxTrainFormationRepository::new(&mut uow.tx);

        let formation = TrainFormation::create("tf-trac-1".into(), "Traction Formation".into())
            .expect("create");
        repo.save(&formation).await.expect("save");

        let loco = FormationElement {
            id: "el-trac-loco".into(),
            prototype_id: "proto-trac-loco".into(),
            owned_rolling_stock_id: None,
            position_order: 0,
            traction_override: 0,
        };
        repo.add_element("tf-trac-1", &loco)
            .await
            .expect("add loco");

        let summaries = repo.list_summaries().await.expect("list summaries");
        let summary = summaries
            .iter()
            .find(|s| s.id == "tf-trac-1")
            .expect("formation must appear in summary list");

        assert!(
            summary.has_traction,
            "formation with a locomotive must have has_traction=true"
        );
    }

    // ── prototype_repo tests ──────────────────────────────────────────────────

    /// Running the prototype seed twice must not change the row count (`INSERT OR IGNORE`).
    #[sqlx::test(migrations = "./migrations")]
    async fn test_seed_prototypes_idempotent(pool: SqlitePool) {
        insert_test_companies(&pool).await;

        crate::core::infrastructure::seeder::seed_prototypes(&pool)
            .await
            .expect("first seed");
        let count_first: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM prototypes")
            .fetch_one(&pool)
            .await
            .expect("count after first seed");

        crate::core::infrastructure::seeder::seed_prototypes(&pool)
            .await
            .expect("second seed");
        let count_second: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM prototypes")
            .fetch_one(&pool)
            .await
            .expect("count after second seed");

        assert!(count_first > 0, "seed must insert at least one prototype");
        assert_eq!(
            count_first, count_second,
            "second seed must not change row count (INSERT OR IGNORE)"
        );
    }

    /// Saving a custom prototype must return `is_custom = true`.
    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_custom_prototype(pool: SqlitePool) {
        insert_test_companies(&pool).await;

        let mut uow = SqliteUnitOfWork::new(&pool).await.expect("create uow");
        let mut repo = SqlxTrainFormationRepository::new(&mut uow.tx);

        let view = repo
            .save_prototype(SavePrototypeParams {
                id: "proto-custom-1",
                railway_company_id: "trn:railway-company:fs",
                series_code: "E.656 Custom",
                car_type: "Locomotive",
                service_level: None,
                category: "Locomotive",
                is_motorized: true,
                default_is_dummy: false,
                notes: None,
            })
            .await
            .expect("save prototype");

        assert!(
            view.is_custom,
            "is_custom must be true for custom prototypes"
        );
        assert_eq!(view.series_code, "E.656 Custom");
        assert_eq!(view.railway_company_id, "trn:railway-company:fs");
    }

    /// Searching with a query must return only matching prototypes.
    #[sqlx::test(migrations = "./migrations")]
    async fn test_search_prototypes_filters_by_query(pool: SqlitePool) {
        insert_test_companies(&pool).await;
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "INSERT INTO prototypes
             (id, railway_company_id, series_code, car_type, service_level,
              category, is_motorized, default_is_dummy, is_custom, created_at, updated_at, version)
             VALUES
               ('proto-gc', 'trn:railway-company:fs', 'UIC-Z1 Gran Comfort', 'Coach',
                '1st Class', 'Passenger', 0, 0, 0, ?, ?, 0),
               ('proto-loco-x', 'trn:railway-company:fs', 'E.444 Tartaruga', 'Locomotive',
                NULL, 'Locomotive', 1, 0, 0, ?, ?, 0)",
        )
        .bind(&now)
        .bind(&now)
        .bind(&now)
        .bind(&now)
        .execute(&pool)
        .await
        .expect("insert test prototypes");

        let mut uow = SqliteUnitOfWork::new(&pool).await.expect("create uow");
        let mut repo = SqlxTrainFormationRepository::new(&mut uow.tx);

        let results = repo
            .search_prototypes(Some("Gran Comfort"))
            .await
            .expect("search");

        let series: Vec<&str> = results
            .iter()
            .flat_map(|g| g.prototypes.iter().map(|p| p.series_code.as_str()))
            .collect();

        assert!(
            series.contains(&"UIC-Z1 Gran Comfort"),
            "search must match 'Gran Comfort' prototype"
        );
        assert!(
            !series.contains(&"E.444 Tartaruga"),
            "search must not return unrelated prototype"
        );
    }

    /// Search results must be grouped by railway company.
    #[sqlx::test(migrations = "./migrations")]
    async fn test_search_prototypes_grouped_by_company(pool: SqlitePool) {
        insert_test_companies(&pool).await;
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "INSERT INTO prototypes
             (id, railway_company_id, series_code, car_type, service_level,
              category, is_motorized, default_is_dummy, is_custom, created_at, updated_at, version)
             VALUES
               ('proto-grp-fs-1', 'trn:railway-company:fs', 'E.444', 'Locomotive',
                NULL, 'Locomotive', 1, 0, 0, ?, ?, 0),
               ('proto-grp-fs-2', 'trn:railway-company:fs', 'E.646', 'Locomotive',
                NULL, 'Locomotive', 1, 0, 0, ?, ?, 0),
               ('proto-grp-sbb-1', 'trn:railway-company:sbb-cff-ffs', 'Re 4/4 II', 'Locomotive',
                NULL, 'Locomotive', 1, 0, 0, ?, ?, 0)",
        )
        .bind(&now)
        .bind(&now)
        .bind(&now)
        .bind(&now)
        .bind(&now)
        .bind(&now)
        .execute(&pool)
        .await
        .expect("insert grouped prototypes");

        let mut uow = SqliteUnitOfWork::new(&pool).await.expect("create uow");
        let mut repo = SqlxTrainFormationRepository::new(&mut uow.tx);

        let groups = repo.search_prototypes(None).await.expect("search all");

        let fs = groups
            .iter()
            .find(|g| g.railway_company_id == "trn:railway-company:fs")
            .expect("FS group must exist");
        assert_eq!(fs.prototypes.len(), 2, "FS must have 2 prototypes");

        let sbb = groups
            .iter()
            .find(|g| g.railway_company_id == "trn:railway-company:sbb-cff-ffs")
            .expect("SBB group must exist");
        assert_eq!(sbb.prototypes.len(), 1, "SBB must have 1 prototype");
    }

    /// Attempting to delete a prototype referenced by a formation element must fail
    /// with a FOREIGN KEY constraint error (`ON DELETE RESTRICT`).
    #[sqlx::test(migrations = "./migrations")]
    async fn test_prototype_delete_restricted_when_in_use(pool: SqlitePool) {
        insert_test_companies(&pool).await;
        insert_locomotive(
            &pool,
            "proto-restricted",
            "trn:railway-company:fs",
            "E.444 Restricted",
        )
        .await;

        let mut uow = SqliteUnitOfWork::new(&pool).await.expect("create uow");
        {
            let mut repo = SqlxTrainFormationRepository::new(&mut uow.tx);

            let formation =
                TrainFormation::create("tf-restrict-1".into(), "Formation With Prototype".into())
                    .expect("create");
            repo.save(&formation).await.expect("save");

            let element = FormationElement {
                id: "el-restrict-1".into(),
                prototype_id: "proto-restricted".into(),
                owned_rolling_stock_id: None,
                position_order: 0,
                traction_override: 0,
            };
            repo.add_element("tf-restrict-1", &element)
                .await
                .expect("add element");
        }
        uow.commit().await.expect("commit");

        // Direct SQL delete must fail — FK ON DELETE RESTRICT blocks it
        let result = sqlx::query("DELETE FROM prototypes WHERE id = 'proto-restricted'")
            .execute(&pool)
            .await;

        assert!(
            result.is_err(),
            "deleting a prototype in use must be blocked by FK RESTRICT constraint"
        );
    }
}
