use crate::cloud_backup::domain::{CloudBackupError, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;

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
    if let Some((error_code, error_msg)) = parse_drive_api_error(error_text) {
        return map_drive_api_error(error_code, &error_msg);
    }

    map_http_status_error(status, error_text)
}

fn parse_drive_api_error(error_text: &str) -> Option<(u16, String)> {
    let error_json = serde_json::from_str::<serde_json::Value>(error_text).ok()?;
    let error_obj = error_json.get("error")?;
    let error_code = error_obj
        .get("code")
        .and_then(|c| c.as_u64())
        .map(|v| v as u16)
        .unwrap_or_default();
    let error_msg = error_obj
        .get("message")
        .and_then(|m| m.as_str())
        .unwrap_or("Unknown error")
        .to_string();

    Some((error_code, error_msg))
}

fn map_drive_api_error(error_code: u16, error_msg: &str) -> CloudBackupError {
    match error_code {
        403 => {
            if error_msg.contains("quotaExceeded") || error_msg.contains("storageQuotaExceeded") {
                CloudBackupError::DriveError(
                    "Your Google Drive storage quota is full. Please free up space and try again."
                        .to_string(),
                )
            } else if error_msg.contains("Forbidden") || error_msg.contains("permission") {
                CloudBackupError::DriveError(
                    "Permission denied. Please reconnect your Google account.".to_string(),
                )
            } else {
                CloudBackupError::DriveError(format!("Access forbidden: {}", error_msg))
            }
        }
        401 => CloudBackupError::TokenExpired,
        404 => CloudBackupError::DriveError("File or folder not found.".to_string()),
        429 => CloudBackupError::DriveError(
            "Rate limit exceeded. Please wait a moment and try again.".to_string(),
        ),
        500..=599 => CloudBackupError::DriveError(
            "Google Drive service is temporarily unavailable. Please try again later.".to_string(),
        ),
        _ => CloudBackupError::DriveError(format!("API Error: {}", error_msg)),
    }
}

fn map_http_status_error(status: u16, error_text: &str) -> CloudBackupError {
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
        let folder_name = "RustyShedBackups";

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
            let error_text = response
                .text()
                .await
                .map_err(|e| CloudBackupError::NetworkError(e.to_string()))?;
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
            let error_text = response
                .text()
                .await
                .map_err(|e| CloudBackupError::NetworkError(e.to_string()))?;
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

    /// Simple multipart upload for files under 5 MB
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

        // Build a multipart/related body as required by the Drive upload API
        let boundary_uuid = uuid::Uuid::new_v4().simple().to_string();
        let boundary = format!("boundary-{}", boundary_uuid);
        let metadata_json = serde_json::to_string(&file_metadata).map_err(|e| {
            CloudBackupError::DriveError(format!("Failed to serialize file metadata: {}", e))
        })?;

        let mut body: Vec<u8> = Vec::new();
        body.extend_from_slice(
            format!("--{boundary}\r\nContent-Type: application/json; charset=UTF-8\r\n\r\n")
                .as_bytes(),
        );
        body.extend_from_slice(metadata_json.as_bytes());
        body.extend_from_slice(b"\r\n");
        body.extend_from_slice(
            format!("--{boundary}\r\nContent-Type: application/octet-stream\r\n\r\n").as_bytes(),
        );
        body.extend_from_slice(&file_data);
        body.extend_from_slice(b"\r\n");
        body.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());

        let response = self
            .http_client
            .post("https://www.googleapis.com/upload/drive/v3/files")
            .bearer_auth(&self.access_token)
            .header(
                "Content-Type",
                format!("multipart/related; boundary={}", boundary),
            )
            .query(&[("uploadType", "multipart"), ("fields", "id,name,size")])
            .body(body)
            .send()
            .await
            .map_err(|e| CloudBackupError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_text = response
                .text()
                .await
                .map_err(|e| CloudBackupError::NetworkError(e.to_string()))?;
            return Err(extract_drive_error(status, &error_text));
        }

        let data: serde_json::Value = response.json().await.map_err(|e| {
            CloudBackupError::DriveError(format!("Failed to parse response: {}", e))
        })?;

        let id = data["id"]
            .as_str()
            .ok_or_else(|| {
                CloudBackupError::DriveError("No file ID in upload response".to_string())
            })?
            .to_string();

        let name = data["name"]
            .as_str()
            .ok_or_else(|| {
                CloudBackupError::DriveError("No file name in upload response".to_string())
            })?
            .to_string();

        Ok(UploadedFile {
            id,
            name,
            size: data["size"].as_u64().unwrap_or(0),
        })
    }

    /// Resumable upload for files ≥ 5 MB
    ///
    /// Implements the Google Drive resumable upload protocol:
    /// 1. Initiate the upload session and obtain an upload URL.
    /// 2. Upload the full file content to that URL in a single PUT request.
    ///
    /// This approach is required by the Drive API for large files and avoids
    /// multipart size limits.
    async fn resumable_upload(
        &self,
        folder_id: &str,
        file_name: &str,
        file_data: Vec<u8>,
        metadata: serde_json::Value,
    ) -> Result<UploadedFile> {
        let transport = ReqwestResumableUploadTransport {
            http_client: &self.http_client,
            access_token: &self.access_token,
        };

        self.resumable_upload_with_transport(folder_id, file_name, file_data, metadata, &transport)
            .await
    }

    async fn resumable_upload_with_transport<T: ResumableUploadTransport + Sync>(
        &self,
        folder_id: &str,
        file_name: &str,
        file_data: Vec<u8>,
        metadata: serde_json::Value,
        transport: &T,
    ) -> Result<UploadedFile> {
        let file_metadata = build_resumable_file_metadata(folder_id, file_name, &metadata);
        let file_size = file_data.len();

        let init_response = transport
            .initiate_resumable_upload(file_size, file_metadata)
            .await?;
        if !is_success_status(init_response.status) {
            return Err(extract_drive_error(
                init_response.status,
                &init_response.body,
            ));
        }

        let upload_url = extract_upload_url(&init_response.headers)?;

        let upload_response = transport
            .upload_resumable_content(&upload_url, file_size, file_data)
            .await?;
        if !is_success_status(upload_response.status) {
            return Err(extract_drive_error(
                upload_response.status,
                &upload_response.body,
            ));
        }

        parse_uploaded_file_response(&upload_response.body, true)
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
            let error_text = response
                .text()
                .await
                .map_err(|e| CloudBackupError::NetworkError(e.to_string()))?;
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
            let error_text = response
                .text()
                .await
                .map_err(|e| CloudBackupError::NetworkError(e.to_string()))?;
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
            let error_text = response
                .text()
                .await
                .map_err(|e| CloudBackupError::NetworkError(e.to_string()))?;
            return Err(extract_drive_error(status, &error_text));
        }

        Ok(())
    }

    /// Rename a file in Google Drive
    ///
    /// Updates the display name of a file using the Drive Files.update endpoint.
    /// Used by the stage-and-commit upload pattern to atomically promote a
    /// temporary upload to its final name.
    ///
    /// # Arguments
    /// * `file_id` - ID of file to rename
    /// * `new_name` - New display name for the file
    ///
    /// # Returns
    /// * `Ok(())` - File renamed successfully
    /// * `Err(CloudBackupError)` - If rename fails
    pub async fn rename_file(&self, file_id: &str, new_name: &str) -> Result<()> {
        let body = json!({ "name": new_name });

        let response = self
            .http_client
            .patch(format!(
                "https://www.googleapis.com/drive/v3/files/{}",
                file_id
            ))
            .bearer_auth(&self.access_token)
            .query(&[("fields", "id,name")])
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudBackupError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_text = response
                .text()
                .await
                .map_err(|e| CloudBackupError::NetworkError(e.to_string()))?;
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
            let error_text = response
                .text()
                .await
                .map_err(|e| CloudBackupError::NetworkError(e.to_string()))?;
            return Err(extract_drive_error(status, &error_text));
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct ResumableInitResponse {
    status: u16,
    headers: HashMap<String, String>,
    body: String,
}

#[derive(Debug, Clone)]
struct ResumableUploadResponse {
    status: u16,
    body: String,
}

#[async_trait]
trait ResumableUploadTransport {
    async fn initiate_resumable_upload(
        &self,
        file_size: usize,
        file_metadata: serde_json::Value,
    ) -> Result<ResumableInitResponse>;

    async fn upload_resumable_content(
        &self,
        upload_url: &str,
        file_size: usize,
        file_data: Vec<u8>,
    ) -> Result<ResumableUploadResponse>;
}

struct ReqwestResumableUploadTransport<'a> {
    http_client: &'a Client,
    access_token: &'a str,
}

#[async_trait]
impl ResumableUploadTransport for ReqwestResumableUploadTransport<'_> {
    async fn initiate_resumable_upload(
        &self,
        file_size: usize,
        file_metadata: serde_json::Value,
    ) -> Result<ResumableInitResponse> {
        let response = self
            .http_client
            .post("https://www.googleapis.com/upload/drive/v3/files")
            .bearer_auth(self.access_token)
            .header("X-Upload-Content-Type", "application/octet-stream")
            .header("X-Upload-Content-Length", file_size.to_string())
            .query(&[("uploadType", "resumable"), ("fields", "id,name,size")])
            .json(&file_metadata)
            .send()
            .await
            .map_err(|e| CloudBackupError::NetworkError(e.to_string()))?;

        let status = response.status().as_u16();
        let headers = response
            .headers()
            .iter()
            .filter_map(|(name, value)| {
                value
                    .to_str()
                    .ok()
                    .map(|text| (name.as_str().to_ascii_lowercase(), text.to_string()))
            })
            .collect::<HashMap<_, _>>();
        let body = response
            .text()
            .await
            .map_err(|e| CloudBackupError::NetworkError(e.to_string()))?;

        Ok(ResumableInitResponse {
            status,
            headers,
            body,
        })
    }

    async fn upload_resumable_content(
        &self,
        upload_url: &str,
        file_size: usize,
        file_data: Vec<u8>,
    ) -> Result<ResumableUploadResponse> {
        let response = self
            .http_client
            .put(upload_url)
            .header("Content-Type", "application/octet-stream")
            .header("Content-Length", file_size.to_string())
            .header("Content-Range", build_content_range(file_size))
            .body(file_data)
            .send()
            .await
            .map_err(|e| CloudBackupError::NetworkError(e.to_string()))?;

        let status = response.status().as_u16();
        let body = response
            .text()
            .await
            .map_err(|e| CloudBackupError::NetworkError(e.to_string()))?;

        Ok(ResumableUploadResponse { status, body })
    }
}

fn build_resumable_file_metadata(
    folder_id: &str,
    file_name: &str,
    metadata: &serde_json::Value,
) -> serde_json::Value {
    let mut file_metadata = json!({
        "name": file_name,
        "parents": [folder_id]
    });

    if let Some(props) = metadata.get("appProperties") {
        file_metadata["appProperties"] = props.clone();
    }

    file_metadata
}

fn build_content_range(file_size: usize) -> String {
    let end = file_size.saturating_sub(1);
    format!("bytes 0-{end}/{file_size}")
}

fn is_success_status(status: u16) -> bool {
    (200..300).contains(&status)
}

fn extract_upload_url(headers: &HashMap<String, String>) -> Result<String> {
    headers.get("location").cloned().ok_or_else(|| {
        CloudBackupError::DriveError(
            "No upload URL in resumable upload initiation response".to_string(),
        )
    })
}

fn parse_uploaded_file_response(body: &str, is_resumable: bool) -> Result<UploadedFile> {
    let data: serde_json::Value = serde_json::from_str(body).map_err(|e| {
        let context = if is_resumable {
            "Failed to parse upload response"
        } else {
            "Failed to parse response"
        };
        CloudBackupError::DriveError(format!("{context}: {e}"))
    })?;

    let id = data["id"]
        .as_str()
        .ok_or_else(|| {
            if is_resumable {
                CloudBackupError::DriveError("No file ID in resumable upload response".to_string())
            } else {
                CloudBackupError::DriveError("No file ID in upload response".to_string())
            }
        })?
        .to_string();

    let name = data["name"]
        .as_str()
        .ok_or_else(|| {
            if is_resumable {
                CloudBackupError::DriveError(
                    "No file name in resumable upload response".to_string(),
                )
            } else {
                CloudBackupError::DriveError("No file name in upload response".to_string())
            }
        })?
        .to_string();

    Ok(UploadedFile {
        id,
        name,
        size: data["size"].as_u64().unwrap_or(0),
    })
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
    use std::sync::{Arc, Mutex};

    #[derive(Debug, Clone)]
    struct FakeTransportState {
        init_request_size: Option<usize>,
        init_request_metadata: Option<serde_json::Value>,
        upload_url: Option<String>,
        upload_size: Option<usize>,
        upload_data_len: Option<usize>,
    }

    impl FakeTransportState {
        fn new() -> Self {
            Self {
                init_request_size: None,
                init_request_metadata: None,
                upload_url: None,
                upload_size: None,
                upload_data_len: None,
            }
        }
    }

    struct FakeResumableTransport {
        init_response: Result<ResumableInitResponse>,
        upload_response: Result<ResumableUploadResponse>,
        state: Arc<Mutex<FakeTransportState>>,
    }

    impl FakeResumableTransport {
        fn new(
            init_response: Result<ResumableInitResponse>,
            upload_response: Result<ResumableUploadResponse>,
            state: Arc<Mutex<FakeTransportState>>,
        ) -> Self {
            Self {
                init_response,
                upload_response,
                state,
            }
        }
    }

    #[async_trait]
    impl ResumableUploadTransport for FakeResumableTransport {
        async fn initiate_resumable_upload(
            &self,
            file_size: usize,
            file_metadata: serde_json::Value,
        ) -> Result<ResumableInitResponse> {
            let mut lock = self.state.lock().expect("state lock");
            lock.init_request_size = Some(file_size);
            lock.init_request_metadata = Some(file_metadata);
            self.init_response.clone()
        }

        async fn upload_resumable_content(
            &self,
            upload_url: &str,
            file_size: usize,
            file_data: Vec<u8>,
        ) -> Result<ResumableUploadResponse> {
            let mut lock = self.state.lock().expect("state lock");
            lock.upload_url = Some(upload_url.to_string());
            lock.upload_size = Some(file_size);
            lock.upload_data_len = Some(file_data.len());
            self.upload_response.clone()
        }
    }

    #[test]
    fn test_google_drive_client_creation() {
        let client = GoogleDriveClient::new("test_client_id".to_string(), "test_token".to_string());
        assert_eq!(client.access_token, "test_token");
    }

    #[test]
    fn test_extract_drive_error_maps_quota_exceeded() {
        let error = extract_drive_error(
            403,
            r#"{"error":{"code":403,"message":"storageQuotaExceeded"}}"#,
        );

        match error {
            CloudBackupError::DriveError(msg) => assert!(msg.contains("storage quota")),
            other => panic!("expected DriveError, got: {other:?}"),
        }
    }

    #[test]
    fn test_extract_drive_error_maps_permission_denied() {
        let error = extract_drive_error(
            403,
            r#"{"error":{"code":403,"message":"Forbidden: permission denied"}}"#,
        );

        match error {
            CloudBackupError::DriveError(msg) => assert!(msg.contains("Permission denied")),
            other => panic!("expected DriveError, got: {other:?}"),
        }
    }

    #[test]
    fn test_extract_drive_error_maps_token_expired_from_json() {
        let error = extract_drive_error(401, r#"{"error":{"code":401,"message":"Expired"}}"#);
        assert!(matches!(error, CloudBackupError::TokenExpired));
    }

    #[test]
    fn test_extract_drive_error_maps_not_found_from_json() {
        let error = extract_drive_error(404, r#"{"error":{"code":404,"message":"Not Found"}}"#);

        match error {
            CloudBackupError::DriveError(msg) => assert!(msg.contains("File or folder not found")),
            other => panic!("expected DriveError, got: {other:?}"),
        }
    }

    #[test]
    fn test_extract_drive_error_maps_rate_limit_from_json() {
        let error = extract_drive_error(429, r#"{"error":{"code":429,"message":"Too many"}}"#);

        match error {
            CloudBackupError::DriveError(msg) => assert!(msg.contains("Rate limit exceeded")),
            other => panic!("expected DriveError, got: {other:?}"),
        }
    }

    #[test]
    fn test_extract_drive_error_maps_server_error_from_json() {
        let error = extract_drive_error(500, r#"{"error":{"code":500,"message":"Oops"}}"#);

        match error {
            CloudBackupError::DriveError(msg) => {
                assert!(msg.contains("temporarily unavailable"))
            }
            other => panic!("expected DriveError, got: {other:?}"),
        }
    }

    #[test]
    fn test_extract_drive_error_falls_back_to_http_status_mapping() {
        let error = extract_drive_error(404, "not-json");

        match error {
            CloudBackupError::DriveError(msg) => assert!(msg.contains("Resource not found")),
            other => panic!("expected DriveError, got: {other:?}"),
        }
    }

    #[test]
    fn test_extract_drive_error_falls_back_to_generic_http_error() {
        let error = extract_drive_error(418, "teapot");

        match error {
            CloudBackupError::DriveError(msg) => {
                assert!(msg.contains("Drive API error (418): teapot"))
            }
            other => panic!("expected DriveError, got: {other:?}"),
        }
    }

    #[test]
    fn test_build_content_range_for_non_empty_and_empty_payload() {
        assert_eq!(build_content_range(10), "bytes 0-9/10");
        assert_eq!(build_content_range(0), "bytes 0-0/0");
    }

    #[test]
    fn test_build_resumable_file_metadata_merges_app_properties() {
        let metadata = build_resumable_file_metadata(
            "folder-1",
            "backup.db.gz",
            &json!({"appProperties": {"checksum": "abc"}}),
        );

        assert_eq!(metadata["name"], "backup.db.gz");
        assert_eq!(metadata["parents"][0], "folder-1");
        assert_eq!(metadata["appProperties"]["checksum"], "abc");
    }

    #[tokio::test]
    async fn test_resumable_upload_with_transport_success() {
        let client = GoogleDriveClient::new("id".to_string(), "token".to_string());
        let state = Arc::new(Mutex::new(FakeTransportState::new()));

        let transport = FakeResumableTransport::new(
            Ok(ResumableInitResponse {
                status: 200,
                headers: HashMap::from([(
                    "location".to_string(),
                    "https://upload.example/session".to_string(),
                )]),
                body: "{}".to_string(),
            }),
            Ok(ResumableUploadResponse {
                status: 200,
                body: r#"{"id":"file-1","name":"backup.db.gz","size":1024}"#.to_string(),
            }),
            state.clone(),
        );

        let uploaded = client
            .resumable_upload_with_transport(
                "folder-1",
                "backup.db.gz",
                vec![1, 2, 3, 4],
                json!({"appProperties": {"checksum": "abc"}}),
                &transport,
            )
            .await
            .expect("upload should succeed");

        assert_eq!(uploaded.id, "file-1");
        assert_eq!(uploaded.name, "backup.db.gz");
        assert_eq!(uploaded.size, 1024);

        let lock = state.lock().expect("state lock");
        assert_eq!(lock.init_request_size, Some(4));
        assert_eq!(lock.upload_size, Some(4));
        assert_eq!(lock.upload_data_len, Some(4));
        assert_eq!(
            lock.upload_url.clone().unwrap_or_default(),
            "https://upload.example/session"
        );
        let checksum = lock
            .init_request_metadata
            .as_ref()
            .and_then(|m| m.get("appProperties"))
            .and_then(|p| p.get("checksum"))
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        assert_eq!(checksum, "abc");
    }

    #[tokio::test]
    async fn test_resumable_upload_with_transport_maps_init_auth_error() {
        let client = GoogleDriveClient::new("id".to_string(), "token".to_string());
        let transport = FakeResumableTransport::new(
            Ok(ResumableInitResponse {
                status: 401,
                headers: HashMap::new(),
                body: "expired".to_string(),
            }),
            Ok(ResumableUploadResponse {
                status: 200,
                body: "{}".to_string(),
            }),
            Arc::new(Mutex::new(FakeTransportState::new())),
        );

        let error = client
            .resumable_upload_with_transport(
                "folder-1",
                "backup.db.gz",
                vec![1],
                json!({}),
                &transport,
            )
            .await
            .expect_err("expected auth error");

        assert!(matches!(error, CloudBackupError::TokenExpired));
    }

    #[tokio::test]
    async fn test_resumable_upload_with_transport_handles_missing_upload_url() {
        let client = GoogleDriveClient::new("id".to_string(), "token".to_string());
        let transport = FakeResumableTransport::new(
            Ok(ResumableInitResponse {
                status: 200,
                headers: HashMap::new(),
                body: "{}".to_string(),
            }),
            Ok(ResumableUploadResponse {
                status: 200,
                body: "{}".to_string(),
            }),
            Arc::new(Mutex::new(FakeTransportState::new())),
        );

        let error = client
            .resumable_upload_with_transport(
                "folder-1",
                "backup.db.gz",
                vec![1],
                json!({}),
                &transport,
            )
            .await
            .expect_err("expected missing location error");

        match error {
            CloudBackupError::DriveError(msg) => assert!(msg.contains("No upload URL")),
            other => panic!("expected DriveError, got: {other:?}"),
        }
    }

    #[tokio::test]
    async fn test_resumable_upload_with_transport_maps_network_drop_during_upload() {
        let client = GoogleDriveClient::new("id".to_string(), "token".to_string());
        let transport = FakeResumableTransport::new(
            Ok(ResumableInitResponse {
                status: 200,
                headers: HashMap::from([(
                    "location".to_string(),
                    "https://upload.example/session".to_string(),
                )]),
                body: "{}".to_string(),
            }),
            Err(CloudBackupError::NetworkError(
                "simulated network drop".to_string(),
            )),
            Arc::new(Mutex::new(FakeTransportState::new())),
        );

        let error = client
            .resumable_upload_with_transport(
                "folder-1",
                "backup.db.gz",
                vec![1, 2],
                json!({}),
                &transport,
            )
            .await
            .expect_err("expected network error");

        match error {
            CloudBackupError::NetworkError(msg) => assert!(msg.contains("network drop")),
            other => panic!("expected NetworkError, got: {other:?}"),
        }
    }

    #[tokio::test]
    async fn test_resumable_upload_with_transport_rejects_missing_id_in_payload() {
        let client = GoogleDriveClient::new("id".to_string(), "token".to_string());
        let transport = FakeResumableTransport::new(
            Ok(ResumableInitResponse {
                status: 200,
                headers: HashMap::from([(
                    "location".to_string(),
                    "https://upload.example/session".to_string(),
                )]),
                body: "{}".to_string(),
            }),
            Ok(ResumableUploadResponse {
                status: 200,
                body: r#"{"name":"backup.db.gz"}"#.to_string(),
            }),
            Arc::new(Mutex::new(FakeTransportState::new())),
        );

        let error = client
            .resumable_upload_with_transport(
                "folder-1",
                "backup.db.gz",
                vec![1],
                json!({}),
                &transport,
            )
            .await
            .expect_err("expected parse validation error");

        match error {
            CloudBackupError::DriveError(msg) => assert!(msg.contains("No file ID")),
            other => panic!("expected DriveError, got: {other:?}"),
        }
    }
}
