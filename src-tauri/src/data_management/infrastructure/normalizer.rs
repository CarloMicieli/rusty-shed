/// Manifest normalization utilities.
///
/// Converts old-format (lowercase/camelCase) enum values to
/// schema-canonical tokens before validation.
/// This allows importing archives produced by earlier versions of the app.
use serde_json::Value;

pub struct Normalizer;

impl Normalizer {
    pub fn new() -> Self {
        Self
    }

    /// Normalize all enum fields in a manifest JSON value in-place.
    ///
    /// Must be called before schema validation so that legacy values
    /// like `"mint"` are accepted as the canonical `"ORIGINAL_MINT"`.
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
                normalize_field(item, "category", normalize_category);
                if let Some(rolling_stocks) =
                    item.get_mut("rollingStocks").and_then(|v| v.as_array_mut())
                {
                    for rs in rolling_stocks.iter_mut() {
                        normalize_field(rs, "serviceLevel", normalize_service_level);
                        normalize_field(rs, "control", normalize_control);
                    }
                }
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
                        normalize_maintenance_event_type(event);
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
        "active" => Some("ACTIVE"),
        "merged" => Some("MERGED"),
        "outOfBusiness" => Some("OUT_OF_BUSINESS"),
        _ => None,
    }
}

fn normalize_railway_company_status(s: &str) -> Option<&'static str> {
    match s {
        "active" => Some("ACTIVE"),
        "inactive" => Some("INACTIVE"),
        "merged" => Some("MERGED"),
        _ => None,
    }
}

fn normalize_availability_status(s: &str) -> Option<&'static str> {
    match s {
        "available" => Some("AVAILABLE"),
        "announced" => Some("ANNOUNCED"),
        "cancelled" => Some("CANCELLED"),
        "discontinued" => Some("DISCONTINUED"),
        _ => None,
    }
}

fn normalize_power_method(s: &str) -> Option<&'static str> {
    match s {
        "ac" => Some("AC"),
        "dc" => Some("DC"),
        "trixExpress" => Some("TRIX_EXPRESS"),
        _ => None,
    }
}

fn normalize_seller_type(s: &str) -> Option<&'static str> {
    match s {
        "shop" => Some("SHOP"),
        "private" => Some("PRIVATE"),
        "marketplace" => Some("MARKETPLACE"),
        "distributor" => Some("DISTRIBUTOR"),
        _ => None,
    }
}

fn normalize_category(s: &str) -> Option<&'static str> {
    match s {
        "locomotive" => Some("LOCOMOTIVES"),
        "trainSet" => Some("TRAIN_SETS"),
        "freightCar" => Some("FREIGHT_CARS"),
        "passengerCar" => Some("PASSENGER_CARS"),
        "electricMultipleUnit" => Some("ELECTRIC_MULTIPLE_UNITS"),
        "railcar" => Some("RAILCARS"),
        _ => None,
    }
}

fn normalize_service_level(s: &str) -> Option<&'static str> {
    match s {
        "1" => Some("FIRST"),
        "2" => Some("SECOND"),
        "3" => Some("THIRD"),
        "1/2" => Some("FIRST_SECOND"),
        "2/3" => Some("SECOND_THIRD"),
        "1/2/3" => Some("FIRST_SECOND_THIRD"),
        _ => None,
    }
}

fn normalize_control(s: &str) -> Option<&'static str> {
    match s {
        "dccReady" => Some("DCC_READY"),
        "dccFitted" => Some("DCC_FITTED"),
        "dccSound" => Some("DCC_SOUND"),
        "noDcc" => Some("NO_DCC"),
        _ => None,
    }
}

fn normalize_purchase_type(s: &str) -> Option<&'static str> {
    match s.to_ascii_uppercase().as_str() {
        "PURCHASED" => Some("purchased"),
        "SOLD" => Some("sold"),
        "PREORDERED" => Some("preOrdered"),
        _ => None,
    }
}

fn normalize_purchase_condition(s: &str) -> Option<&'static str> {
    match s {
        "new" => Some("NEW"),
        "used" | "preowned" => Some("PRE_OWNED"),
        _ => None,
    }
}

fn normalize_model_condition(s: &str) -> Option<&'static str> {
    match s {
        "mint" => Some("MINT"),
        "excellent" => Some("EXCELLENT"),
        "good" => Some("GOOD"),
        "fair" => Some("FAIR"),
        "poor" => Some("POOR"),
        _ => None,
    }
}

fn normalize_box_condition(s: &str) -> Option<&'static str> {
    match s {
        "mint" => Some("ORIGINAL_MINT"),
        "good" => Some("ORIGINAL_GOOD"),
        "damaged" => Some("ORIGINAL_WORN"),
        "missing" => Some("NO_BOX"),
        _ => None,
    }
}

/// Normalize a maintenance event's `type` field.
///
/// Handles three cases:
/// - DB-native SCREAMING_SNAKE_CASE string → canonical schema value
/// - `null` or missing field → `"repair"` (same as `OTHER`; safe fallback for legacy data)
/// - Already-canonical string → unchanged (converter returns `None`)
fn normalize_maintenance_event_type(event: &mut Value) {
    match event.get("type") {
        Some(Value::String(raw)) => {
            if let Some(canonical) = normalize_maintenance_type(raw.as_str()) {
                event["type"] = Value::String(canonical.to_string());
            }
        }
        // null value or missing field: insert default so schema validation passes
        Some(Value::Null) | None => {
            event["type"] = Value::String("repair".to_string());
        }
        _ => {}
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
        assert_eq!(m["data"]["manufacturers"][0]["status"], "ACTIVE");
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
        assert_eq!(
            m["data"]["collectionItems"][0]["boxCondition"],
            "ORIGINAL_MINT"
        );
    }

    #[test]
    fn normalizes_maintenance_event_type_null_to_repair() {
        let mut m = json!({
            "version": "1.0",
            "data": {
                "maintenanceCards": [{
                    "id": "mc1", "collectionItemId": "ci1",
                    "events": [{ "id": "ev1", "date": "2026-01-01", "type": null }]
                }]
            }
        });
        Normalizer::normalize_manifest(&mut m);
        assert_eq!(
            m["data"]["maintenanceCards"][0]["events"][0]["type"],
            "repair"
        );
    }

    #[test]
    fn normalizes_maintenance_event_type_missing_to_repair() {
        let mut m = json!({
            "version": "1.0",
            "data": {
                "maintenanceCards": [{
                    "id": "mc1", "collectionItemId": "ci1",
                    "events": [{ "id": "ev1", "date": "2026-01-01" }]
                }]
            }
        });
        Normalizer::normalize_manifest(&mut m);
        assert_eq!(
            m["data"]["maintenanceCards"][0]["events"][0]["type"],
            "repair"
        );
    }

    #[test]
    fn normalizes_maintenance_event_type_uppercase_to_canonical() {
        let mut m = json!({
            "version": "1.0",
            "data": {
                "maintenanceCards": [{
                    "id": "mc1", "collectionItemId": "ci1",
                    "events": [{ "id": "ev1", "date": "2026-01-01", "type": "WHEEL_CLEANING" }]
                }]
            }
        });
        Normalizer::normalize_manifest(&mut m);
        assert_eq!(
            m["data"]["maintenanceCards"][0]["events"][0]["type"],
            "cleaning"
        );
    }

    #[test]
    fn leaves_canonical_maintenance_event_type_unchanged() {
        let mut m = json!({
            "version": "1.0",
            "data": {
                "maintenanceCards": [{
                    "id": "mc1", "collectionItemId": "ci1",
                    "events": [{ "id": "ev1", "date": "2026-01-01", "type": "cleaning" }]
                }]
            }
        });
        Normalizer::normalize_manifest(&mut m);
        assert_eq!(
            m["data"]["maintenanceCards"][0]["events"][0]["type"],
            "cleaning"
        );
    }

    #[test]
    fn leaves_canonical_values_unchanged() {
        let mut m = json!({
            "version": "1.0",
            "data": {
                "manufacturers": [{"id": "m1", "name": "Märklin", "status": "ACTIVE"}]
            }
        });
        Normalizer::normalize_manifest(&mut m);
        assert_eq!(m["data"]["manufacturers"][0]["status"], "ACTIVE");
    }
}
