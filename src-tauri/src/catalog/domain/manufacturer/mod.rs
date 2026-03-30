#[allow(clippy::module_inception)]
mod manufacturer;
mod manufacturer_id;
mod manufacturer_status;
mod repositories;

pub use manufacturer::Manufacturer;
pub use manufacturer_id::ManufacturerId;
pub use manufacturer_id::validate_manufacturer_id;
pub use manufacturer_status::ManufacturerStatus;
pub use repositories::ManufacturerRepository;
pub use repositories::ManufacturerUowExt;

#[cfg(test)]
pub use repositories::MockManufacturerRepository;
