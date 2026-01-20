# ADR 3: Local Persistence Strategy (SQLite & SQLx)

**Status:** Accepted

**Date:** 2026-01-08

**Deciders:** Project Lead

## 1. Context and Problem Statement

The application manages highly relational data: a "Model" has many "Maintenance Records," and "Maintenance Records" might reference specific "Spare Parts." We need a persistence layer that ensures data integrity and allows the schema to evolve safely as the app grows from a simple inventory to a full maintenance tracker.

## 2. Decision Drivers

- **Relational Integrity:** Must prevent "orphaned" records (e.g., maintenance logs for a deleted locomotive).
- **Schema Evolution:** A reliable way to update the database structure on user devices without data loss.
- **Rust Integration:** Direct access from the Rust business logic layer (as per ADR 2).
- **Portability:** Seamless operation across Windows, macOS, Linux, Android, and iOS.

---

## 3. Considered Options

### Option A: Tauri Official SQL Plugin

- **Discarded because:** Interaction happens primarily via JavaScript. It lacks the deep Rust integration needed for complex backend logic and compile-time query validation.

### Option B: SQLite via `rusqlite`

- **Pros:** Synchronous, lightweight, and simple.
- **Cons:** No built-in async support (requires manual threading) and lacks the compile-time SQL verification found in newer tools.

### Option C: SQLite via `sqlx` (Chosen)

- **Pros:** \* **Compile-time Verification:** Queries are checked against a real database during compilation.
- **Async Native:** Works perfectly with Tauri’s async command system.
- **Migration Management:** Built-in toolset for versioning the database schema.

- **Cons:** Slightly more complex setup (requires a DATABASE_URL for the compiler).

---

## 4. Decision Outcome

**Chosen Option: SQLite with `sqlx**`

### Justification

`sqlx` allows us to treat the database as a type-safe extension of our Rust code. Given that this app manages physical collections where data loss is frustrating for users, the robustness of SQLite combined with `sqlx`'s safety features is the optimal choice.

### Key Requirements

#### A. Foreign Key (FK) Enforcement

SQLite disables Foreign Key constraints by default.

- **Requirement:** Every database connection **must** execute `PRAGMA foreign_keys = ON;` immediately upon opening.
- **Purpose:** To ensure that deleting a locomotive correctly triggers a `CASCADE` delete or a failure if maintenance logs exist, preventing database corruption.

#### B. SQLx Migrations

- **Requirement:** All schema changes must be handled via `sqlx` migration files (`.sql` files in a `/migrations` folder).
- **Requirement:** Migrations must be **embedded** in the Rust binary using `sqlx::migrate!().run(&pool).await`.
- **Purpose:** This ensures that when a user updates the app on their phone or desktop, the database schema is automatically updated to the latest version upon launch.

---

## 5. Consequences

- **Positive:** High confidence in data relationships via enforced FKs.
- **Positive:** Automatic schema updates for end-users via embedded migrations.
- **Negative:** The build process requires a `sqlx-data.json` file or an active database connection to verify queries (Offline mode is available via `SQLX_OFFLINE=true`).
- **Neutral:** Maintenance records will be stored in a hidden `app_data_dir` to prevent users from accidentally deleting the database file.
