mod add_railway_model;
mod add_rolling_stock_to_model;
mod get_manufacturer_by_id;
mod get_manufacturers;
mod get_railway_companies;
mod get_railway_company_by_id;
mod get_railway_model_by_id;
mod get_railway_model_translations;
mod save_railway_model;
mod search_railway_models;
mod update_railway_model_classification;
mod update_railway_model_delivery_date;
mod update_railway_model_text;
mod update_rolling_stock_category;
mod update_rolling_stock_dcc;
mod update_rolling_stock_identification;
mod update_rolling_stock_railway_company;
mod update_rolling_stock_specifications;
mod upsert_railway_model_translation;

#[cfg(test)]
mod testing;

pub use add_railway_model::AddRailwayModel;
pub use add_railway_model::{
    CouplingInput, CreateRailwayModelInput, CreateRollingStockInput, LengthOverBuffersInput,
    TechnicalSpecificationsInput,
};
pub use add_rolling_stock_to_model::{
    AddRollingStockToModel, AddRollingStockToModelInput, parse_add_rolling_stock_args,
};
pub use get_manufacturer_by_id::GetManufacturerById;
pub use get_manufacturers::GetManufacturers;
pub use get_railway_companies::GetRailwayCompanies;
pub use get_railway_company_by_id::GetRailwayCompanyById;
pub use get_railway_model_by_id::GetRailwayModelById;
pub use get_railway_model_by_id::GetRailwayModelViewById;
pub use get_railway_model_translations::GetRailwayModelTranslations;
pub use save_railway_model::SaveRailwayModel;
pub use save_railway_model::{SaveRailwayModelInput, SimplifiedRollingStockInput};
pub use search_railway_models::{SearchRailwayModels, SearchRailwayModelsInput};
pub use update_railway_model_classification::UpdateRailwayModelClassification;
pub use update_railway_model_classification::UpdateRailwayModelClassificationInput;
pub use update_railway_model_delivery_date::UpdateRailwayModelDeliveryDate;
pub use update_railway_model_delivery_date::UpdateRailwayModelDeliveryDateInput;
pub use update_railway_model_text::UpdateRailwayModelText;
pub use update_railway_model_text::{RailwayModelTextField, UpdateRailwayModelTextInput};
pub use update_rolling_stock_category::UpdateRollingStockCategory;
pub use update_rolling_stock_category::UpdateRollingStockCategoryInput;
pub use update_rolling_stock_dcc::UpdateRollingStockDcc;
pub use update_rolling_stock_dcc::UpdateRollingStockDccInput;
pub use update_rolling_stock_identification::UpdateRollingStockIdentification;
pub use update_rolling_stock_identification::UpdateRollingStockIdentificationInput;
pub use update_rolling_stock_railway_company::UpdateRollingStockRailwayCompany;
pub use update_rolling_stock_railway_company::UpdateRollingStockRailwayCompanyInput;
pub use update_rolling_stock_specifications::UpdateRollingStockSpecifications;
pub use update_rolling_stock_specifications::UpdateRollingStockSpecificationsInput;
pub use upsert_railway_model_translation::{
    UpsertRailwayModelTranslation, UpsertRailwayModelTranslationInput,
};
