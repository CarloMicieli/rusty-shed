# ADR 9: Localized Data Management and Full-Text Search Strategy

Status: Accepted

Date: 2026-01-24

Deciders: Project Lead

## 1. Context and Problem Statement

The model railway collection management app requires a system to store and retrieve localized product descriptions (initially 4 languages: EN, DE, FR, IT). We need a solution that integrates with Tauri 2 (Rust/SQLite) and Svelte 5, supports _Full-Text Search_ (FTS), and maintains a Clean Architecture.

## 2. Decision Drivers

* **Scalability:** The ability to add new languages without schema migrations.
* **Search Performance:** High-speed searching of technical terms (e.g., "Dampflok", "Pantograph").
* **Type Safety:** Using Rust's type system to distinguish between plain strings and localized content.
* **IPC Efficiency:** Sending only the required language data to the Svelte 5 UI.

## 3. Considered Options

### **Option A: Relational Table (Translation Table)**

Separate table for strings with language codes.
* **Pros:**
    * Native SQLite FTS5 support for high-performance indexing.
    * Allows SQL-level filtering and fallback logic (COALESCE).
    * Keeps the main `products` table lean.
* **Cons:**
    * Requires `JOIN` operations for every read.
    * More boilerplate for `INSERT/UPDATE` operations.

### **Option B: JSON Blob**
* **Pros:**
    * Simplified schema; no extra tables or joins.
    * Faster reads for single rows.
* **Cons:**
    * Poor FTS5 support (cannot easily index keys/values separately).
    * Increases IPC payload if the entire blob is sent to the UI.
    * Shifts filtering logic from the DB to the Rust application layer.
Storing a JSON object `{"en": "...", "de": "..."}` in a column within the main products table.

## 4. Decision Outcome

Chosen Option: **Chosen Option: Option A (Relational Table).** This approach is superior for a collection manager where search accuracy across different languages is a primary feature. We will use a "one-to-many" relationship coupled with SQLite FTS5 triggers

### Consequences

In a **MADR** format, the "Consequences" section is where you acknowledge both the positive outcomes and the trade-offs (the "price you pay") for your decision.

For your model railway app, the consequences center on the shift of complexity from the UI/Domain layer into the Database/Infrastructure layer.

Here is the **Consequences** section for your ADR:

---

## Consequences

### Positive (The Wins)

* **Data Integrity:** Foreign keys ensure that no translation exists without a parent product, preventing "orphan" text.
* **Search Performance:** By using a relational table and FTS5, searching for technical terms like "Dampflok" or "Epoche III" remains O(log N) even as the collection grows.
* **Minimal IPC Payload:** The app only sends the strings for the active language over the Tauri bridge, keeping memory usage low on the Svelte side.
* **Type Safety:** The `LocalizedString` type in Rust prevents developers from accidentally treating a localized object as a raw string.

### Negative (The Trade-offs)

* **Increased Write Complexity:** Every "Create Product" action now requires multiple database inserts (one for the product, at least one for the translation).
* **Join Overhead:** Reading a product now requires a `LEFT JOIN`, which is slightly more CPU-intensive for the database than a simple single-table select.
* **Migration Management:** Adding a new localized field (e.g., adding a `short_history` field) now requires updating both the relational table and the FTS5 virtual table/triggers.

### Neutral / Risks

* **Fallback Maintenance:** The system assumes "en" is always present. If a product is added without an English translation, the `COALESCE` logic must be robust enough to handle nulls or return the SKU as a last resort.
* **Trigger Maintenance:** If the database schema changes, triggers must be manually updated to keep the `product_search_idx` in sync, as SQLx cannot verify triggers at compile-time.

### Technical Details

#### 1. Database Schema & FTS

```sql
-- Core Railway Model Table
CREATE TABLE railway_models (
    id TEXT PRIMARY KEY,
    manufactuer_id TEXT NOT NULL,
    product_code TEXT NOT NULL,
    ....
);

-- Localization Table
CREATE TABLE railway_model_translations (
    railway_model_id TEXT NOT NULL,
    language_code TEXT NOT NULL,
    description TEXT,
    details TEXT,
    PRIMARY KEY (railway_model_id, language_code),
    FOREIGN KEY (railway_model_id) REFERENCES railway_models(id)
);

-- Search Index
CREATE VIRTUAL TABLE railway_model_search_idx USING fts5(
    railway_model_id UNINDEXED, 
    language_code, 
    description,
    details,
    tokenize = 'unicode61'
);

-- Trigger to keep search index in sync
CREATE TRIGGER tr_railway_model_translations_insert 
AFTER INSERT ON railway_model_translations BEGIN
  INSERT INTO railway_model_search_idx(
    railway_model_id, language_code, description, details
  )
  VALUES (
    new.railway_model_id, new.language_code, new.description, new.details
  );
END;
```

#### 2. Domain Layer: Custom Types

We define a `LocalizedString` type to ensure the business logic explicitly handles language context.

```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalizedString {
    pub lang: String,
    pub value: String,
}

pub struct RailwayModel {
    pub id: RailwayModelId,
    pub product_code: ProductCode,
    pub description: LocalizedString,
    pub details: Option<LocalizedString>,
}

```

#### 3. Infrastructure Layer: Fetching with Fallback

The Rust repository uses `COALESCE` to provide a fallback language (e.g., English) if the user's preferred language is missing.

```rust
pub async fn get_model(pool: &SqlitePool, id: &str, lang: &str) -> Result<RailwayModel, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT 
            p.id, p.product_code,
            COALESCE(t.language_code, f.language_code) as res_lang,
            COALESCE(t.description, f.description) as res_desc,
            COALESCE(t.details, f.details) as res_details,
            ...
        FROM railway_models p
        LEFT JOIN product_translations t 
        ON p.id = t.railway_model_id AND t.language_code = ?
        LEFT JOIN product_translations f 
        ON p.id = f.railway_model_id AND f.language_code = 'en'
        WHERE p.id = ?
        "#,
        lang, id
    ).fetch_one(pool).await?;

    Ok(RailwayModel {
        id: row.id,
        product_code: row.product_code,
        description: LocalizedString { lang: row.res_lang.unwrap(), value: row.res_desc.unwrap() },
        details: LocalizedString { lang: row.res_lang.unwrap(), value: row.res_details.unwrap() },
    })
}

```

#### 4. UI Layer: Svelte 5 Integration

IPC commands send only the relevant `LocalizedString` to the Svelte frontend.

```svelte
<script lang="ts">
  let model = $state(null);
  
  async function load(id, lang) {
    // Rust returns { name: { lang: 'de', value: 'Dampflok' }, ... }
    model = await invoke('get_model', { id, lang });
  }
</script>

{#if model}
  <h3>{model.description.value}</h3>
  <p>{model.details.value}</p>
{/if}
```
