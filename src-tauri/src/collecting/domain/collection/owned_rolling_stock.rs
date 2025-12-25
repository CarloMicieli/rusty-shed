use crate::catalog::domain::{Category, RailwayCompany, ServiceLevel, SubCategory};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnedRollingStock {
    pub id: String,
    pub item_id: String,
    pub road_number: String,
    pub type_name: String,
    pub series: Option<String>,
    pub railway: RailwayCompany,
    pub category: Category,
    pub sub_category: Option<SubCategory>,
    pub depot: Option<String>,
    pub length: Option<f64>,
    pub livery: Option<String>,
    pub service_level: Option<ServiceLevel>,
}
