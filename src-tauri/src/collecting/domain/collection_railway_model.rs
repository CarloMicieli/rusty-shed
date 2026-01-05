use crate::catalog::domain::railway_model::{Category, Control, Epoch};
use crate::catalog::domain::scale::Scale;
use serde::{Deserialize, Serialize};

/// A lightweight representation of a railway model for collection display purposes.
///
/// This struct captures the essential details of a railway model as needed
/// for displaying in a collection context, omitting extraneous catalog information.
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, specta::Type)]
pub struct CollectionRailwayModel {
    /// The manufacturer of the railway model.
    pub manufacturer: String,
    /// The product code of the railway model.
    pub product_code: String,
    /// The scale of the railway model.
    pub scale: Scale,
    /// The epoch of the railway model.
    pub epoch: Epoch,
    /// A brief description of the railway model.
    pub description: String,
    /// The control type of the railway model, if specified.
    pub control: Option<Control>,
    /// The category of the railway model, if specified.
    pub category: Option<Category>,
}
