/// Manifest normalization utilities.
///
/// Converts old-format (DB-native SCREAMING_SNAKE_CASE) enum values to
/// their schema-canonical lowercase/camelCase equivalents before validation.
/// This allows importing archives produced by earlier versions of the app.
use serde_json::Value;

pub struct Normalizer;

impl Normalizer {
    pub fn new() -> Self {
        Self
    }

    /// Normalize all enum fields in a manifest JSON value in-place.
    ///
    /// Must be called before schema validation so that DB-native values
    /// like `"ORIGINAL_MINT"` are accepted as the canonical `"mint"`.
    pub fn normalize_manifest(manifest: &mut Value) {
        let data = match manifest.get_mut("data").and_then(|d| d.as_object_mut()) {
            Some(d) => d,
            None => return,
        };

        if let Some(manufacturers) = data.get_mut("manufacturers").and_then(|v| v.as_array_mut()) {
            for item in manufacturers.iter_mut() {
                normalize_field(item, "status", normalize_manufacturer_status);
            }
        }

        if let Some(companies) = data
            .get_mut("railwayCompanies")
            .and_then(|v| v.as_array_mut())
        {
            for item in companies.iter_mut() {
                normalize_field(item, "status", normalize_railway_company_status);
            }
        }

        if let Some(sellers) = data.get_mut("sellers").and_then(|v| v.as_array_mut()) {
            for item in sellers.iter_mut() {
                normalize_field(item, "sellerType", normalize_seller_type);
            }
        }

        if let Some(models) = data.get_mut("railwayModels").and_then(|v| v.as_array_mut()) {
            for item in models.iter_mut() {
                normalize_field(item, "availabilityStatus", normalize_availability_status);
                normalize_field(item, "powerMethod", normalize_power_method);
            }
        }

        if let Some(items) = data
            .get_mut("collectionItems")
            .and_then(|v| v.as_array_mut())
        {
            for item in items.iter_mut() {
                normalize_field(item, "purchaseCondition", normalize_purchase_condition);
                normalize_field(item, "modelCondition", normalize_model_condition);
                normalize_field(item, "boxCondition", normalize_box_condition);
                if let Some(purchase) = item.get_mut("purchase") {
                    normalize_field(purchase, "type", normalize_purchase_type);
                }
            }
        }

        if let Some(cards) = data
            .get_mut("maintenanceCards")
            .and_then(|v| v.as_array_mut())
        {
            for card in cards.iter_mut() {
                if let Some(events) = card.get_mut("events").and_then(|v| v.as_array_mut()) {
                    for event in events.iter_mut() {
                        normalize_field(event, "type", normalize_maintenance_type);
                    }
                }
            }
        }
    }
}

impl Default for Normalizer {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// If `obj[field]` is a string, replace it with the converter's output.
/// Leaves the field unchanged if the converter returns `None` (already canonical).
fn normalize_field<F>(obj: &mut Value, field: &str, converter: F)
where
    F: Fn(&str) -> Option<&'static str>,
{
    if let Some(Value::String(raw)) = obj.get(field)
        && let Some(canonical) = converter(raw.as_str())
    {
        obj[field] = Value::String(canonical.to_string());
    }
}

// ─── Per-field converters ─────────────────────────────────────────────────────

fn normalize_manufacturer_status(s: &str) -> Option<&'static str> {
    match s {
        "ACTIVE" => Some("active"),
        "MERGED" => Some("merged"),
        "OUT_OF_BUSINESS" => Some("outOfBusiness"),
        _ => None,
    }
}

fn normalize_railway_company_status(s: &str) -> Option<&'static str> {
    match s {
        "ACTIVE" => Some("active"),
        "INACTIVE" => Some("inactive"),
        _ => None,
    }
}

fn normalize_availability_status(s: &str) -> Option<&'static str> {
    match s {
        "AVAILABLE" => Some("available"),
        "ANNOUNCED" => Some("announced"),
        "CANCELLED" => Some("cancelled"),
        "DISCONTINUED" => Some("discontinued"),
        _ => None,
    }
}

fn normalize_power_method(s: &str) -> Option<&'static str> {
    match s {
        "AC" => Some("ac"),
        "DC" => Some("dc"),
        "TRIX_EXPRESS" => Some("trixExpress"),
        _ => None,
    }
}

fn normalize_seller_type(s: &str) -> Option<&'static str> {
    match s {
        "SHOP" => Some("shop"),
        "PRIVATE" => Some("private"),
        "MARKETPLACE" => Some("marketplace"),
        "DISTRIBUTOR" => Some("distributor"),
        _ => None,
    }
}

fn normalize_purchase_type(s: &str) -> Option<&'static str> {
    match s.to_ascii_uppercase().as_str() {
        "PURCHASED" => Some("purchased"),
        "SOLD" => Some("sold"),
        "PREORDERED" => Some("preordered"),
        _ => None,
    }
}

fn normalize_purchase_condition(s: &str) -> Option<&'static str> {
    match s {
        "NEW" => Some("new"),
        "PRE_OWNED" => Some("preowned"),
        "USED" => Some("used"),
        _ => None,
    }
}

fn normalize_model_condition(s: &str) -> Option<&'static str> {
    match s {
        "MINT" | "NEAR_MINT" => Some("mint"),
        "EXCELLENT" | "VERY_GOOD" => Some("excellent"),
        "GOOD" => Some("good"),
        "FAIR" => Some("fair"),
        "POOR" | "FOR_PARTS" => Some("poor"),
        _ => None,
    }
}

fn normalize_box_condition(s: &str) -> Option<&'static str> {
    match s {
        "ORIGINAL_MINT" => Some("mint"),
        "ORIGINAL_GOOD" | "REPLACEMENT_BOX" => Some("good"),
        "ORIGINAL_WORN" => Some("damaged"),
        "NO_BOX" => Some("missing"),
        _ => None,
    }
}

fn normalize_maintenance_type(s: &str) -> Option<&'static str> {
    match s {
        "WHEEL_CLEANING" | "TRACK_CLEANING" | "CONTACT_CLEANING" => Some("cleaning"),
        "LUBRICATION" | "GEAR_GREASE" => Some("lubrication"),
        "MOTOR_BRUSH_REPLACEMENT"
        | "TRACTION_TIRE_REPLACEMENT"
        | "SPEAKER_REPAIR"
        | "COUPLER_ADJUSTMENT"
        | "DETAIL_REPAIR"
        | "DECODER_INSTALL"
        | "FIRMWARE_UPDATE"
        | "STAY_ALIVE_INSTALL"
        | "OTHER" => Some("repair"),
        "WEATHERING" => Some("modification"),
        "GENERAL_INSPECTION" => Some("inspection"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn normalizes_manufacturer_status() {
        let mut m = json!({
            "version": "1.0",
            "data": {
                "manufacturers": [{"id": "m1", "name": "Märklin", "status": "ACTIVE"}]
            }
        });
        Normalizer::normalize_manifest(&mut m);
        assert_eq!(m["data"]["manufacturers"][0]["status"], "active");
    }

    #[test]
    fn normalizes_box_condition() {
        let mut m = json!({
            "version": "1.0",
            "data": {
                "collectionItems": [{
                    "id": "ci1", "railwayModelId": "rm1", "addedDate": "2024-01-01",
                    "boxCondition": "ORIGINAL_MINT"
                }]
            }
        });
        Normalizer::normalize_manifest(&mut m);
        assert_eq!(m["data"]["collectionItems"][0]["boxCondition"], "mint");
    }

    #[test]
    fn leaves_canonical_values_unchanged() {
        let mut m = json!({
            "version": "1.0",
            "data": {
                "manufacturers": [{"id": "m1", "name": "Märklin", "status": "active"}]
            }
        });
        Normalizer::normalize_manifest(&mut m);
        assert_eq!(m["data"]["manufacturers"][0]["status"], "active");
    }
}
