# Tauri Commands Reference

This document provides a comprehensive overview of all Tauri commands available in the Rusty Shed application, organized by feature domain. Each command is categorized as either a **Command** (writes/modifies data) or **Query** (reads data) following CQRS principles.

## Table of Contents

- [Core / System](#core--system)
- [Settings](#settings)
- [Catalog](#catalog)
- [Collecting](#collecting)
- [Dashboard](#dashboard)
- [Wishlist](#wishlist)
- [Sellers](#sellers)
- [Maintenance](#maintenance)
- [Tracks Inventory](#tracks-inventory)

---

## Core / System

System-level commands for application initialization and state management.

| Command Name        | Type    | Description                                                            |
| ------------------- | ------- | ---------------------------------------------------------------------- |
| `is_db_initialized` | Query   | Check if the database has been initialized                             |
| `get_app_version`   | Query   | Retrieve the application version string                                |
| `init_database`     | Command | Initialize the database with migrations, seeding, and default settings |
| `show_main_window`  | Command | Show and focus the main application window                             |
| `get_image_path`    | Query   | Retrieve the absolute path for static or railway model images          |

---

## Settings

Application configuration and user preferences.

| Command Name      | Type    | Description                                                                                           |
| ----------------- | ------- | ----------------------------------------------------------------------------------------------------- |
| `get_settings`    | Query   | Retrieve current application settings (currency, length unit, favorite scale, power method, language) |
| `update_settings` | Command | Update application settings and persist changes                                                       |

---

## Catalog

Railway model catalog management, including manufacturers and railway companies.

### Railway Models

| Command Name              | Type    | Description                                               |
| ------------------------- | ------- | --------------------------------------------------------- |
| `get_railway_model_by_id` | Query   | Retrieve a specific railway model by its identifier       |
| `create_railway_model`    | Command | Create a new railway model with associated rolling stocks |

### Manufacturers

| Command Name             | Type  | Description                                        |
| ------------------------ | ----- | -------------------------------------------------- |
| `get_manufacturers`      | Query | Retrieve all manufacturers from the database       |
| `get_manufacturer_by_id` | Query | Retrieve a specific manufacturer by its identifier |

### Railway Companies

| Command Name                | Type  | Description                                           |
| --------------------------- | ----- | ----------------------------------------------------- |
| `get_railway_companies`     | Query | Retrieve all railway companies from the database      |
| `get_railway_company_by_id` | Query | Retrieve a specific railway company by its identifier |

---

## Collecting

User collection management for rolling stock items.

| Command Name             | Type    | Description                                                |
| ------------------------ | ------- | ---------------------------------------------------------- |
| `get_collection`         | Query   | Retrieve the default collection view with all items        |
| `get_depot`              | Query   | Retrieve the depot view (rolling stocks in the collection) |
| `add_collection_item`    | Command | Add a new item to the collection with purchase details     |
| `remove_collection_item` | Command | Remove an item from the collection by ID                   |

---

## Dashboard

Dashboard summary and overview statistics.

| Command Name            | Type  | Description                                                                         |
| ----------------------- | ----- | ----------------------------------------------------------------------------------- |
| `get_dashboard_summary` | Query | Retrieve dashboard summary with configurable criteria (recent items, depot entries) |

---

## Wishlist

Wishlist management for tracking desired railway models.

| Command Name           | Type    | Description                                                   |
| ---------------------- | ------- | ------------------------------------------------------------- |
| `get_wishlists`        | Query   | Retrieve all wishlists with their preview information         |
| `get_wishlist_by_id`   | Query   | Retrieve a specific wishlist by its identifier with all items |
| `create_wishlist`      | Command | Create a new wishlist with a name and optional description    |
| `rename_wishlist`      | Command | Rename an existing wishlist                                   |
| `delete_wishlist`      | Command | Delete a wishlist by its identifier                           |
| `set_default_wishlist` | Command | Set a wishlist as the default wishlist                        |
| `add_to_wishlist`      | Command | Add a railway model item to a specific wishlist               |
| `remove_from_wishlist` | Command | Remove an item from a wishlist by item ID                     |
| `move_item_to_list`    | Command | Move a wishlist item from one wishlist to another             |

---

## Sellers

Seller and vendor management for tracking where models are purchased.

| Command Name       | Type    | Description                                              |
| ------------------ | ------- | -------------------------------------------------------- |
| `get_sellers`      | Query   | Retrieve all sellers from the database                   |
| `get_seller_by_id` | Query   | Retrieve a specific seller by its identifier             |
| `create_seller`    | Command | Create a new seller with contact and address information |
| `update_seller`    | Command | Update an existing seller's information                  |
| `delete_seller`    | Command | Delete a seller by its identifier                        |

---

## Maintenance

Maintenance tracking and scheduling for collection items.

| Command Name                | Type    | Description                                              |
| --------------------------- | ------- | -------------------------------------------------------- |
| `get_maintenance_dashboard` | Query   | Retrieve maintenance cards that are due or overdue       |
| `add_maintenance_record`    | Command | Add a maintenance record and update the maintenance card |

---

## Tracks Inventory

Track inventory management for monitoring track pieces and purchases.

| Command Name              | Type    | Description                                                |
| ------------------------- | ------- | ---------------------------------------------------------- |
| `create_track_inventory`  | Command | Create a new track inventory with a name                   |
| `rename_track_inventory`  | Command | Rename an existing track inventory                         |
| `add_track_purchase`      | Command | Add a track purchase to an inventory with items            |
| `set_track_item_quantity` | Command | Set the quantity for a specific track item in an inventory |

---

## Command Summary Statistics

- **Total Commands**: 38
- **Queries (Read Operations)**: 17
- **Commands (Write Operations)**: 21

## Architecture Notes

### CQRS Pattern

This application follows the Command Query Responsibility Segregation (CQRS) pattern:

- **Queries**: Read-only operations that return data without modifying state
- **Commands**: Operations that modify state and may or may not return data

### Tauri IPC Layer

All commands are exposed through Tauri's IPC mechanism using the `#[tauri::command]` attribute. They are type-safe with TypeScript bindings automatically generated using `specta` and `tauri-specta`.

### Error Handling

All commands return `Result<T, CommandError>` where:

- `T` is the success type (specific to each command)
- `CommandError` is a unified error type that maps domain and infrastructure errors to frontend-friendly messages

### Unit of Work Pattern

Commands use the Unit of Work pattern for transaction management:

1. Acquire a `SqliteUnitOfWork` from the application state
2. Execute domain logic within the transaction
3. Commit or rollback based on the result

This ensures data consistency and proper error handling across all operations.
