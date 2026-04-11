pub mod database;
pub mod entities;
pub mod mappers;
pub mod repository;
#[cfg(test)]
mod tests;

// Re-export the product repository for use by other infrastructure modules
// (e.g. command handlers that need direct repository access without a full UoW).
pub use repository::SqliteTrackProductRepository;
