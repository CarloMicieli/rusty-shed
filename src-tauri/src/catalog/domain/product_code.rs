use serde::{Deserialize, Serialize};

/// A product identifier (manufacturer model/code) used to uniquely identify
/// a rolling stock model or catalogue item.
///
/// This is a thin newtype wrapper around `String` to provide domain-level
/// type-safety and to allow attaching trait impls specific to product codes.
///
/// It derives `Serialize`/`Deserialize` for easy (de)serialization with Serde.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[specta(transparent)]
pub struct ProductCode(pub String);
