//! Domain layer for the trains module.
pub mod formation;
pub mod formation_category;
pub mod repository;
pub mod views;

pub use repository::{CreatePrototypeInput, TrainsRepository, TrainsUowExt};
pub use views::{
    FormationCategoryView, FormationElementView, PrototypeGroupView, PrototypeView,
    TrainFormationDetail, TrainFormationSummary, TrainFormationView,
};
