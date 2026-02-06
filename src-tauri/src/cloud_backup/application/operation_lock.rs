/// Operation locking to prevent concurrent backup/restore from multiple invocations
///
/// This module provides a simple lock mechanism to ensure that only one
/// backup or restore operation can run at a time within the same process.
/// This is an MVP implementation - in production, this could be extended to
/// support cross-device synchronization using the database.
use crate::cloud_backup::domain::{CloudBackupError, Result};
use once_cell::sync::Lazy;
use std::sync::Mutex;

/// Represents a lock on backup operations
/// Protects against concurrent sync/restore that would corrupt the database
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationType {
    /// Backup/sync operation
    Backup,
    /// Restore operation
    Restore,
}

/// Global operation lock (MVP - protects against concurrent operations in same process)
///
/// In a production system, this would be backed by a database table to support
/// cross-device locking. Currently, it only prevents concurrent operations
/// within the same application instance.
static OPERATION_LOCK: Lazy<Mutex<Option<OperationType>>> = Lazy::new(|| Mutex::new(None));

/// Attempt to acquire operation lock
///
/// # Arguments
/// * `operation` - Type of operation to lock (Backup or Restore)
///
/// # Returns
/// * `Ok(OperationLockGuard)` - Lock acquired successfully
/// * `Err(CloudBackupError)` - Another operation is already running
pub fn try_acquire_lock(operation: OperationType) -> Result<OperationLockGuard> {
    let mut lock = OPERATION_LOCK
        .lock()
        .map_err(|_| CloudBackupError::UnexpectedError("Failed to acquire lock".to_string()))?;

    if lock.is_some() {
        return Err(CloudBackupError::UnexpectedError(
            "Another backup or restore operation is already in progress. Please wait for it to complete.".to_string()
        ));
    }

    *lock = Some(operation);
    Ok(OperationLockGuard { operation })
}

/// RAII guard that releases the lock when dropped
///
/// Automatically releases the operation lock when this guard is dropped,
/// allowing the next operation to proceed. This ensures locks are always
/// released, even if the operation panics.
#[allow(dead_code)]
pub struct OperationLockGuard {
    operation: OperationType,
}

impl Drop for OperationLockGuard {
    fn drop(&mut self) {
        if let Ok(mut lock) = OPERATION_LOCK.lock() {
            *lock = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acquire_lock_succeeds() {
        let guard = try_acquire_lock(OperationType::Backup);
        assert!(guard.is_ok());
    }

    #[test]
    fn test_lock_prevents_concurrent_operations() {
        let _guard1 = try_acquire_lock(OperationType::Backup).unwrap();
        let result = try_acquire_lock(OperationType::Restore);
        assert!(result.is_err());
    }

    #[test]
    fn test_lock_released_on_drop() {
        {
            let _guard = try_acquire_lock(OperationType::Backup).unwrap();
            // Lock is held here
        }
        // Guard dropped, lock should be released
        let result = try_acquire_lock(OperationType::Backup);
        assert!(result.is_ok());
    }
}
