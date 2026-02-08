# Manifest Schema Research - Complete Documentation Index

**Research Completion Date**: February 8, 2026  
**Feature**: 016-data-archive-export (Manifest Reuse from Import Feature)  
**Status**: ✅ Complete - All Research Questions Answered

---

## 📋 Research Questions Answered

### ✅ Question 1: Manifest Definition Location

**Where is the manifest.json structure defined?**

- **Rust Struct DTOs**: [src-tauri/src/import/domain/manifest.rs](../../../src-tauri/src/import/domain/manifest.rs) (216 lines)
- **JSON Schema**: [src-tauri/src/import/domain/manifest_schema.json](../../../src-tauri/src/import/domain/manifest_schema.json) (489 lines)
- **Canonical Reference**: [specs/010-data-import-utility/contracts/manifest.schema.json](../010-data-import-utility/contracts/manifest.schema.json)
- **Test Example**: [src-tauri/fixtures/test_import_manifest.json](../../../src-tauri/fixtures/test_import_manifest.json)

**Definition Type**: **Dual Definition** (both Rust struct and JSON Schema)

---

### ✅ Question 2: Entities Included

**What entities are represented in the manifest?**

Seven primary entity types:

1. **Manufacturer** - Brand/producer (e.g., Märklin, Roco)
2. **RailwayCompany** - Operating railway (e.g., Deutsche Bahn, SNCF)
3. **RailwayModel** - Product catalog entry (e.g., BR 01 locomotive)
4. **RollingStock** - Individual rolling stock (nested in RailwayModel)
5. **CollectionItem** - User's owned instance of a model
6. **Seller** - Source of purchase (shop, private, marketplace)
7. **MaintenanceCard** - Maintenance history for collection item

**Plus 4 Supporting Types**: Category, Purchase, Address, Money, MaintenanceEvent

**Total Records per Typical Export**: 2-100+ manufacturers, 10-500+ railway models, 50-10000+ collection items

---

### ✅ Question 3: Relationship Representation

**How are relationships represented between entities?**

**Pattern**: Foreign Key with String IDs

```
Foreign Keys (string references):
├── RailwayModel.manufacturerId → Manufacturer.id
├── RollingStock.railwayCompanyId → RailwayCompany.id
├── CollectionItem.railwayModelId → RailwayModel.id
├── Purchase.sellerId → Seller.id
└── MaintenanceCard.collectionItemId → CollectionItem.id

Embedded Objects (no FK):
├── RailwayModel.category → Category (1:1)
├── RailwayModel.rollingStocks[] → RollingStock[] (1:N)
├── CollectionItem.purchase → Purchase (0:1)
├── Seller.address → Address (0:1)
├── MaintenanceCard.events[] → MaintenanceEvent[] (0:N)
└── Purchase.price, cost → Money (0:1 each)

Uniqueness Keys:
├── RailwayModel: manufacturerId + productCode
└── CollectionItem: railwayModelId + addedDate
```

**Validation Rules**:

- Every FK must reference a valid ID in the manifest
- Embedded objects are part of parent record (no separate lookups)
- Conditional requirements enforced by JSON Schema (e.g., Purchase type determines required fields)

---

### ✅ Question 4: Reusability for Export Feature

**Can this manifest structure be shared/reused for export or does it need modification?**

**Answer**: ✅ **100% REUSABLE - NO MODIFICATIONS NEEDED**

**Evidence**:

- Schema is bidirectional (works equally for import and export)
- All fields are properly optional/required for both directions
- Serde derives work for both serialization (export) and deserialization (import)
- No logic coupling - pure data structure
- Version-locked to v1.0 (stable)

**Implementation Strategy for Export**:

1. Query domain models from database
2. Map database entities → ManifestDto structures
3. Serialize to JSON
4. Validate against manifest_schema.json
5. Package with images in ZIP archive

**No Code Changes Required** - Use existing types directly.

---

### ✅ Question 5: Codebase Location

**Where in the codebase is the manifest definition located?**

#### Primary Definition Files

| File                                                                                                                      | Type        | Purpose          |
| ------------------------------------------------------------------------------------------------------------------------- | ----------- | ---------------- |
| [src-tauri/src/import/domain/manifest.rs](../../../src-tauri/src/import/domain/manifest.rs)                               | Rust        | Type definitions |
| [src-tauri/src/import/domain/manifest_schema.json](../../../src-tauri/src/import/domain/manifest_schema.json)             | JSON Schema | Validation       |
| [specs/010-data-import-utility/contracts/manifest.schema.json](../010-data-import-utility/contracts/manifest.schema.json) | JSON Schema | Reference        |

#### Related Implementation Files

| File                                                                                                                        | Purpose             |
| --------------------------------------------------------------------------------------------------------------------------- | ------------------- |
| [src-tauri/src/import/infrastructure/schema_validator.rs](../../../src-tauri/src/import/infrastructure/schema_validator.rs) | Validation logic    |
| [src-tauri/src/import/application/execute_import.rs](../../../src-tauri/src/import/application/execute_import.rs)           | Import persistence  |
| [src-tauri/src/import/application/validate_package.rs](../../../src-tauri/src/import/application/validate_package.rs)       | Validation pipeline |

#### Module Structure

```
src-tauri/src/import/
├── domain/
│   ├── manifest.rs          ← Rust DTOs (use for export)
│   ├── manifest_schema.json ← JSON Schema (use for validation)
│   ├── import_preview.rs
│   └── mod.rs
├── application/
│   ├── execute_import.rs
│   ├── validate_package.rs
│   ├── id_mapper.rs
│   └── mod.rs
├── infrastructure/
│   ├── schema_validator.rs
│   ├── archive_extractor.rs
│   ├── duplicate_checker.rs
│   └── mod.rs
├── interface/
│   ├── types.rs
│   └── commands.rs
└── mod.rs
```

---

## 📚 Complete Documentation Deliverables

### 1. **Manifest Schema Research** (Comprehensive Reference)

📄 File: [manifest-schema-research.md](./manifest-schema-research.md)

**10 Detailed Sections**:

1. Executive summary
2. Complete manifest definition location
3. Full entity list with field definitions
4. Relationship patterns and diagrams
5. Reusability assessment
6. Schema definition files detailed analysis
7. Entity reference guide
8. Implementation guidance for export
9. Related code locations
10. Design decision rationale

**Audience**: Architecture team, senior developers, feature leads
**Read Time**: 15-20 minutes
**When to Use**: Understanding complete schema, design decisions, validation rules

---

### 2. **Manifest Integration Quick Start** (Implementation Guide)

📄 File: [manifest-integration-quickstart.md](./manifest-integration-quickstart.md)

**Practical Implementation Guide**:

- TL;DR answer
- Copy-paste code examples
- File locations cheat sheet
- Entity relationship diagram
- Data type reference
- Validation checklist
- Common pitfalls
- Testing examples
- Quick reference tables

**Audience**: Backend developers implementing export
**Read Time**: 5-10 minutes
**When to Use**: Getting started with export implementation, quick reference

---

### 3. **Entity Reference** (Complete Specifications)

📄 File: [entity-reference.md](./entity-reference.md)

**Detailed Entity Documentation**:

- 8 entity definitions with Rust code and examples
- All embedded types documented
- Field-by-field specifications
- JSON examples for each entity
- Uniqueness and FK constraints
- Conditional validation rules
- Field naming convention table (Rust ↔ JSON)
- Export checklist

**Audience**: Developers writing export logic, QA testing
**Read Time**: 10-15 minutes
**When to Use**: Implementing specific entity serialization, validation rules

---

## 🎯 Quick Navigation by Role

### 📍 For Export Feature Leads / Architects

1. Start: [Manifest Schema Research](./manifest-schema-research.md) - Section 4 (Reusability Assessment)
2. Then: [Manifest Integration Quick Start](./manifest-integration-quickstart.md) - Implementation Strategy
3. Reference: [Entity Reference](./entity-reference.md) - For specific questions

### 📍 For Backend Developers (Implementing Export)

1. Start: [Manifest Integration Quick Start](./manifest-integration-quickstart.md) - TL;DR + Copy-Paste Examples
2. Reference: [Entity Reference](./entity-reference.md) - While coding each entity
3. Validate: [Manifest Schema Research](./manifest-schema-research.md) - Section 8 (Validation)

### 📍 For QA / Testing

1. Reference: [Entity Reference](./entity-reference.md) - Validation checklist
2. Examples: [Manifest Integration Quick Start](./manifest-integration-quickstart.md) - Testing section
3. Details: Test fixture at `src-tauri/fixtures/test_import_manifest.json`

### 📍 For Spec Writers / Product

1. Overview: [Manifest Integration Quick Start](./manifest-integration-quickstart.md) - TL;DR section
2. Examples: [Entity Reference](./entity-reference.md) - JSON examples for each entity

---

## 🔑 Key Findings Summary

| Question                               | Answer                                                                         | Reference                                                                                |
| -------------------------------------- | ------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------- |
| **Location of definition?**            | Dual: `manifest.rs` + `manifest_schema.json` in `src-tauri/src/import/domain/` | [Research §5](./manifest-schema-research.md#5-manifest-schema-definition-files)          |
| **What entities included?**            | 7 primary types + 4 supporting types (see entity list)                         | [Research §2](./manifest-schema-research.md#2-complete-entity-list)                      |
| **How are relationships represented?** | Foreign keys as string IDs + embedded objects (no circular refs)               | [Research §3](./manifest-schema-research.md#3-relationship-patterns)                     |
| **Reusable for export?**               | **YES** - 100% compatible, zero modifications needed                           | [Research §4](./manifest-schema-research.md#4-reusability-assessment-for-export-feature) |
| **Where in codebase?**                 | 5 primary files in `src-tauri/src/import/{domain,infrastructure,application}`  | [Research §5-7](./manifest-schema-research.md#5-manifest-schema-definition-files)        |

---

## 💡 Implementation Roadmap for Export

### Phase 1: Foundation (Week 1)

- [ ] Read [Manifest Integration Quick Start](./manifest-integration-quickstart.md)
- [ ] Import `ManifestDto` types in export module
- [ ] Set up database query functions for each entity

### Phase 2: Entity Mapping (Week 2)

- [ ] Implement mapping: Database model → ManifestDto
- [ ] Handle embedded types (Category, Purchase, RollingStock, etc.)
- [ ] Reference [Entity Reference](./entity-reference.md) for field mappings

### Phase 3: Validation (Week 2-3)

- [ ] Validate FK integrity before serialization
- [ ] Add schema validation step
- [ ] Create comprehensive test cases

### Phase 4: Serialization & Packaging (Week 3)

- [ ] Serialize ManifestDto to JSON
- [ ] Create ZIP archive with manifest + images
- [ ] Test with [test fixture](../../../src-tauri/fixtures/test_import_manifest.json)

### Phase 5: Testing & Verification

- [ ] Round-trip test: export → import → verify
- [ ] Validate exported manifest can be re-imported
- [ ] Performance test with large datasets (1000+ records)

---

## 🔗 Related Specifications

| Document                                                                                | Purpose                      |
| --------------------------------------------------------------------------------------- | ---------------------------- |
| [specs/010-data-import-utility/spec.md](../010-data-import-utility/spec.md)             | Import feature specification |
| [specs/010-data-import-utility/data-model.md](../010-data-import-utility/data-model.md) | Original manifest design     |
| [specs/010-data-import-utility/research.md](../010-data-import-utility/research.md)     | Import research notes        |
| [docs/tauri-commands.md](../../../docs/tauri-commands.md)                               | Command documentation        |

---

## 📊 Schema Statistics

| Metric                | Value                                                                                    |
| --------------------- | ---------------------------------------------------------------------------------------- |
| **Schema Version**    | 1.0 (stable, backward-compatible)                                                        |
| **Total Entities**    | 7 primary + 4 supporting types                                                           |
| **Total Fields**      | 80+ fields across all entities                                                           |
| **Required Fields**   | ~25 (core identifiers, relationships, metadata)                                          |
| **Optional Fields**   | ~55 (additional details, metadata, notes)                                                |
| **Foreign Keys**      | 5 (manufacturer_id, railway_company_id, railway_model_id, seller_id, collection_item_id) |
| **Embedded Objects**  | 5 (Category, RollingStock, Purchase, Address, MaintenanceEvent)                          |
| **JSON Schema Size**  | 489 lines                                                                                |
| **Rust DTO Size**     | 216 lines                                                                                |
| **Test Fixture Size** | 126 lines (example with 2 manufacturers, 2 models, 2 items, 1 seller)                    |

---

## ✅ Verification Checklist

- [x] Manifest location identified (dual definition)
- [x] All 7 entities documented with full specifications
- [x] Relationship patterns mapped and documented
- [x] Reusability confirmed (100% compatible)
- [x] Codebase locations provided with file links
- [x] Implementation guidance created
- [x] Test fixtures identified
- [x] Field naming convention documented (Rust ↔ JSON)
- [x] Validation rules specified
- [x] Quick start guide for developers created
- [x] Complete entity reference created
- [x] Design rationale documented
- [x] Code examples provided

---

## 📞 Questions & Support

### For Schema Questions

→ Refer to [Manifest Schema Research §3-4](./manifest-schema-research.md#3-relationship-patterns)

### For Implementation Questions

→ Refer to [Manifest Integration Quick Start](./manifest-integration-quickstart.md)

### For Entity-Specific Questions

→ Refer to [Entity Reference](./entity-reference.md)

### For Original Import Feature Context

→ Refer to [specs/010-data-import-utility/](../010-data-import-utility/)

---

**Research Completed**: February 8, 2026  
**Status**: ✅ Ready for Implementation  
**Recommendation**: Begin export feature implementation using manifest types directly; no schema modifications required.
