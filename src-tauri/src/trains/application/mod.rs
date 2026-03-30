//! Application use-case handlers for train formations.
pub mod add_formation_element;
pub mod assign_rolling_stock_to_element;
pub mod create_custom_prototype;
pub mod create_formation_category;
pub mod create_train_formation;
pub mod delete_train_formation;
pub mod get_formation_categories;
pub mod get_prototypes;
pub mod get_train_formation;
pub mod get_train_formations;
pub mod remove_formation_element;
pub mod reorder_formation_elements;
pub mod set_traction_override;
pub mod update_train_formation;

#[cfg(test)]
mod tests;
