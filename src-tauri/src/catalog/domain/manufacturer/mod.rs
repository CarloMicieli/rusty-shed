#[allow(clippy::module_inception)]
mod manufacturer;
mod manufacturer_id;
mod manufacturer_status;

pub use manufacturer::Manufacturer;
pub use manufacturer_id::ManufacturerId;
pub use manufacturer_status::ManufacturerStatus;
