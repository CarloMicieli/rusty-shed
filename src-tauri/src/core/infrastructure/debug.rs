use crate::core::infrastructure::error::CommandError;
use serde::Serialize;
use sqlx::{Row, SqlitePool};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

const INTERNAL_TABLES: &[&str] = &["_sqlx_migrations"];

/// Read model for a single SQLite table's high-level statistics.
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseTableStat {
    /// SQLite table name.
    pub table_name: String,
    /// Current row count.
    pub row_count: i64,
    /// Best-effort approximate size in bytes, if available.
    pub estimated_bytes: Option<i64>,
}

/// Load row counts and best-effort size estimates for application tables.
///
/// Row counts are queried dynamically from `sqlite_master` discovered tables.
/// Size estimates are read from `dbstat` when the SQLite build exposes it.
/// If `dbstat` is unavailable, `estimated_bytes` is set to `None` instead of
/// failing the whole request.
pub async fn load_database_table_stats(
    pool: &SqlitePool,
) -> Result<Vec<DatabaseTableStat>, CommandError> {
    let table_names = load_user_table_names(pool).await?;
    let size_map = load_table_size_map(pool).await?;

    let mut stats = Vec::with_capacity(table_names.len());
    for table_name in table_names {
        let count_query = format!(
            "SELECT COUNT(*) AS row_count FROM \"{}\"",
            escape_sqlite_identifier(&table_name)
        );
        let row_count = sqlx::query_scalar::<_, i64>(&count_query)
            .fetch_one(pool)
            .await
            .map_err(|error| {
                CommandError::DatabaseError(format!(
                    "failed to count rows for table '{table_name}': {error}"
                ))
            })?;

        stats.push(DatabaseTableStat {
            estimated_bytes: size_map.get(&table_name).copied(),
            row_count,
            table_name,
        });
    }

    Ok(stats)
}

/// Tail the newest persisted application log file and return the latest lines.
pub async fn tail_recent_logs(log_dir: &Path, limit: usize) -> Result<Vec<String>, CommandError> {
    let Some(log_file) = newest_log_file(log_dir)? else {
        return Ok(Vec::new());
    };

    let bytes = match tokio::fs::read(&log_file).await {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => {
            return Err(CommandError::unknown(format!(
                "failed to read log file '{}': {error}",
                log_file.display()
            )));
        }
    };

    let content = String::from_utf8_lossy(&bytes);
    Ok(tail_lines(&content, limit))
}

async fn load_user_table_names(pool: &SqlitePool) -> Result<Vec<String>, CommandError> {
    let mut table_names = sqlx::query_scalar::<_, String>(
        "SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%' ORDER BY name",
    )
    .fetch_all(pool)
    .await
    .map_err(|error| CommandError::DatabaseError(format!("failed to enumerate SQLite tables: {error}")))?;

    table_names.retain(|name| !INTERNAL_TABLES.contains(&name.as_str()));
    Ok(table_names)
}

async fn load_table_size_map(pool: &SqlitePool) -> Result<HashMap<String, i64>, CommandError> {
    let rows = match sqlx::query("SELECT name, SUM(pgsize) AS size_bytes FROM dbstat GROUP BY name")
        .fetch_all(pool)
        .await
    {
        Ok(rows) => rows,
        Err(sqlx::Error::Database(error)) if error.message().contains("no such table: dbstat") => {
            return Ok(HashMap::new());
        }
        Err(error) => {
            return Err(CommandError::DatabaseError(format!(
                "failed to query SQLite dbstat virtual table: {error}"
            )));
        }
    };

    let mut sizes = HashMap::with_capacity(rows.len());
    for row in rows {
        let name = row.try_get::<String, _>("name").map_err(|error| {
            CommandError::DatabaseError(format!("failed to decode dbstat table name: {error}"))
        })?;
        let size_bytes = row
            .try_get::<Option<i64>, _>("size_bytes")
            .map_err(|error| {
                CommandError::DatabaseError(format!(
                    "failed to decode dbstat size for '{name}': {error}"
                ))
            })?;

        if let Some(size_bytes) = size_bytes {
            sizes.insert(name, size_bytes);
        }
    }

    Ok(sizes)
}

fn newest_log_file(log_dir: &Path) -> Result<Option<PathBuf>, CommandError> {
    let entries = match std::fs::read_dir(log_dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(CommandError::unknown(format!(
                "failed to read log directory '{}': {error}",
                log_dir.display()
            )));
        }
    };

    let mut newest: Option<(SystemTime, PathBuf)> = None;
    for entry in entries.flatten() {
        let path = entry.path();
        let metadata = match entry.metadata() {
            Ok(metadata) if metadata.is_file() => metadata,
            _ => continue,
        };

        let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        let should_replace = newest
            .as_ref()
            .map(|(current, _)| modified > *current)
            .unwrap_or(true);

        if should_replace {
            newest = Some((modified, path));
        }
    }

    Ok(newest.map(|(_, path)| path))
}

fn tail_lines(content: &str, limit: usize) -> Vec<String> {
    if limit == 0 {
        return Vec::new();
    }

    let lines: Vec<String> = content.lines().map(ToOwned::to_owned).collect();
    let skip = lines.len().saturating_sub(limit);
    lines.into_iter().skip(skip).collect()
}

fn escape_sqlite_identifier(identifier: &str) -> String {
    identifier.replace('"', "\"\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn load_database_table_stats_returns_counts_for_user_tables() {
        let pool = SqlitePool::connect(":memory:")
            .await
            .expect("in-memory sqlite pool");
        sqlx::query("CREATE TABLE alpha (id INTEGER PRIMARY KEY, name TEXT NOT NULL)")
            .execute(&pool)
            .await
            .expect("create alpha");
        sqlx::query("CREATE TABLE beta (id INTEGER PRIMARY KEY)")
            .execute(&pool)
            .await
            .expect("create beta");
        sqlx::query("INSERT INTO alpha (name) VALUES ('one'), ('two')")
            .execute(&pool)
            .await
            .expect("seed alpha");
        sqlx::query("INSERT INTO beta DEFAULT VALUES")
            .execute(&pool)
            .await
            .expect("seed beta");

        let stats = load_database_table_stats(&pool).await.expect("load stats");

        assert!(
            stats
                .iter()
                .any(|stat| stat.table_name == "alpha" && stat.row_count == 2)
        );
        assert!(
            stats
                .iter()
                .any(|stat| stat.table_name == "beta" && stat.row_count == 1)
        );
    }

    #[test]
    fn tail_lines_returns_latest_requested_lines() {
        let content = "first\nsecond\nthird\nfourth\n";

        let lines = tail_lines(content, 2);

        assert_eq!(lines, vec!["third".to_string(), "fourth".to_string()]);
    }
}
