use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RailwayCompany {
    pub name: String,
    pub registered_company_name: Option<String>,
    pub country_code: Option<String>,
}
