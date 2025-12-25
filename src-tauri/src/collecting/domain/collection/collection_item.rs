use crate::catalog::domain::{DeliveryDate, Epoch, PowerMethod, ProductCode, Scale};
use crate::collecting::domain::collection::{OwnedRollingStock, PurchaseInfo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionItem {
    pub id: String,
    pub collection_id: String,
    pub manufacturer: String,
    pub product_code: ProductCode,
    pub description: String,
    pub power_method: PowerMethod,
    pub scale: Scale,
    pub epoch: Epoch,
    pub delivery_date: DeliveryDate,

    pub rolling_stocks: Vec<OwnedRollingStock>,
    pub purchase_info: Option<PurchaseInfo>,
}
