# Data Model: Train Formations

**Branch**: `039-train-formations` | **Phase**: 1 | **Date**: 2026-03-29

---

## Entity Map

```
RailwayCompany (existing)
    │
    └── Prototype (new — master catalog)
            │
            └── FormationElement (new — composition slot)
                    ├── → TrainFormation (new — header)
                    └── → owned_rolling_stocks (existing — optional assignment)

FormationCategory (new — classification)
    └── → TrainFormation.category_id
```

---

## Entities

### `RailwayCompany` (existing — no change)

Already seeded. No schema modifications required. Referenced by `Prototype.railway_company_id`.

---

### `Prototype` (new)

Defines a real-world rolling stock class or series — the master catalog entry.

| Column               | Type      | Constraints                            | Notes                                                                                                                  |
| -------------------- | --------- | -------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| `id`                 | `TEXT`    | PK                                     | Format: `trn:prototype:<railway>-<series-slug>`                                                                        |
| `railway_company_id` | `TEXT`    | NOT NULL, FK → `railway_companies(id)` | Operator (e.g., FS, SBB)                                                                                               |
| `series_code`        | `TEXT`    | NOT NULL                               | e.g., `"UIC-Z1 (Gran Comfort)"`                                                                                        |
| `car_type`           | `TEXT`    | NOT NULL                               | Enum: `Locomotive`, `PowerCar`, `Coach`, `Couchette`, `Dining`, `Sleeping`, `BaggageCar`, `ControlCar`, `FreightWagon` |
| `service_level`      | `TEXT`    | NULLABLE                               | e.g., `"1st Class"`, `"2nd Class"`, `"Mixed"`                                                                          |
| `category`           | `TEXT`    | NOT NULL                               | e.g., `"Passenger"`, `"Freight"`, `"Locomotive"`                                                                       |
| `is_motorized`       | `INTEGER` | NOT NULL DEFAULT 0                     | 1 if `car_type` is `Locomotive` or `PowerCar` at seed time; editable for custom                                        |
| `default_is_dummy`   | `INTEGER` | NOT NULL DEFAULT 0                     | 1 for display-only static prototypes (no motor even if `Locomotive` type)                                              |
| `is_custom`          | `INTEGER` | NOT NULL DEFAULT 0                     | 1 = user-created; 0 = seeded                                                                                           |
| `notes`              | `TEXT`    | NULLABLE                               | Optional description                                                                                                   |
| `created_at`         | `TEXT`    | NOT NULL DEFAULT CURRENT_TIMESTAMP     |                                                                                                                        |
| `updated_at`         | `TEXT`    | NOT NULL DEFAULT CURRENT_TIMESTAMP     |                                                                                                                        |
| `version`            | `INTEGER` | NOT NULL DEFAULT 0                     | Optimistic concurrency                                                                                                 |

**ID pattern**: `trn:prototype:fs-uic-z1-gran-comfort`
**Indexes**: `(railway_company_id, series_code)`, `(car_type)`, `(is_custom)`

**Seed data examples**:

```
trn:prototype:fs-e444-tartaruga       FS / E.444 Tartaruga    / Locomotive  / 1st Class / is_motorized=1
trn:prototype:fs-uic-z1-gran-comfort  FS / UIC-Z1 Gran Comfort/ Coach       / 1st Class / is_motorized=0
trn:prototype:fs-uic-x-1cl            FS / UIC-X (1982)       / Coach       / 1st Class / is_motorized=0
trn:prototype:fs-uic-x-2cl            FS / UIC-X (1982)       / Coach       / 2nd Class / is_motorized=0
trn:prototype:sbb-re44-ii             SBB/ Re 4/4 II          / Locomotive  / —         / is_motorized=1
trn:prototype:sbb-ewiv-1cl            SBB/ EW IV              / Coach       / 1st Class / is_motorized=0
trn:prototype:db-e103                 DB / Baureihe 103        / Locomotive  / —         / is_motorized=1
trn:prototype:db-avmz-eurocity        DB / Avmz EC             / Coach       / 1st Class / is_motorized=0
```

---

### `FormationCategory` (new)

Named classification for a type of train formation.

| Column       | Type      | Constraints                        | Notes                                   |
| ------------ | --------- | ---------------------------------- | --------------------------------------- |
| `id`         | `TEXT`    | PK                                 | Format: `trn:formation-category:<slug>` |
| `name`       | `TEXT`    | NOT NULL UNIQUE                    | e.g., `"EuroCity"`, `"TEE"`             |
| `is_custom`  | `INTEGER` | NOT NULL DEFAULT 0                 | 1 = user-created                        |
| `created_at` | `TEXT`    | NOT NULL DEFAULT CURRENT_TIMESTAMP |                                         |

**Seeded built-in values**: EuroCity, Intercity, TEE, Express, Regional, Freight, Special, Thematic

---

### `TrainFormation` (new)

The header record for a named train set.

| Column        | Type      | Constraints                               | Notes                                                              |
| ------------- | --------- | ----------------------------------------- | ------------------------------------------------------------------ |
| `id`          | `TEXT`    | PK                                        | Format: `trn:formation:<uuid>`                                     |
| `name`        | `TEXT`    | NOT NULL UNIQUE                           | FR-002: uniqueness enforced in DB and use case                     |
| `category_id` | `TEXT`    | NULLABLE, FK → `formation_categories(id)` |                                                                    |
| `start_year`  | `INTEGER` | NULLABLE                                  | Service start year                                                 |
| `end_year`    | `INTEGER` | NULLABLE                                  | Service end year; CHECK `start_year <= end_year` when both present |
| `epoch`       | `TEXT`    | NULLABLE                                  | Roman numeral epoch (I–VI+)                                        |
| `notes`       | `TEXT`    | NULLABLE                                  | Markdown-enabled content                                           |
| `created_at`  | `TEXT`    | NOT NULL DEFAULT CURRENT_TIMESTAMP        |                                                                    |
| `updated_at`  | `TEXT`    | NOT NULL DEFAULT CURRENT_TIMESTAMP        |                                                                    |
| `version`     | `INTEGER` | NOT NULL DEFAULT 0                        | Optimistic concurrency                                             |

**Constraint**: `CHECK (start_year IS NULL OR end_year IS NULL OR start_year <= end_year)`

---

### `FormationElement` (new)

A single ordered slot in a formation's composition.

| Column                   | Type      | Constraints                                                  | Notes                                                                                     |
| ------------------------ | --------- | ------------------------------------------------------------ | ----------------------------------------------------------------------------------------- |
| `id`                     | `TEXT`    | PK                                                           | Format: `trn:element:<uuid>`                                                              |
| `formation_id`           | `TEXT`    | NOT NULL, FK → `train_formations(id) ON DELETE CASCADE`      |                                                                                           |
| `prototype_id`           | `TEXT`    | NOT NULL, FK → `prototypes(id)`                              | Mandatory prototype anchor                                                                |
| `owned_rolling_stock_id` | `TEXT`    | NULLABLE, FK → `owned_rolling_stocks(id) ON DELETE SET NULL` | Optional explicit model assignment                                                        |
| `position_order`         | `INTEGER` | NOT NULL                                                     | 0-based; unique within formation                                                          |
| `traction_override`      | `INTEGER` | NOT NULL DEFAULT 0                                           | 1 = this slot counts for traction regardless of Prototype type (-1 = explicitly excluded) |
| `created_at`             | `TEXT`    | NOT NULL DEFAULT CURRENT_TIMESTAMP                           |                                                                                           |
| `updated_at`             | `TEXT`    | NOT NULL DEFAULT CURRENT_TIMESTAMP                           |                                                                                           |

**Indexes**: `(formation_id, position_order)`, `(prototype_id)`

---

### `owned_rolling_stocks` (existing — additive FK)

One **migration** adds `prototype_id` to the existing `owned_rolling_stocks` table to enable the ownership lookup join:

```sql
ALTER TABLE owned_rolling_stocks ADD COLUMN prototype_id TEXT REFERENCES prototypes(id) ON DELETE SET NULL;
CREATE INDEX IF NOT EXISTS idx_owned_rolling_stocks_prototype_id ON owned_rolling_stocks(prototype_id);
```

This is a non-breaking additive change — existing rows will have `NULL` (unlinked).

---

## State Transitions

### TrainFormation lifecycle

```
[created] ──edit──▶ [active]
   └── [deleted] (cascades to all FormationElements)
```

### FormationElement lifecycle

```
[appended] ──reorder──▶ [position updated]
   └─ ──assign model──▶ [has owned_rolling_stock_id]
   └─ ──unassign──▶ [owned_rolling_stock_id = NULL]
   └─ ──removed──▶ [deleted; siblings shift position_order]
   └─ ──prototype deleted──▶ [remains; prototype_id FK stays via SET NULL or restricted]
```

**Note on prototype deletion**: When a seeded prototype is "deleted" by a user custom override, existing `FormationElement` rows that referenced it must show a "Prototype not found" visual state (FR-020 analogue). The FK is `ON DELETE RESTRICT` for prototypes to prevent accidental breaks — soft-delete pattern preferred for prototypes.

---

## Validation Rules

| Rule                                            | Layer                                                      | Notes                                                    |
| ----------------------------------------------- | ---------------------------------------------------------- | -------------------------------------------------------- |
| Formation name uniqueness                       | DB UNIQUE + Rust use case                                  | FR-002                                                   |
| `start_year <= end_year`                        | DB CHECK + Rust + Frontend `$derived`                      | FR-003                                                   |
| `prototype_id` required on FormationElement     | DB NOT NULL + Rust                                         | Mandatory prototype anchor                               |
| Duplicate prototype slots allowed               | No uniqueness constraint on `(formation_id, prototype_id)` | FR-016                                                   |
| Same `owned_rolling_stock_id` in multiple slots | Not blocked                                                | A model can be logically "counted" in multiple positions |
| Prototype `car_type` in allowed enum            | Rust validation at boundary                                | Enforced via `validator` crate on `Args`                 |

---

## Derived / Computed Values (Frontend)

These are never stored — always derived reactively via Svelte 5 `$derived`:

| Value                 | Computation                                                                                  |
| --------------------- | -------------------------------------------------------------------------------------------- | --- | -------------------------- | --- | ---------------------- |
| `ownedCount` per slot | `SELECT COUNT(*) FROM owned_rolling_stocks WHERE prototype_id = ?` (preloaded in view query) |
| `hasTraction`         | `entries.some(e => isTractionSlot(e))`                                                       |
| `isTractionSlot(e)`   | `(e.prototype.is_motorized && !e.prototype.default_is_dummy && e.traction_override !== -1)   |     | e.traction_override === 1` |
| Form `isValidRange`   | `$derived(startYear === null                                                                 |     | endYear === null           |     | startYear <= endYear)` |

---

## SQL Migration File

**Filename**: `0009_create_train_formations_schema.sql`

```sql
-- ──────────────────────────────────────────────
-- Migration: Train Formations (Feature 039)
-- ──────────────────────────────────────────────

-- 1. Prototype Library (master catalog)
CREATE TABLE IF NOT EXISTS prototypes
(
    id                  TEXT NOT NULL PRIMARY KEY,
    railway_company_id   TEXT NOT NULL,
    series_code         TEXT NOT NULL,
    car_type            TEXT NOT NULL,
    service_level       TEXT,
    category            TEXT NOT NULL,
    is_motorized        INTEGER NOT NULL DEFAULT 0 CHECK (is_motorized IN (0,1)),
    default_is_dummy    INTEGER NOT NULL DEFAULT 0 CHECK (default_is_dummy IN (0,1)),
    is_custom           INTEGER NOT NULL DEFAULT 0 CHECK (is_custom IN (0,1)),
    notes               TEXT,
    created_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version             INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (railway_company_id) REFERENCES railway_companies(id) ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS idx_prototypes_company_series ON prototypes (railway_company_id, series_code);
CREATE INDEX IF NOT EXISTS idx_prototypes_car_type ON prototypes (car_type);
CREATE INDEX IF NOT EXISTS idx_prototypes_is_custom ON prototypes (is_custom);

-- 2. Formation Categories
CREATE TABLE IF NOT EXISTS formation_categories
(
    id          TEXT NOT NULL PRIMARY KEY,
    name        TEXT NOT NULL UNIQUE,
    is_custom   INTEGER NOT NULL DEFAULT 0 CHECK (is_custom IN (0,1)),
    created_at  TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 3. Train Formations (header)
CREATE TABLE IF NOT EXISTS train_formations
(
    id          TEXT NOT NULL PRIMARY KEY,
    name        TEXT NOT NULL UNIQUE,
    category_id TEXT,
    start_year  INTEGER,
    end_year    INTEGER,
    epoch       TEXT,
    notes       TEXT,
    created_at  TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version     INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (category_id) REFERENCES formation_categories(id) ON DELETE SET NULL,
    CHECK (start_year IS NULL OR end_year IS NULL OR start_year <= end_year)
);

CREATE INDEX IF NOT EXISTS idx_train_formations_category_id ON train_formations (category_id);
CREATE INDEX IF NOT EXISTS idx_train_formations_name ON train_formations (name);

-- 4. Formation Elements (composition slots)
CREATE TABLE IF NOT EXISTS formation_elements
(
    id                      TEXT NOT NULL PRIMARY KEY,
    formation_id            TEXT NOT NULL,
    prototype_id            TEXT NOT NULL,
    owned_rolling_stock_id  TEXT,
    position_order          INTEGER NOT NULL,
    traction_override       INTEGER NOT NULL DEFAULT 0,
    created_at              TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at              TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (formation_id)           REFERENCES train_formations(id)      ON DELETE CASCADE,
    FOREIGN KEY (prototype_id)           REFERENCES prototypes(id)            ON DELETE RESTRICT,
    FOREIGN KEY (owned_rolling_stock_id) REFERENCES owned_rolling_stocks(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_formation_elements_formation ON formation_elements (formation_id, position_order);
CREATE INDEX IF NOT EXISTS idx_formation_elements_prototype ON formation_elements (prototype_id);

-- 5. Add prototype_id to owned_rolling_stocks (additive, non-breaking)
ALTER TABLE owned_rolling_stocks ADD COLUMN prototype_id TEXT REFERENCES prototypes(id) ON DELETE SET NULL;
CREATE INDEX IF NOT EXISTS idx_owned_rolling_stocks_prototype_id ON owned_rolling_stocks (prototype_id);

-- 6. Seed: Formation Categories
INSERT OR IGNORE INTO formation_categories (id, name, is_custom) VALUES
    ('trn:formation-category:eurocity',  'EuroCity',   0),
    ('trn:formation-category:intercity', 'Intercity',  0),
    ('trn:formation-category:tee',       'TEE',        0),
    ('trn:formation-category:express',   'Express',    0),
    ('trn:formation-category:regional',  'Regional',   0),
    ('trn:formation-category:freight',   'Freight',    0),
    ('trn:formation-category:special',   'Special',    0),
    ('trn:formation-category:thematic',  'Thematic',   0);
```

---

## Seed Data File

**Location**: `src-tauri/src/trains/infrastructure/seed_data.rs` (or equivalent seed module)

The prototype seed covers common FS, SBB, DB, SNCF prototypes. A minimum viable seed list (expandable):

```rust
// Locomotives
("trn:prototype:fs-e444-tartaruga",      "trn:railway-company:fs",  "E.444 Tartaruga",      "Locomotive",    None,             "Locomotive", true,  false),
("trn:prototype:fs-e646",                "trn:railway-company:fs",  "E.646",                "Locomotive",    None,             "Locomotive", true,  false),
("trn:prototype:sbb-re44-ii",            "trn:railway-company:sbb", "Re 4/4 II",            "Locomotive",    None,             "Locomotive", true,  false),
("trn:prototype:db-e103",                "trn:railway-company:db",  "Baureihe 103",         "Locomotive",    None,             "Locomotive", true,  false),
// Coaches
("trn:prototype:fs-uic-z1-gran-comfort", "trn:railway-company:fs",  "UIC-Z1 Gran Comfort",  "Coach",         Some("1st Class"),"Passenger",  false, false),
("trn:prototype:fs-uic-x-1cl",           "trn:railway-company:fs",  "UIC-X (1982)",         "Coach",         Some("1st Class"),"Passenger",  false, false),
("trn:prototype:fs-uic-x-2cl",           "trn:railway-company:fs",  "UIC-X (1982)",         "Coach",         Some("2nd Class"),"Passenger",  false, false),
("trn:prototype:sbb-ewiv-1cl",           "trn:railway-company:sbb", "EW IV",                "Coach",         Some("1st Class"),"Passenger",  false, false),
("trn:prototype:db-avmz-eurocity",       "trn:railway-company:db",  "Avmz (EuroCity)",      "Coach",         Some("1st Class"),"Passenger",  false, false),
```
