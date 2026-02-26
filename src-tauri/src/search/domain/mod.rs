pub mod global_search_result;
pub mod repository;

pub use global_search_result::{GlobalSearchResult, SearchSource};
pub use repository::{GlobalSearchRepository, GlobalSearchUowExt};

#[cfg(test)]
pub use repository::MockGlobalSearchRepository;
