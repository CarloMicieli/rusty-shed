# ADR: Localized Data Management and Full-Text Search Strategy

## Status
Proposed

## Context and Problem Statement
The model railway collection management app requires a system to store and retrieve localized product descriptions (initially 4 languages: EN, DE, FR, IT). We need a solution that integrates with Tauri 2 (Rust/SQLite) and Svelte 5, supports Full-Text Search (FTS), and maintains a clean Hexagonal Architecture.

## Decision Drivers
* **Scalability:** The ability to add new languages without schema migrations.
* **Search Performance:** High-speed searching of technical terms (e.g., "Dampflok", "Pantograph").
* **Type Safety:** Using Rust's type system to distinguish between plain strings and localized content.
* **IPC Efficiency:** Sending only the required language data to the Svelte 5 UI.

## Considered Options
1. **Option 1: Relational Table (Translation Table)** - Separate table for strings with language codes.
2. **Option 2: JSON Blob** - Storing a JSON object `{"en": "...", "de": "..."}` in a column within the main products table.

---

## Pros and Cons of the Options

### Option 1: Relational Table
* **Pros:**
    * Native SQLite FTS5 support for high-performance indexing.
    * Allows SQL-level filtering and fallback logic (COALESCE).
    * Keeps the main `products` table lean.
* **Cons:**
    * Requires `JOIN` operations for every read.
    * More boilerplate for `INSERT/UPDATE` operations.

### Option 2: JSON Blob
* **Pros:**
    * Simplified schema; no extra tables or joins.
    * Faster reads for single rows.
* **Cons:**
    * Poor FTS5 support (cannot easily index keys/values separately).
    * Increases IPC payload if the entire blob is sent to the UI.
    * Shifts filtering logic from the DB to the Rust application layer.

---

## Decision Outcome
**Chosen Option: Option 1 (Relational Table).** This approach is superior for a collection manager where search accuracy across different languages is a primary feature. We will use a "one-to-many" relationship coupled with SQLite FTS5 triggers


