use crate::catalog::domain::railway_model::{RailwayModelId, RailwayModelUowExt};
use crate::core::domain::Language;
use crate::core::domain::domain_error::DomainError;
use crate::media::domain::image_validation::{ImageFormat, ModelImagePath, StorageError};
use crate::media::infrastructure::FileStorage;

/// Input for image deletion
#[derive(Debug, Clone)]
pub struct DeleteImageInput {
    pub model_id: RailwayModelId,
}

/// Use case for deleting model images
pub struct DeleteModelImage {
    storage: FileStorage,
}

impl DeleteModelImage {
    /// Create a new instance with the given storage
    pub fn new(storage: FileStorage) -> Self {
        Self { storage }
    }

    /// Execute the delete use case
    ///
    /// # Steps
    /// 1. Validate model exists
    /// 2. Try to find and delete image with any supported extension
    /// 3. Return success (idempotent - no error if image doesn't exist)
    ///
    /// # Errors
    /// - DomainError::NotFound: Model doesn't exist
    /// - StorageError: File deletion failed (permissions, etc.)
    pub async fn execute<U>(
        &self,
        input: DeleteImageInput,
        unit_of_work: &mut U,
    ) -> Result<(), DeleteError>
    where
        U: RailwayModelUowExt + Send,
    {
        // Step 1: Validate model exists
        self.validate_model_exists(&input.model_id, unit_of_work)
            .await?;

        // Step 2: Try to delete image with any supported extension
        let mut deleted = false;
        for format in [ImageFormat::Jpeg, ImageFormat::Png, ImageFormat::WebP] {
            let path =
                ModelImagePath::new(self.storage.storage_dir(), input.model_id.as_ref(), format);

            if path.exists() {
                tracing::debug!("Deleting image: {}", path.full_path().display());
                self.storage
                    .delete_image(&path)
                    .await
                    .map_err(DeleteError::Storage)?;
                deleted = true;
                break; // Only one image per model
            }
        }

        if deleted {
            tracing::info!(
                "Successfully deleted image for model {}",
                input.model_id.as_ref()
            );
        } else {
            tracing::debug!(
                "No image found to delete for model {}",
                input.model_id.as_ref()
            );
        }

        Ok(())
    }

    /// Validate that the model exists in the database
    async fn validate_model_exists<U>(
        &self,
        model_id: &RailwayModelId,
        unit_of_work: &mut U,
    ) -> Result<(), DeleteError>
    where
        U: RailwayModelUowExt + Send,
    {
        let mut repository = unit_of_work.railway_model_repository();
        let model = repository
            .find_by_id(model_id, Language::English)
            .await
            .map_err(DeleteError::Domain)?;

        match model {
            Some(_) => Ok(()),
            None => Err(DeleteError::ModelNotFound(model_id.as_ref().to_string())),
        }
    }
}

/// Errors that can occur during image deletion
#[derive(Debug, thiserror::Error)]
pub enum DeleteError {
    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),

    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::manufacturer::ManufacturerId;
    use crate::catalog::domain::railway_model::localized_field::LocalizedField;
    use crate::catalog::domain::railway_model::{
        Category, MockRailwayModelRepository, PowerMethod, ProductCode, RailwayModel,
        RailwayModelRepository,
    };
    use crate::catalog::domain::scale::Scale;
    use crate::core::domain::Language;
    use crate::core::domain::identifiers::Identifier;
    use mockall::predicate::*;
    use tempfile::TempDir;

    // Mock UnitOfWork for testing
    #[derive(Default)]
    struct FakeUow {
        railway_models_repo: Option<MockRailwayModelRepository>,
    }

    impl FakeUow {
        fn with_railway_models_repo(railway_models_repo: MockRailwayModelRepository) -> Self {
            Self {
                railway_models_repo: Some(railway_models_repo),
            }
        }
    }

    impl RailwayModelUowExt for FakeUow {
        fn railway_model_repository(&mut self) -> Box<dyn RailwayModelRepository + '_> {
            Box::new(
                self.railway_models_repo
                    .take()
                    .expect("railway model repository already taken"),
            )
        }
    }

    fn create_test_railway_model(model_id_str: &str) -> RailwayModel {
        let railway_model_id = RailwayModelId::try_from(model_id_str).unwrap();
        RailwayModel {
            id: railway_model_id.clone(),
            manufacturer_id: ManufacturerId::from_string_unchecked(
                "trn:manufacturer:marklin".to_string(),
            ),
            product_code: ProductCode::try_from("39216").unwrap(),
            description: LocalizedField {
                lang: Language::English,
                value: "Test model".to_string(),
            },
            details: None,
            power_method: PowerMethod::DC,
            scale: Scale::H0,
            epoch: "IV".into(),
            category: Category::Locomotives,
            delivery_date: None,
            availability_status: None,
            rolling_stocks: vec![],
            pending_events: Vec::new(),
        }
    }

    #[tokio::test]
    async fn test_delete_image_success_jpeg() {
        let temp_dir = TempDir::new().unwrap();
        let storage_dir = temp_dir.path().join("storage");
        let storage = FileStorage::new(storage_dir.clone()).unwrap();

        let model_id_str = "trn:railway-model:marklin:39216";

        // Create an existing JPEG image
        let image_path = ModelImagePath::new(&storage_dir, model_id_str, ImageFormat::Jpeg);
        std::fs::write(image_path.full_path(), b"image data").unwrap();
        assert!(image_path.exists());

        let model_id = RailwayModelId::try_from(model_id_str).unwrap();

        let mut mock_repo = MockRailwayModelRepository::new();
        let test_model = create_test_railway_model(model_id_str);
        mock_repo
            .expect_find_by_id()
            .withf(move |id, _lang| id.as_ref() == model_id_str)
            .times(1)
            .returning(move |_, _| Ok(Some(test_model.clone())));

        let mut uow = FakeUow::with_railway_models_repo(mock_repo);

        let use_case = DeleteModelImage::new(storage);
        let input = DeleteImageInput { model_id };

        let result = use_case.execute(input, &mut uow).await;
        assert!(result.is_ok());
        assert!(!image_path.exists());
    }

    #[tokio::test]
    async fn test_delete_image_success_png() {
        let temp_dir = TempDir::new().unwrap();
        let storage_dir = temp_dir.path().join("storage");
        let storage = FileStorage::new(storage_dir.clone()).unwrap();

        let model_id_str = "trn:railway-model:roco:12345";

        // Create an existing PNG image
        let image_path = ModelImagePath::new(&storage_dir, model_id_str, ImageFormat::Png);
        std::fs::write(image_path.full_path(), b"png data").unwrap();
        assert!(image_path.exists());

        let model_id = RailwayModelId::try_from(model_id_str).unwrap();

        let mut mock_repo = MockRailwayModelRepository::new();
        let test_model = create_test_railway_model(model_id_str);
        mock_repo
            .expect_find_by_id()
            .withf(move |id, _lang| id.as_ref() == model_id_str)
            .times(1)
            .returning(move |_, _| Ok(Some(test_model.clone())));

        let mut uow = FakeUow::with_railway_models_repo(mock_repo);

        let use_case = DeleteModelImage::new(storage);
        let input = DeleteImageInput { model_id };

        let result = use_case.execute(input, &mut uow).await;
        assert!(result.is_ok());
        assert!(!image_path.exists());
    }

    #[tokio::test]
    async fn test_delete_image_no_image_exists() {
        let temp_dir = TempDir::new().unwrap();
        let storage_dir = temp_dir.path().join("storage");
        let storage = FileStorage::new(storage_dir.clone()).unwrap();

        // No image exists for this model
        let model_id_str = "trn:railway-model:fleischmann:4321";
        let jpeg_path = ModelImagePath::new(&storage_dir, model_id_str, ImageFormat::Jpeg);
        let png_path = ModelImagePath::new(&storage_dir, model_id_str, ImageFormat::Png);
        let webp_path = ModelImagePath::new(&storage_dir, model_id_str, ImageFormat::WebP);

        assert!(!jpeg_path.exists());
        assert!(!png_path.exists());
        assert!(!webp_path.exists());

        let model_id = RailwayModelId::try_from(model_id_str).unwrap();

        let mut mock_repo = MockRailwayModelRepository::new();
        let test_model = create_test_railway_model(model_id_str);
        mock_repo
            .expect_find_by_id()
            .withf(move |id, _lang| id.as_ref() == model_id_str)
            .times(1)
            .returning(move |_, _| Ok(Some(test_model.clone())));

        let mut uow = FakeUow::with_railway_models_repo(mock_repo);

        let use_case = DeleteModelImage::new(storage);
        let input = DeleteImageInput { model_id };

        // Should succeed (idempotent operation)
        let result = use_case.execute(input, &mut uow).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_image_model_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let storage_dir = temp_dir.path().join("storage");
        let storage = FileStorage::new(storage_dir.clone()).unwrap();

        let model_id_str = "trn:railway-model:nonexistent:999";
        let model_id = RailwayModelId::try_from(model_id_str).unwrap();

        let mut mock_repo = MockRailwayModelRepository::new();
        mock_repo
            .expect_find_by_id()
            .withf(move |id, _lang| id.as_ref() == model_id_str)
            .times(1)
            .returning(|_, _| Ok(None));

        let mut uow = FakeUow::with_railway_models_repo(mock_repo);

        let use_case = DeleteModelImage::new(storage);
        let input = DeleteImageInput { model_id };

        let result = use_case.execute(input, &mut uow).await;
        assert!(result.is_err());
        assert!(matches!(result, Err(DeleteError::ModelNotFound(_))));
    }

    #[tokio::test]
    async fn test_delete_image_finds_any_format() {
        let temp_dir = TempDir::new().unwrap();
        let storage_dir = temp_dir.path().join("storage");
        let storage = FileStorage::new(storage_dir.clone()).unwrap();

        let model_id_str = "trn:railway-model:lima:5678";

        // Create a WebP image (should be found even though we check JPEG and PNG first)
        let webp_path = ModelImagePath::new(&storage_dir, model_id_str, ImageFormat::WebP);
        std::fs::write(webp_path.full_path(), b"webp data").unwrap();
        assert!(webp_path.exists());

        let model_id = RailwayModelId::try_from(model_id_str).unwrap();

        let mut mock_repo = MockRailwayModelRepository::new();
        let test_model = create_test_railway_model(model_id_str);
        mock_repo
            .expect_find_by_id()
            .withf(move |id, _lang| id.as_ref() == model_id_str)
            .times(1)
            .returning(move |_, _| Ok(Some(test_model.clone())));

        let mut uow = FakeUow::with_railway_models_repo(mock_repo);

        let use_case = DeleteModelImage::new(storage);
        let input = DeleteImageInput { model_id };

        let result = use_case.execute(input, &mut uow).await;
        assert!(result.is_ok());
        assert!(!webp_path.exists());
    }
}
