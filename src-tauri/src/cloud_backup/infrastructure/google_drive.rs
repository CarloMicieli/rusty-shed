/// Google Drive API client for backup operations
use crate::cloud_backup::domain::{CloudBackupError, Result};
use reqwest::Client;
use serde_json::json;

/// Extract detailed error information from Google Drive API responses
///
/// Parses API error responses and maps them to user-friendly CloudBackupError variants.
/// Handles specific error codes (401, 403, 404, 429, 5xx) with appropriate messages.
///
/// # Arguments
/// * `status` - HTTP status code
/// * `error_text` - Response body text (may be JSON)
///
/// # Returns
/// `CloudBackupError` variant with descriptive message
///
/// # Error Handling
/// - 403 quotaExceeded → Storage quota error
/// - 401/403 permission → Permission denied
/// - 404 → Resource not found
/// - 429 → Rate limited
/// - 5xx → Service unavailable
fn extract_drive_error(status: u16, error_text: &str) -> CloudBackupError {
    // Try to parse as JSON first
    if let Ok(error_json) = serde_json::from_str::<serde_json::Value>(error_text)
        && let Some(error_obj) = error_json.get("error")
    {
        let error_code = error_obj.get("code").and_then(|c| c.as_u64()).unwrap_or(0);

        let error_msg = error_obj
            .get("message")
            .and_then(|m| m.as_str())
            .unwrap_or("Unknown error");

        // Handle specific error codes
        match error_code {
            403 => {
                // Check for specific 403 reasons
                if error_msg.contains("quotaExceeded") || error_msg.contains("storageQuotaExceeded")
                {
                    return CloudBackupError::DriveError(
                            "Your Google Drive storage quota is full. Please free up space and try again.".to_string()
                        );
                } else if error_msg.contains("Forbidden") || error_msg.contains("permission") {
                    return CloudBackupError::DriveError(
                        "Permission denied. Please reconnect your Google account.".to_string(),
                    );
                } else {
                    return CloudBackupError::DriveError(format!(
                        "Access forbidden: {}",
                        error_msg
                    ));
                }
            }
            401 => {
                return CloudBackupError::TokenExpired;
            }
            404 => {
                return CloudBackupError::DriveError("File or folder not found.".to_string());
            }
            429 => {
                return CloudBackupError::DriveError(
                    "Rate limit exceeded. Please wait a moment and try again.".to_string(),
                );
            }
            500..=599 => {
                return CloudBackupError::DriveError(
                    "Google Drive service is temporarily unavailable. Please try again later."
                        .to_string(),
                );
            }
            _ => {
                return CloudBackupError::DriveError(format!("API Error: {}", error_msg));
            }
        }
    }

    // Handle HTTP status codes directly
    match status {
        401 | 403 => CloudBackupError::TokenExpired,
        404 => CloudBackupError::DriveError("Resource not found.".to_string()),
        429 => CloudBackupError::DriveError("Rate limited. Please try again later.".to_string()),
        500..=599 => CloudBackupError::DriveError(
            "Google Drive service error. Please try again later.".to_string(),
        ),
        _ => CloudBackupError::DriveError(format!("Drive API error ({}): {}", status, error_text)),
    }
}

/// Google Drive client for managing backup files
///
/// Provides methods to interact with Google Drive API for cloud backup operations.
/// All operations use the access token provided at construction.
pub struct GoogleDriveClient {
    access_token: String,
    http_client: Client,
}

impl GoogleDriveClient {
    /// Create new Google Drive client
    ///
    /// # Arguments
    /// * `_client_id` - OAuth client ID (for future use)
    /// * `access_token` - Valid OAuth access token for Google Drive API
    pub fn new(_client_id: String, access_token: String) -> Self {
        Self {
            access_token,
            http_client: Client::new(),
        }
    }

    /// Get or create the backup folder in appDataFolder
    ///
    /// Searches for existing "RustyShed Backups" folder in Google Drive's appDataFolder.
    /// If found, returns its ID. Otherwise, creates a new folder and returns its ID.
    ///
    /// # Returns
    /// * `Ok(String)` - ID of the backup folder
    /// * `Err(CloudBackupError)` - If folder cannot be found/created
    pub async fn get_or_create_backup_folder(&self) -> Result<String> {
        let folder_name = "RustyShed Backups";

        // Query for existing folder
        let query = format!(
            "name='{}' and mimeType='application/vnd.google-apps.folder' and 'appDataFolder' in parents and trashed=false",
            folder_name
        );

        let folder_id = self.find_folder_by_query(&query).await?;

        if let Some(id) = folder_id {
            return Ok(id);
        }

        // Create new folder if not found
        self.create_folder(folder_name, "appDataFolder").await
    }

    /// Find a folder by query
    async fn find_folder_by_query(&self, query: &str) -> Result<Option<String>> {
        let response = self
            .http_client
            .get("https://www.googleapis.com/drive/v3/files")
            .bearer_auth(&self.access_token)
            .query(&[
                ("q", query),
                ("spaces", "appDataFolder"),
                ("fields", "files(id, name)"),
                ("pageSize", "1"),
            ])
            .send()
            .await
            .map_err(|e| CloudBackupError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_text = response.text().await.unwrap_or_default();
            return Err(extract_drive_error(status, &error_text));
        }

        let data: serde_json::Value = response.json().await.map_err(|e| {
            CloudBackupError::DriveError(format!("Failed to parse response: {}", e))
        })?;

        let folder_id = data["files"]
            .get(0)
            .and_then(|f| f["id"].as_str())
            .map(|s| s.to_string());

        Ok(folder_id)
    }

    /// Create a folder in Google Drive
    async fn create_folder(&self, name: &str, parent_id: &str) -> Result<String> {
        let body = json!({
            "name": name,
            "mimeType": "application/vnd.google-apps.folder",
            "parents": [parent_id]
        });

        let response = self
            .http_client
            .post("https://www.googleapis.com/drive/v3/files")
            .bearer_auth(&self.access_token)
            .json(&body)
            .query(&[("fields", "id")])
            .send()
            .await
            .map_err(|e| CloudBackupError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_text = response.text().await.unwrap_or_default();
            return Err(extract_drive_error(status, &error_text));
        }

        let data: serde_json::Value = response.json().await.map_err(|e| {
            CloudBackupError::DriveError(format!("Failed to parse response: {}", e))
        })?;

        data["id"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| CloudBackupError::DriveError("No folder ID in response".to_string()))
    }

    /// Upload a file to Google Drive with appropriate upload method
    ///
    /// Automatically selects between simple and resumable upload based on file size.
    /// Files < 5MB use simple upload; larger files use resumable upload.
    ///
    /// # Arguments
    /// * `folder_id` - ID of parent folder
    /// * `file_name` - Name for the file in Drive
    /// * `file_data` - Raw file contents to upload
    /// * `metadata` - Additional metadata (app properties, etc.)
    ///
    /// # Returns
    /// * `Ok(UploadedFile)` - File ID, name, and size
    /// * `Err(CloudBackupError)` - If upload fails
    pub async fn upload_file(
        &self,
        folder_id: &str,
        file_name: &str,
        file_data: Vec<u8>,
        metadata: serde_json::Value,
    ) -> Result<UploadedFile> {
        // Use simple upload for small files
        if file_data.len() < 5 * 1024 * 1024 {
            self.simple_upload(folder_id, file_name, file_data, metadata)
                .await
        } else {
            self.resumable_upload(folder_id, file_name, file_data, metadata)
                .await
        }
    }

    /// Simple upload for small files
    async fn simple_upload(
        &self,
        folder_id: &str,
        file_name: &str,
        file_data: Vec<u8>,
        metadata: serde_json::Value,
    ) -> Result<UploadedFile> {
        let mut file_metadata = json!({
            "name": file_name,
            "parents": [folder_id]
        });

        // Merge app properties if present
        if let Some(props) = metadata.get("appProperties") {
            file_metadata["appProperties"] = props.clone();
        }

        let client = reqwest::Client::new();
        let response = client
            .post("https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart")
            .bearer_auth(&self.access_token)
            .header("Content-Type", "application/json")
            .json(&file_metadata)
            .body(file_data)
            .send()
            .await
            .map_err(|e| CloudBackupError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_text = response.text().await.unwrap_or_default();
            return Err(extract_drive_error(status, &error_text));
        }

        let data: serde_json::Value = response.json().await.map_err(|e| {
            CloudBackupError::DriveError(format!("Failed to parse response: {}", e))
        })?;

        Ok(UploadedFile {
            id: data["id"].as_str().unwrap_or("").to_string(),
            name: data["name"].as_str().unwrap_or("").to_string(),
            size: data["size"].as_u64().unwrap_or(0),
        })
    }

    /// Resumable upload for large files
    async fn resumable_upload(
        &self,
        folder_id: &str,
        file_name: &str,
        file_data: Vec<u8>,
        metadata: serde_json::Value,
    ) -> Result<UploadedFile> {
        // For now, use simple upload
        // TODO: Implement proper resumable upload with progress tracking
        self.simple_upload(folder_id, file_name, file_data, metadata)
            .await
    }

    /// Download a file from Google Drive
    ///
    /// Retrieves the raw file contents from Google Drive.
    ///
    /// # Arguments
    /// * `file_id` - ID of the file to download
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` - Raw file contents
    /// * `Err(CloudBackupError)` - If download fails
    pub async fn download_file(&self, file_id: &str) -> Result<Vec<u8>> {
        let response = self
            .http_client
            .get(format!(
                "https://www.googleapis.com/drive/v3/files/{}?alt=media",
                file_id
            ))
            .bearer_auth(&self.access_token)
            .send()
            .await
            .map_err(|e| CloudBackupError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_text = response.text().await.unwrap_or_default();
            return Err(extract_drive_error(status, &error_text));
        }

        response
            .bytes()
            .await
            .map(|b| b.to_vec())
            .map_err(|e| CloudBackupError::NetworkError(e.to_string()))
    }

    /// List all backup files in a folder
    ///
    /// Retrieves metadata for all non-trashed files in the specified folder.
    /// Results are sorted by modified time (newest first).
    ///
    /// # Arguments
    /// * `folder_id` - ID of the folder to list
    ///
    /// # Returns
    /// * `Ok(Vec<DriveFile>)` - List of files with metadata
    /// * `Err(CloudBackupError)` - If listing fails
    pub async fn list_files(&self, folder_id: &str) -> Result<Vec<DriveFile>> {
        let query = format!("'{}' in parents and trashed=false", folder_id);

        let response = self
            .http_client
            .get("https://www.googleapis.com/drive/v3/files")
            .bearer_auth(&self.access_token)
            .query(&[
                ("q", &query),
                ("spaces", &"drive".to_string()),
                (
                    "fields",
                    &"files(id, name, size, modifiedTime, appProperties)".to_string(),
                ),
                ("pageSize", &"100".to_string()),
                ("orderBy", &"modifiedTime desc".to_string()),
            ])
            .send()
            .await
            .map_err(|e| CloudBackupError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_text = response.text().await.unwrap_or_default();
            return Err(extract_drive_error(status, &error_text));
        }

        let data: serde_json::Value = response.json().await.map_err(|e| {
            CloudBackupError::DriveError(format!("Failed to parse response: {}", e))
        })?;

        let files = data["files"]
            .as_array()
            .ok_or_else(|| CloudBackupError::DriveError("No files in response".to_string()))?
            .iter()
            .filter_map(|f| {
                Some(DriveFile {
                    id: f["id"].as_str()?.to_string(),
                    name: f["name"].as_str()?.to_string(),
                    size: f["size"].as_u64().unwrap_or(0),
                    modified_time: f["modifiedTime"].as_str().map(|s| s.to_string()),
                    app_properties: f["appProperties"].as_object().cloned(),
                })
            })
            .collect();

        Ok(files)
    }

    /// Delete a file from Google Drive
    ///
    /// Permanently deletes a file from Google Drive. Does not fail if file
    /// doesn't exist (404 is treated as success).
    ///
    /// # Arguments
    /// * `file_id` - ID of file to delete
    ///
    /// # Returns
    /// * `Ok(())` - File deleted successfully
    /// * `Err(CloudBackupError)` - If deletion fails
    pub async fn delete_file(&self, file_id: &str) -> Result<()> {
        let response = self
            .http_client
            .delete(format!(
                "https://www.googleapis.com/drive/v3/files/{}",
                file_id
            ))
            .bearer_auth(&self.access_token)
            .send()
            .await
            .map_err(|e| CloudBackupError::NetworkError(e.to_string()))?;

        if !response.status().is_success() && response.status().as_u16() != 404 {
            let status = response.status().as_u16();
            let error_text = response.text().await.unwrap_or_default();
            return Err(extract_drive_error(status, &error_text));
        }

        Ok(())
    }

    /// Update file metadata (app properties)
    ///
    /// Updates the custom app properties of a file in Google Drive.
    ///
    /// # Arguments
    /// * `file_id` - ID of file to update
    /// * `metadata` - JSON object with appProperties key
    ///
    /// # Returns
    /// * `Ok(())` - Metadata updated successfully
    /// * `Err(CloudBackupError)` - If update fails
    pub async fn update_file_metadata(
        &self,
        file_id: &str,
        metadata: serde_json::Value,
    ) -> Result<()> {
        let body = json!({
            "appProperties": metadata.get("appProperties")
        });

        let response = self
            .http_client
            .patch(format!(
                "https://www.googleapis.com/drive/v3/files/{}",
                file_id
            ))
            .bearer_auth(&self.access_token)
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudBackupError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_text = response.text().await.unwrap_or_default();
            return Err(extract_drive_error(status, &error_text));
        }

        Ok(())
    }
}

/// Represents a file from Google Drive
///
/// Contains metadata about a file retrieved from the Drive API.
#[derive(Debug, Clone)]
pub struct DriveFile {
    /// Unique Google Drive file ID
    pub id: String,
    /// File display name
    pub name: String,
    /// File size in bytes
    pub size: u64,
    /// Last modified time in RFC3339 format
    pub modified_time: Option<String>,
    /// Custom app properties stored with the file
    pub app_properties: Option<serde_json::Map<String, serde_json::Value>>,
}

/// Represents an uploaded file
///
/// Returned after successfully uploading a file to Google Drive.
#[derive(Debug, Clone)]
pub struct UploadedFile {
    /// Unique Google Drive file ID
    pub id: String,
    /// File name in Drive
    pub name: String,
    /// File size in bytes
    pub size: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_drive_client_creation() {
        let client = GoogleDriveClient::new("test_client_id".to_string(), "test_token".to_string());
        assert_eq!(client.access_token, "test_token");
    }
}
