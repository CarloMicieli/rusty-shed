//! Railway Model Image Entity
//!
//! Domain entity representing an image associated with a railway model.

use crate::catalog::domain::railway_model::RailwayModelId;
use crate::media::domain::ImageError;
use std::path::{Path, PathBuf};

/// Represents an image for a railway model.
///
/// This entity encapsulates the logic for resolving image file paths
/// from model IDs, following the naming convention:
/// `{model_id_with_underscores}.{ext}`
///
/// # Example
///
/// ```ignore
/// use railway_model_image::RailwayModelImage;
/// use railway_model_id::RailwayModelId;
///
/// let model_id = RailwayModelId::try_from("trn:railway-model:roco:43210")?;
/// let models_dir = Path::new("/app/data/models");
/// let image = RailwayModelImage::from_model_id(&model_id, models_dir)?;
///
/// assert_eq!(image.filename(), "trn_railway-model_roco_43210");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RailwayModelImage {
    /// The railway model ID this image belongs to
    model_id: RailwayModelId,
    /// Full filesystem path to the image
    path: PathBuf,
    /// Whether the image file exists on disk
    exists: bool,
}

impl RailwayModelImage {
    /// Create a new `RailwayModelImage` from a model ID and base directory.
    ///
    /// This constructor resolves the filename from the model ID by replacing
    /// colons with underscores, but does NOT check if the file exists.
    /// Use the infrastructure layer's `ImageRepository` to verify existence.
    ///
    /// # Arguments
    ///
    /// * `model_id` - The railway model identifier
    /// * `models_dir` - Base directory where model images are stored
    ///
    /// # Returns
    ///
    /// Returns a `RailwayModelImage` with `exists = false`. The caller must
    /// use `ImageRepository::find_image()` to locate the actual file.
    pub fn from_model_id(model_id: &RailwayModelId, models_dir: &Path) -> Result<Self, ImageError> {
        let filename = Self::resolve_filename(model_id);
        let path = models_dir.join(&filename);

        Ok(RailwayModelImage {
            model_id: model_id.clone(),
            path,
            exists: false,
        })
    }

    /// Resolve the base filename (without extension) from a model ID.
    ///
    /// Converts the model ID by replacing colons with underscores.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Model ID: "trn:railway-model:roco:43210"
    /// // Filename: "trn_railway-model_roco_43210"
    /// let filename = RailwayModelImage::resolve_filename(&model_id);
    /// ```
    pub fn resolve_filename(model_id: &RailwayModelId) -> String {
        model_id.as_ref().replace(':', "_")
    }

    /// Get the model ID associated with this image.
    pub fn model_id(&self) -> &RailwayModelId {
        &self.model_id
    }

    /// Get the full path to the image file.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Check if the image file exists.
    ///
    /// Note: This returns the cached existence check result.
    /// Use `ImageRepository` to perform fresh filesystem checks.
    pub fn exists(&self) -> bool {
        self.exists
    }

    /// Get the base filename without extension.
    pub fn filename(&self) -> String {
        Self::resolve_filename(&self.model_id)
    }

    /// Create a new instance with updated path and existence flag.
    ///
    /// Used by the infrastructure layer after resolving the actual file.
    pub fn with_path_and_exists(mut self, path: PathBuf, exists: bool) -> Self {
        self.path = path;
        self.exists = exists;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_filename() {
        let model_id = RailwayModelId::try_from("trn:railway-model:roco:43210").unwrap();
        let filename = RailwayModelImage::resolve_filename(&model_id);

        assert_eq!(filename, "trn_railway-model_roco_43210");
    }

    #[test]
    fn test_from_model_id() {
        let model_id = RailwayModelId::try_from("trn:railway-model:fleischmann:6380").unwrap();
        let models_dir = Path::new("/tmp/models");

        let image = RailwayModelImage::from_model_id(&model_id, models_dir).unwrap();

        assert_eq!(image.model_id(), &model_id);
        assert_eq!(image.filename(), "trn_railway-model_fleischmann_6380");
        assert!(!image.exists());
        assert!(
            image
                .path()
                .to_str()
                .unwrap()
                .contains("trn_railway-model_fleischmann_6380")
        );
    }

    #[test]
    fn test_with_path_and_exists() {
        let model_id = RailwayModelId::try_from("trn:railway-model:marklin:3000").unwrap();
        let models_dir = Path::new("/tmp/models");

        let image = RailwayModelImage::from_model_id(&model_id, models_dir)
            .unwrap()
            .with_path_and_exists(PathBuf::from("/tmp/models/test.png"), true);

        assert!(image.exists());
        assert_eq!(image.path(), Path::new("/tmp/models/test.png"));
    }
}
