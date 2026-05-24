use crate::cloud_backup::domain::{ConnectivityChangedEvent, ConnectivityStatus, Result};
use chrono::Utc;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

/// Check internet connectivity
///
/// Uses the `online` crate to perform actual connectivity check
/// by attempting to reach well-known endpoints.
pub async fn check_connectivity() -> Result<ConnectivityStatus> {
    let is_online = online::tokio::check(None).await.is_ok();

    Ok(ConnectivityStatus {
        is_online,
        checked_at: Utc::now().to_rfc3339(),
    })
}

/// Check if we're online (simple wrapper)
pub async fn is_online() -> bool {
    online::tokio::check(None).await.is_ok()
}

fn has_connectivity_changed(last_status: Option<bool>, next_status: bool) -> bool {
    last_status
        .map(|previous| previous != next_status)
        .unwrap_or(true)
}

fn emit_connectivity_changed(app: &AppHandle, status: &ConnectivityStatus) {
    let payload = ConnectivityChangedEvent {
        is_online: status.is_online,
        checked_at: status.checked_at.clone(),
    };

    if let Err(error) = app.emit("cloud-backup://connectivity-changed", payload) {
        tracing::warn!("cloud backup connectivity event emission failed: {error}");
    }
}

fn handle_connectivity_status(
    app: &AppHandle,
    last_status: &mut Option<bool>,
    status: ConnectivityStatus,
) {
    if has_connectivity_changed(*last_status, status.is_online) {
        *last_status = Some(status.is_online);
        emit_connectivity_changed(app, &status);
    }
}

/// Start a periodic connectivity monitor that emits events on status changes.
pub fn start_connectivity_monitor(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut last_status: Option<bool> = None;
        let mut ticker = tokio::time::interval(Duration::from_secs(30));

        loop {
            ticker.tick().await;

            match check_connectivity().await {
                Ok(status) => handle_connectivity_status(&app, &mut last_status, status),
                Err(error) => {
                    tracing::warn!("cloud backup connectivity check failed: {error}");
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_connectivity() {
        let status = check_connectivity().await.unwrap();
        // We can't assert the value since it depends on actual network
        // but we can verify the structure is correct
        assert!(!status.checked_at.is_empty());
    }

    #[test]
    fn test_has_connectivity_changed() {
        assert!(has_connectivity_changed(None, true));
        assert!(has_connectivity_changed(Some(false), true));
        assert!(!has_connectivity_changed(Some(true), true));
    }
}
