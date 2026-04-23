use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents the user's Google account connection state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleConnection {
    /// Connected Google email address
    pub email: String,

    /// When the connection was established
    pub connected_at: DateTime<Utc>,

    /// Connection status
    pub status: ConnectionStatus,
}

impl GoogleConnection {
    /// Create a new Google connection
    pub fn new(email: String) -> Self {
        Self {
            email,
            connected_at: Utc::now(),
            status: ConnectionStatus::Connected,
        }
    }

    /// Check if the connection is active
    pub fn is_connected(&self) -> bool {
        matches!(self.status, ConnectionStatus::Connected)
    }

    /// Mark connection as expired
    pub fn mark_expired(&mut self) {
        self.status = ConnectionStatus::TokenExpired;
    }

    /// Mark connection as disconnected
    pub fn disconnect(&mut self) {
        self.status = ConnectionStatus::Disconnected;
    }
}

/// Connection status enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ConnectionStatus {
    /// Successfully connected
    Connected,

    /// Disconnected by user
    Disconnected,

    /// OAuth token has expired
    TokenExpired,

    /// Connection error occurred
    Error { message: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_connection_is_connected() {
        let conn = GoogleConnection::new("user@example.com".to_string());
        assert!(conn.is_connected());
        assert_eq!(conn.email, "user@example.com");
    }

    #[test]
    fn test_mark_expired() {
        let mut conn = GoogleConnection::new("user@example.com".to_string());
        conn.mark_expired();
        assert!(!conn.is_connected());
        assert_eq!(conn.status, ConnectionStatus::TokenExpired);
    }

    #[test]
    fn test_disconnect() {
        let mut conn = GoogleConnection::new("user@example.com".to_string());
        conn.disconnect();
        assert!(!conn.is_connected());
        assert_eq!(conn.status, ConnectionStatus::Disconnected);
    }
}
