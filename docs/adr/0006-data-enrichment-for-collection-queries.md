# ADR 5: Data Enrichment for Collection Queries

Status: Accepted

Date: 2026-01-13

Deciders: Project Lead

## 1. Context and Problem Statement

The application follows a Clean Architecture. We have two distinct domain areas:

1. **Catalog Domain**: Contains master data for railway models (Scale, Epoch, Company, etc.).
1. **Collection Domain**: Manages the user's personal inventory.

To maintain domain boundaries and follow the "Lean Aggregate" principle, the `Collection` aggregate only stores the `railway_model_id` for each collection item. However, the User Interface requires a "rich" view that includes catalog metadata (e.g., "Show me my collection with the epoch and company for each item").

We need a way to provide this enriched data to the Tauri frontend without polluting the Collection Domain with Catalog logic or causing performance bottlenecks.

## 2. Decision Drivers

- _Domain Isolation (Boundary Integrity)_: We must ensure the Collection domain does not become "polluted" with Catalog fields. If the collection logic starts calculating things based on Epoch or Scale, the domains have leaked into one another.
- _Performance (UI Responsiveness)_: In a desktop app, users expect instantaneous loading of their collection. We must avoid "N+1" query patterns (multiple database round-trips) that would cause stuttering in the Tauri frontend.
- _Operational Simplicity_: Since this is a local SQLite-based app, we want to avoid the overhead of complex background synchronization or event-driven consistency (which would be required for the "Projection" alternative).
- _Single Source of Truth_: We must ensure that if a model's name is updated in the Catalog, the Collection view reflects this change immediately without needing manual cache-invalidation.
- _Type Safety_: The solution must leverage Rust’s type system to ensure that the data sent to the Tauri frontend matches the expected schema of the TypeScript/frontend components.

## 3. Considered Options

### Option A: Application Service Orchestration (The "Lazy Join")

The Application Layer coordinates the data retrieval. It fetches the IDs from the `CollectionRepository` and then passes those IDs to the `CatalogRepository` to fetch the metadata. The service then maps these two lists into a single DTO.

Pros:

- Strict adherence to Clean architecture/DDD principles; domains remain completely ignorant of each other's persistence logic.
- Repositories stay highly reusable and focused on a single entity.

Cons:

- Performance: Potential "N+1" query overhead.
- Complexity: Requires significant "boilerplate" mapping code in the Application Layer to stitch data together based on matching IDs.

### Option B: Dedicated Query Port (The "CQRS-Lite" Join)

A specific Port is defined in the Application layer for "Collection Views." The Infrastructure implementation uses a SQL JOIN to retrieve both ownership data and metadata in a single database round-trip.

Pros:

- High Performance: Leverages the relational database's native ability to join data efficiently.
- Simplicity: Reduces the amount of Rust code needed for manual data stitching.
- Read-Model Optimization: Allows the UI to receive exactly what it needs without extra fields.

Cons:

- Infrastructure Coupling: The persistence adapter for the Collection must now have knowledge of the Catalog table schema.
- Evolution: If the Catalog schema changes significantly, the Query Adapter must be updated alongside it.

### Option C: Read Model Projections (The "Sync" Table)

A dedicated table (e.g., `collection_display_view`) is maintained in the database. Whenever a model is added to a collection, the system writes a flattened version of all data (IDs + Metadata) into this table.

Pros:

- Read operations are as fast as a simple `SELECT *`.
- The query is decoupled from any logical domain boundaries.

Cons:

- Data Stale-ness: If the master catalog data changes (e.g., correcting a typo in a manufacturer name), the read model becomes out of sync unless a complex update mechanism is implemented.
- Storage: Duplicates data across multiple tables.

### Option D: Domain Aggregate Enrichment (The "Antipattern")

Adding metadata fields (Epoch, Scale) directly into the `Collection` Domain Aggregate and the collection table.

Pros:

- Very easy to implement initially.

Cons:

- Violation of Single Source of Truth: Metadata belongs to the Catalog. If you store it in the Collection table, you have to update it in two places.
- Domain Pollution: The Collection domain starts caring about Catalog concerns, leading to a "Big Ball of Mud."

## 4. Decision Outcome

Chosen Option: Dedicated Query Port (Option B)

### Justification

We have chosen the Dedicated Query Port (CQRS-Lite) approach as the primary method for data enrichment. This decision is based on the following justifications:

- Performance vs. Isolation Balance: While Option 1 (Orchestration) offers perfect isolation, the performance penalty of N+1 queries is unacceptable for a desktop application. Option 2 provides "near-instant" performance via SQL JOIN while maintaining boundary isolation at the Logic level, even if the Persistence level is shared.
- Infrastructure as the "Integration Point": In Clean Architecture, the Infrastructure layer is the natural place for technology-specific optimizations. By placing the SQL JOIN here, we keep the complexity out of the Domain and Application cores.
- Real-time Consistency: Unlike Option 3 (Projections), using a SQL JOIN ensures that any updates to the master Catalog data are immediately visible in the Collection view without requiring event handlers or cache invalidation logic.
- Reduced Boilerplate: This approach avoids the complex "manual stitching" code required in the Application layer, leading to a more maintainable Rust codebase.

### Consequences

Positive (Benefits)

- Simplified Application Logic: The Application Service remains a thin pass-through, simply calling the port and returning the DTO.
- Frontend-Optimized Data: The `CollectionItemView` DTO can be evolved to match UI requirements (e.g., adding a `display_name` field) without touching the core Collection Aggregate.
- Efficient Persistence: We take full advantage of SQLite's relational capabilities, ensuring the app remains snappy even as the user's collection grows to thousands of items.

Negative (Risks & Mitigations)

- Cross-Domain Knowledge in Infrastructure: The `SqliteCollectionQueryAdapter` must know about both collection and catalog tables.
  - Mitigation: We will isolate this knowledge to a specific `QueryAdapter` file, ensuring it does not leak into the standard Repository implementations.

- DTO Proliferation: As more screens are added (e.g., a "Wishlist" or "Maintenance Log"), we may end up with multiple "View" DTOs.
  - Mitigation: We will organize these in a dedicated `application::queries` module to keep them separate from Domain models.

- Schema Rigidity: Changing a table name in the Catalog domain will break the Collection query.
  - Mitigation: We will use a SQL View (as discussed) to provide a stable interface, decoupling the Rust code from the underlying table names.
