# Rust ↔ Frontend Bridge — API Reference

This reference enumerates the IPC commands exposed from Rust to the SvelteKit frontend via Tauri 2. Commands are registered in [src-tauri/src/lib.rs](src-tauri/src/lib.rs#L29-L121) and are invokable from JS using `invoke('<command_name>', args)`.

## Calling From Frontend

- Import: `import { invoke } from '@tauri-apps/api/tauri';`
- Pattern: `const result = await invoke<Return>('command_name', { /* args */ });`
- All commands return `CommandError` on failure; validation errors may include field-specific messages, while some map to `Unknown` with a string payload.

## Commands

### App & System

| Command             | Params | Returns  | Notes                                                                                                       |
| ------------------- | ------ | -------- | ----------------------------------------------------------------------------------------------------------- |
| `is_db_initialized` | none   | `bool`   | Checks `AppState.initialized` after migrations/seed ([src-tauri/src/lib.rs](src-tauri/src/lib.rs#L29-L34)). |
| `get_app_version`   | none   | `String` | Crate version from `CARGO_PKG_VERSION` ([src-tauri/src/lib.rs](src-tauri/src/lib.rs#L36-L40)).              |

### Catalog

| Command                     | Params                           | Returns                  | Notes                                                                                                                                                         |
| --------------------------- | -------------------------------- | ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `get_manufacturer_by_id`    | `manufacturer_id: String`        | `Option<Manufacturer>`   | Validates `ManufacturerId`; DB fetch ([src-tauri/src/catalog/interface/command_handlers.rs](src-tauri/src/catalog/interface/command_handlers.rs#L18-L56)).    |
| `get_railway_company_by_id` | `railway_company_id: String`     | `Option<RailwayCompany>` | Validates `RailwayCompanyId`; DB fetch ([src-tauri/src/catalog/interface/command_handlers.rs](src-tauri/src/catalog/interface/command_handlers.rs#L60-L101)). |
| `get_railway_model_by_id`   | `railway_model_id: String`       | `Option<RailwayModel>`   | Validates `RailwayModelId`; DB fetch ([src-tauri/src/catalog/interface/command_handlers.rs](src-tauri/src/catalog/interface/command_handlers.rs#L103-L139)).  |
| `get_railway_models_by_ids` | `railway_model_ids: Vec<String>` | `Vec<RailwayModel>`      | Batch fetch with ID validation ([src-tauri/src/catalog/interface/command_handlers.rs](src-tauri/src/catalog/interface/command_handlers.rs#L141-L171)).        |
| `create_railway_model`      | `input: CreateRailwayModelInput` | `String` (new model ID)  | Runs use-case in UoW/transaction ([src-tauri/src/catalog/interface/command_handlers.rs](src-tauri/src/catalog/interface/command_handlers.rs#L173-L210)).      |

### Collecting / Depot

| Command          | Params | Returns      | Notes                                                                                                                                                             |
| ---------------- | ------ | ------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `get_collection` | none   | `Collection` | Uses `GetCollectionUseCase` in DB UoW ([src-tauri/src/collecting/interface/command_handlers.rs](src-tauri/src/collecting/interface/command_handlers.rs#L22-L66)). |
| `get_depot`      | none   | `Collection` | Alias of `get_collection` ([src-tauri/src/collecting/interface/command_handlers.rs](src-tauri/src/collecting/interface/command_handlers.rs#L68-L74)).             |

### Dashboard

| Command             | Params | Returns            | Notes                                                                                                                        |
| ------------------- | ------ | ------------------ | ---------------------------------------------------------------------------------------------------------------------------- |
| `dashboard_summary` | none   | `DashboardSummary` | Aggregates collection, wishlists, maintenance snapshots ([src-tauri/src/dashboard.rs](src-tauri/src/dashboard.rs#L40-L115)). |

### Collection (In-Memory Demo)

| Command                  | Params                             | Returns                   | Notes                                                                                                                         |
| ------------------------ | ---------------------------------- | ------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `list_collection_items`  | `search?: String`                  | `Vec<CollectionItemLite>` | In-memory mutex store; optional case-insensitive search ([src-tauri/src/collection.rs](src-tauri/src/collection.rs#L43-L78)). |
| `create_collection_item` | `input: CreateCollectionItemInput` | `CollectionItemLite`      | Adds item to in-memory store ([src-tauri/src/collection.rs](src-tauri/src/collection.rs#L80-L104)).                           |
| `update_collection_item` | `input: UpdateCollectionItemInput` | `CollectionItemLite`      | Updates item by ID; errors if not found ([src-tauri/src/collection.rs](src-tauri/src/collection.rs#L106-L136)).               |
| `delete_collection_item` | `id: String`                       | `()`                      | Removes item by ID; errors if missing ([src-tauri/src/collection.rs](src-tauri/src/collection.rs#L138-L157)).                 |

### Wishlists

| Command                | Params                         | Returns                | Notes                                                                                                                                                                                         |
| ---------------------- | ------------------------------ | ---------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `get_wishlists`        | none                           | `Vec<WishlistPreview>` | Lists wishlists ([src-tauri/src/wishlist/interface/command_handlers.rs](src-tauri/src/wishlist/interface/command_handlers.rs#L31-L47)).                                                       |
| `get_wishlist_by_id`   | `id: String`                   | `Option<Wishlist>`     | Validates `WishlistId`; returns wishlist with items ([src-tauri/src/wishlist/interface/command_handlers.rs](src-tauri/src/wishlist/interface/command_handlers.rs#L19-L29)).                   |
| `create_wishlist`      | `input: CreateWishlistInput`   | `WishlistPreview`      | Creates wishlist (optional `is_default`) and returns preview ([src-tauri/src/wishlist/interface/command_handlers.rs](src-tauri/src/wishlist/interface/command_handlers.rs#L49-L102)).         |
| `rename_wishlist`      | `input: RenameWishlistInput`   | `()`                   | Renames wishlist by ID ([src-tauri/src/wishlist/interface/command_handlers.rs](src-tauri/src/wishlist/interface/command_handlers.rs#L104-L134)).                                              |
| `delete_wishlist`      | `id: String`                   | `()`                   | Deletes wishlist ([src-tauri/src/wishlist/interface/command_handlers.rs](src-tauri/src/wishlist/interface/command_handlers.rs#L136-L152)).                                                    |
| `set_default_wishlist` | `id: String`                   | `()`                   | Marks wishlist as default ([src-tauri/src/wishlist/interface/command_handlers.rs](src-tauri/src/wishlist/interface/command_handlers.rs#L154-L172)).                                           |
| `add_to_wishlist`      | `input: AddToWishlistInput`    | `WishlistItem`         | Adds item with priority/status/desired price; parses dates/currency ([src-tauri/src/wishlist/interface/command_handlers.rs](src-tauri/src/wishlist/interface/command_handlers.rs#L174-L243)). |
| `remove_from_wishlist` | `item_id: String`              | `()`                   | Removes item by ID ([src-tauri/src/wishlist/interface/command_handlers.rs](src-tauri/src/wishlist/interface/command_handlers.rs#L245-L265)).                                                  |
| `move_item_to_list`    | `input: MoveWishlistItemInput` | `()`                   | Moves item to another wishlist ([src-tauri/src/wishlist/interface/command_handlers.rs](src-tauri/src/wishlist/interface/command_handlers.rs#L267-L297)).                                      |

### Maintenance

| Command                     | Params                       | Returns                | Notes                                                                                                                                                                                  |
| --------------------------- | ---------------------------- | ---------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `get_maintenance_dashboard` | none                         | `Vec<MaintenanceCard>` | Lists due/overdue cards ([src-tauri/src/maintenance/interface/command_handlers.rs](src-tauri/src/maintenance/interface/command_handlers.rs#L15-L43)).                                  |
| `add_maintenance_record`    | `input: AddMaintenanceInput` | `()`                   | Parses UUIDs and `YYYY-MM-DD`, updates card and persists ([src-tauri/src/maintenance/interface/command_handlers.rs](src-tauri/src/maintenance/interface/command_handlers.rs#L45-L93)). |

### Sellers

| Command            | Params                         | Returns               | Notes                                                                                                                                                                             |
| ------------------ | ------------------------------ | --------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `get_sellers`      | none                           | `Vec<Seller>`         | Lists sellers ([src-tauri/src/sellers/interface/command_handlers.rs](src-tauri/src/sellers/interface/command_handlers.rs#L12-L32)).                                               |
| `get_seller_by_id` | `id: String`                   | `Option<Seller>`      | Validates `SellerId`; fetches seller ([src-tauri/src/sellers/interface/command_handlers.rs](src-tauri/src/sellers/interface/command_handlers.rs#L34-L61)).                        |
| `create_seller`    | `payload: CreateSellerPayload` | `Seller`              | Creates seller via use-case ([src-tauri/src/sellers/interface/command_handlers.rs](src-tauri/src/sellers/interface/command_handlers.rs#L63-L105)).                                |
| `update_seller`    | `payload: UpdateSellerPayload` | `Seller`              | Updates seller; parses ID; allows contact/location fields ([src-tauri/src/sellers/interface/command_handlers.rs](src-tauri/src/sellers/interface/command_handlers.rs#L107-L154)). |
| `delete_seller`    | `id: String`                   | `u64` (rows affected) | Deletes seller by ID ([src-tauri/src/sellers/interface/command_handlers.rs](src-tauri/src/sellers/interface/command_handlers.rs#L156-L181)).                                      |

## Data Contracts (selected)

- Types are exported to TS during debug builds at `src/lib/bindings.ts` via `tauri_specta` ([src-tauri/src/lib.rs](src-tauri/src/lib.rs#L42-L55)). Key payloads/DTOs include:
  - `CreateRailwayModelInput` (catalog create), `CreateWishlistInput`, `RenameWishlistInput`, `AddToWishlistInput`, `MoveWishlistItemInput`, `CreateSellerPayload`, `UpdateSellerPayload`, `AddMaintenanceInput`, `CollectionItemLite`, `CreateCollectionItemInput`, `UpdateCollectionItemInput`.
- Monetary amounts use `rust_decimal` and custom currency codes; wishlist prices validate currency codes; dates expect `YYYY-MM-DD` where strings are used.

## Error Model

- `CommandError` is the common error envelope; commands often map DB issues to `DatabaseError` and validation issues to field-specific errors, otherwise `Unknown` with a message. Frontend should surface meaningful messages and allow retry for transient DB errors.

## Capabilities & Permissions (Bridge-Relevant)

- Only the `default` capability is declared for the `main` window with HTTP limited to `http://localhost:*`, logging enabled, and devtools toggle allowed ([src-tauri/capabilities/default.json](src-tauri/capabilities/default.json)). Commands that rely on HTTP plugin calls must stay within this allowlist unless the capability is expanded.
