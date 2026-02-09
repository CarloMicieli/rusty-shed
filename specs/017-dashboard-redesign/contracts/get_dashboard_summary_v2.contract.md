# API Contract: get_dashboard_summary (Extended)

**Command**: `get_dashboard_summary`  
**Location**: `src-tauri/src/dashboard/application/get_dashboard_summary.rs`  
**Type**: Tauri Command (IPC)  
**Purpose**: Fetch dashboard summary with purchase-grouped recent acquisitions

## Command Signature

```rust
#[tauri::command]
#[specta::specta]
pub async fn get_dashboard_summary(
    state: tauri::State<'_, AppState>,
) -> Result<DashboardSummary, CommandError>
```

**Parameters**: None (uses global app state)

**Returns**: `Result<DashboardSummary, CommandError>`

---

## Request

### Frontend Invocation

```typescript
import { commands } from '$lib/bindings';

// Usage
const result = await commands.getDashboardSummary();

if (result.status === 'ok') {
  const summary: DashboardSummary = result.data;
  // Use purchase_groups, depot_items, totals
} else {
  console.error('Failed to load dashboard:', result.error);
}
```

**No Parameters Required**: Command uses authenticated user from app state.

---

## Response

### Success Response

```json
{
  "status": "ok",
  "data": {
    "totals": {
      "collectionItems": 42,
      "wishlists": 3,
      "maintenanceDue": 2,
      "totalValue": {
        "amount": 125000,
        "currency": "EUR"
      }
    },
    "purchaseGroups": [
      {
        "id": "purchase-2026-02-01-seller:123",
        "purchaseDate": "2026-02-01",
        "sellerName": "Milan Model Trains",
        "notes": "Birthday gift from Maria",
        "modelCards": [
          {
            "id": "trn:railway-model:roco:62150",
            "thumbnailPath": "/images/trn-railway-model-roco-62150.jpg",
            "manufacturer": "Roco",
            "productCode": "62150",
            "condition": "NEW",
            "description": "Electric locomotive BR 193 Vectron"
          },
          {
            "id": "trn:railway-model:fleischmann:738905",
            "thumbnailPath": null,
            "manufacturer": "Fleischmann",
            "productCode": "738905",
            "condition": "PRE_OWNED",
            "description": "Steam locomotive BR 03.10"
          }
        ],
        "totalCount": 2
      },
      {
        "id": "purchase-2026-01-15-seller:456",
        "purchaseDate": "2026-01-15",
        "sellerName": "Hobby Shop Online",
        "notes": null,
        "modelCards": [
          {
            "id": "trn:railway-model:piko:51311",
            "thumbnailPath": "/images/trn-railway-model-piko-51311.jpg",
            "manufacturer": "Piko",
            "productCode": "51311",
            "condition": "NEW",
            "description": "Diesel locomotive BR 218 DB"
          },
          {
            "id": "trn:railway-model:marklin:39210",
            "thumbnailPath": "/images/trn-railway-model-marklin-39210.jpg",
            "manufacturer": "Märklin",
            "productCode": "39210",
            "condition": "NEW",
            "description": "Electric locomotive BR 110.3"
          },
          {
            "id": "trn:railway-model:roco:73779",
            "thumbnailPath": null,
            "manufacturer": "Roco",
            "productCode": "73779",
            "condition": "NEW",
            "description": "Electric railcar ET 25"
          }
        ],
        "totalCount": 5
      }
    ],
    "depotItems": [
      {
        "id": "trn:railway-model:roco:62150",
        "category": "LOCOMOTIVES",
        "manufacturer": "Roco",
        "productCode": "62150",
        "description": "Electric locomotive BR 193 Vectron",
        "condition": "NEW"
      }
    ],
    "recentItems": []
  }
}
```

### Error Response

```json
{
  "status": "error",
  "error": "Database connection failed"
}
```

**Error Types**:

- `DatabaseError`: SQLite query failed
- `InternalError`: Unexpected backend error
- `DataIntegrityError`: Invalid data in database (e.g., orphaned records)

---

## Backend Implementation

### Query Logic (Repository Layer)

```rust
// src-tauri/src/dashboard/infrastructure/dashboard_repository.rs

pub async fn get_dashboard_summary(
    &self,
    pool: &SqlitePool,
) -> Result<DashboardSummary, RepositoryError> {
    // 1. Fetch totals (existing query, unchanged)
    let totals = self.fetch_totals(pool).await?;

    // 2. Fetch purchase groups (NEW)
    let purchase_groups = self.fetch_purchase_groups(pool).await?;

    // 3. Fetch depot items (existing query, unchanged)
    let depot_items = self.fetch_depot_items(pool).await?;

    Ok(DashboardSummary {
        totals,
        purchase_groups,
        depot_items,
        recent_items: vec![], // Deprecated, kept for backward compatibility
    })
}

async fn fetch_purchase_groups(
    &self,
    pool: &SqlitePool,
) -> Result<Vec<PurchaseGroup>, RepositoryError> {
    // Step 1: Group by purchase_date + seller_id, get counts
    let group_rows = sqlx::query_as::<_, PurchaseGroupRow>(
        r#"
        SELECT
            pi.purchase_date,
            pi.seller_id,
            s.name as seller_name,
            ci.notes,
            COUNT(*) as model_count
        FROM purchase_infos pi
        LEFT JOIN sellers s ON pi.seller_id = s.id
        LEFT JOIN collection_items ci ON pi.collection_item_id = ci.id
        WHERE ci.removed_date IS NULL
        GROUP BY pi.purchase_date, pi.seller_id
        ORDER BY pi.purchase_date DESC
        LIMIT 3
        "#
    )
    .fetch_all(pool)
    .await?;

    // Step 2: For each group, fetch up to 3 model cards
    let mut purchase_groups = Vec::new();

    for group_row in group_rows {
        let model_rows = sqlx::query_as::<_, ModelCardRow>(
            r#"
            SELECT
                rm.id as model_id,
                rm.manufacturer_id,
                m.name as manufacturer_name,
                rm.product_code,
                rm.description,
                rm.image_path,
                ci.purchase_condition,
                pi.purchase_date,
                pi.seller_id
            FROM purchase_infos pi
            INNER JOIN collection_items ci ON pi.collection_item_id = ci.id
            INNER JOIN railway_models rm ON ci.railway_model_id = rm.id
            INNER JOIN manufacturers m ON rm.manufacturer_id = m.id
            WHERE pi.purchase_date = $1
              AND (pi.seller_id = $2 OR (pi.seller_id IS NULL AND $2 IS NULL))
              AND ci.removed_date IS NULL
            ORDER BY ci.added_date DESC
            LIMIT 3
            "#
        )
        .bind(&group_row.purchase_date)
        .bind(&group_row.seller_id)
        .fetch_all(pool)
        .await?;

        let purchase_group = PurchaseGroup::try_from((group_row, model_rows))?;
        purchase_groups.push(purchase_group);
    }

    Ok(purchase_groups)
}
```

---

## Data Transformation

### SQL Row → Domain Entity

```rust
// Infrastructure layer entities
#[derive(Debug, Clone, FromRow)]
pub struct PurchaseGroupRow {
    pub purchase_date: String,
    pub seller_id: Option<String>,
    pub seller_name: Option<String>,
    pub notes: Option<String>,
    pub model_count: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct ModelCardRow {
    pub model_id: String,
    pub manufacturer_id: String,
    pub manufacturer_name: String,
    pub product_code: String,
    pub description: String,
    pub image_path: Option<String>,
    pub purchase_condition: Option<String>,
    pub purchase_date: String,
    pub seller_id: Option<String>,
}

// Domain entity conversion
impl TryFrom<(PurchaseGroupRow, Vec<ModelCardRow>)> for PurchaseGroup {
    type Error = DomainError;

    fn try_from(value: (PurchaseGroupRow, Vec<ModelCardRow>)) -> Result<Self, Self::Error> {
        let (group_row, card_rows) = value;

        // Generate stable ID for group
        let id = format!(
            "purchase-{}-{}",
            group_row.purchase_date,
            group_row.seller_id.as_deref().unwrap_or("unknown")
        );

        // Convert model rows (take max 3)
        let model_cards: Vec<ModelCard> = card_rows
            .into_iter()
            .take(3)
            .map(|row| row.try_into())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(PurchaseGroup {
            id,
            purchase_date: group_row.purchase_date,
            seller_name: group_row.seller_name,
            notes: group_row.notes,
            model_cards,
            total_count: group_row.model_count as usize,
        })
    }
}

impl TryFrom<ModelCardRow> for ModelCard {
    type Error = DomainError;

    fn try_from(row: ModelCardRow) -> Result<Self, Self::Error> {
        Ok(ModelCard {
            id: RailwayModelId::from_str(&row.model_id)?,
            thumbnail_path: row.image_path,
            manufacturer: row.manufacturer_name,
            product_code: row.product_code,
            condition: PurchaseCondition::from(row.purchase_condition),
            description: row.description,
        })
    }
}
```

---

## Performance Characteristics

### Query Complexity

**Purchase Groups Query**:

- **Type**: Single aggregation query with GROUP BY
- **Limit**: 3 groups (hard limit for consistent performance)
- **Indexes Used**:
  - `idx_purchase_infos_collection_item` (existing)
  - `purchase_date` (sorted DESC, benefits from index on datetime)

**Model Cards Query** (per group):

- **Type**: JOIN across 4 tables (purchase_infos, collection_items, railway_models, manufacturers)
- **Limit**: 3 models per group (max 9 total queries for edge case)
- **Optimization**: Could be merged into single query with window functions (future enhancement)

### Estimated Timing

- **Totals query**: ~10ms (existing, unchanged)
- **Purchase groups aggregation**: ~20ms (3 groups max)
- **Model cards fetch**: ~15ms per group × 3 = ~45ms
- **Depot items query**: ~30ms (existing, unchanged)
- **Total**: ~105ms (well under 200ms target)

---

## Caching Strategy

**Current**: No caching (data must be fresh for user expectations)

**Future Optimization** (if needed):

- Cache at Tauri state level with 30-second TTL
- Invalidate on model add/update/delete events
- Would reduce queries from 105ms → <5ms for cache hits

---

## Backward Compatibility

### Migration Strategy

**Phase 1** (This Feature):

- Add `purchase_groups` field to `DashboardSummary`
- Keep `recent_items` field (empty array for now)
- Frontend detects `purchase_groups.length > 0` and renders new UI

**Phase 2** (Future):

- Deprecate `recent_items` field with `#[deprecated]` attribute
- Remove from schema in next major version

**Type Safety**: `specta` regenerates TypeScript types automatically, ensuring frontend-backend sync.

---

## Testing Requirements

### Unit Tests (Backend)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_purchase_groups_returns_max_3() {
        let pool = setup_test_db().await;
        seed_4_purchase_groups(&pool).await;

        let repo = DashboardRepository::new();
        let groups = repo.fetch_purchase_groups(&pool).await.unwrap();

        assert_eq!(groups.len(), 3, "Should return max 3 groups");
    }

    #[tokio::test]
    async fn test_purchase_group_limits_model_cards_to_3() {
        let pool = setup_test_db().await;
        seed_purchase_with_5_models(&pool).await;

        let repo = DashboardRepository::new();
        let groups = repo.fetch_purchase_groups(&pool).await.unwrap();

        assert_eq!(groups[0].model_cards.len(), 3);
        assert_eq!(groups[0].total_count, 5);
    }
}
```

### Integration Tests (Frontend)

```typescript
import { describe, it, expect } from 'vitest';
import { commands } from '$lib/bindings';

describe('get_dashboard_summary', () => {
  it('returns purchase groups with model cards', async () => {
    const result = await commands.getDashboardSummary();

    expect(result.status).toBe('ok');
    expect(result.data.purchaseGroups).toBeDefined();
    expect(result.data.purchaseGroups.length).toBeLessThanOrEqual(3);

    const firstGroup = result.data.purchaseGroups[0];
    expect(firstGroup.modelCards.length).toBeLessThanOrEqual(3);
    expect(firstGroup.totalCount).toBeGreaterThanOrEqual(firstGroup.modelCards.length);
  });
});
```

---

## Error Scenarios

| Error                | Cause                               | Response                                                       |
| -------------------- | ----------------------------------- | -------------------------------------------------------------- |
| `DatabaseError`      | SQLite connection failed            | `{ status: "error", error: "Database connection failed" }`     |
| `QueryError`         | Invalid SQL or constraint violation | `{ status: "error", error: "Failed to fetch dashboard data" }` |
| `DataIntegrityError` | Orphaned records (FK violation)     | Log warning, filter out invalid records, return partial data   |
| `InvalidModelId`     | Model ID doesn't match format       | Skip model, log warning, continue                              |

**Error Handling Philosophy**: Graceful degradation - return partial data rather than failing entirely.

---

## Security Considerations

- **Authorization**: Uses authenticated user from `AppState` (no explicit user_id parameter to prevent tampering)
- **SQL Injection**: Parameterized queries via `sqlx` (safe)
- **Data Exposure**: Only returns user's own collection data (enforced by FK relationships)
- **Rate Limiting**: Not required (single user desktop app)

---

## Future Enhancements

1. **Pagination**: If users request "view more purchases", add `offset` parameter
2. **Filtering**: Add `date_range`, `seller_id` filters for custom views
3. **Performance**: Merge group + model queries into single CTE with window functions
4. **Caching**: Add short-lived cache (30s TTL) with event-based invalidation
