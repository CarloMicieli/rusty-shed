use serde::{Deserialize, Serialize};

/// A strongly-typed identifier for a railway model.
///
/// This newtype wraps a `String` so that code dealing with railway model
/// identifiers can use a distinct type instead of raw `String`s. It derives
/// `Serialize` and `Deserialize` so it can be used directly with `serde`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RailwayModelId(String);
