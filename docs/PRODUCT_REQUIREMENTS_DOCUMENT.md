# Product Requirements Document: Rusty Shed

## Functional Requirements Table

| #        | Title                      | User Story Description                                                                                                            | Priority | Notes                                                                        |
| -------- | -------------------------- | --------------------------------------------------------------------------------------------------------------------------------- | -------- | ---------------------------------------------------------------------------- |
| **1.1**  | **Unified Entry System**   | As a user, I can input all information for a railway model once and then choose to add it to either the Collection or a Wishlist. | **P0**   | Essential for data integrity. Includes "Rolling Stock List" as a sub-entity. |
| **1.2**  | **Collection Management**  | As a user, I can track technical specs, maintenance status, and purchase info (seller/price) for my models.                       | **P0**   | The primary inventory "Source of Truth."                                     |
| **1.3**  | **Digital Roster (DCC)**   | As a user, I can manage digital rolling stock, tracking the decoder model and the DCC address.                                    | **P0**   | Critical for operational layouts.                                            |
| **1.4**  | **Sellers Directory**      | As a user, I can manage a directory of sellers (shops, auctions, private) to track purchase origins.                              | **P1**   | Links to purchase history for financial analytics.                           |
| **1.5**  | **Cloud Backup (G-Drive)** | As a user, I want to back up my collection database to Google Drive to prevent data loss and sync across devices.                 | **P1**   | Requires OAuth2 integration in Rust. Svelte handles the "Sync Now" UI.       |
| **1.6**  | **Data Import Utility**    | As a user, I can import railway models from a file (CSV/JSON) to quickly populate my collection.                                  | **P1**   | Rust handles parsing/validation; Svelte provides the field-mapping UI.       |
| **1.7**  | **Depot View**             | As a user, I want a categorized view of all rolling stock (Locos, Passenger, Freight) for quick access.                           | **P1**   | Filterable gallery view using Tailwind Grid.                                 |
| **1.8**  | **Maintenance System**     | As a user, I can create maintenance cards and log specific events for each piece of rolling stock.                                | **P1**   | Tracks usage history and service intervals.                                  |
| **1.9**  | **Track Inventory**        | As a user, I want to manage my track inventory (count and price) for layout planning.                                             | **P2**   | Aggregates costs of turnouts, flex-track, etc.                               |
| **1.10** | **Wishlist Management**    | As a user, I can create multiple named wishlists with priorities and target prices.                                               | **P2**   | Supports [0..n] lists with drag-and-drop priority.                           |
| **1.11** | **Financial Tracking**     | As a user, I want to track past spending and define monthly/yearly budget limits.                                                 | **P2**   | Visual indicators for budget "overages."                                     |
| **1.12** | **Insights Dashboard**     | As a user, I want a dashboard with charts for spending, maintenance alerts, and highlights.                                       | **P2**   | Summary view using Svelte reactivity.                                        |
| **1.13** | **Global Preferences**     | As a user, I can manage favorites (Scale, AC/DC, Companies) and localization.                                                     | **P3**   | Provides default values for new entries.                                     |

---
