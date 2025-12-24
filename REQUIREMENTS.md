# Requirements — Rusty Shed (Model Railway Collection Manager)

## Summary

Rusty Shed is a single-user collection manager for model railway items. The app stores one user collection, supports zero-to-many wish lists, and tracks maintenance and service history for individual items. 

Key design choices:
- Primary identifiers: UUID for all entities.
- Prices: canonical numeric value + ISO currency code (e.g., `35.00 EUR`).
- Measures: numeric value + unit; `length` uses millimetres (`mm`).

## Main use cases / scenarios

1. Add a new collection item (RailwayModel) with optional purchase info and one or more RollingStock components.
2. Browse and search the collection by brand, scale, epoch, category, livery, depot, road number, and DCC capability.
3. Edit an RailwayModel or any nested RollingStock entry (change quantity, livery, depot, service status).
4. Maintain multiple wish lists: create, add items (by `itemNumber` or search), remove, and reorder entries.
5. Record maintenance events for a RollingStock (date, description, cost, performed-by, next-due date).
7. Generate simple reports and filters: total collection value, items by epoch/scale, DCC-capable items, maintenance history per item.
8. Backup and restore the entire collection data (local-first, exportable file).

## Domain model dictionary

All entities use a UUID `id` as primary key.

- RailwayModel
  - id: UUID (primary)
  - itemNumber: string | null — vendor or catalog number (example: `40152`)
  - brand: string (example: `ACME`)
  - description: string (example: "Bagagliaio FS ad assi tipo Dm 98 per treni merci")
  - scale: string (example: `H0`)
  - powerMethod: enum(string) (example values: `DC`, `AC`, `NONE`, etc.)
  - deliveryDate: string | null — year or ISO date (store as ISO date if full date known; allow year-only as fallback)
  - count: integer — quantity owned (example: `3`)
  - purchaseInfo: PurchaseInfo | null
  - rollingStocks: RollingStock[] — one or more components (composition)
  - notes: string | null

- RollingStock
  - id: UUID (primary)
  - railwayModelId: UUID (reference to parent RailwayModel)
  - roadNumber: string | null (example: `21 83 166 5 155-1 Ghks-w`)
  - typeName: string (example: `Ghkrs`)
  - series: string | null (example: `20500`)
  - railway: string | null (example: `FS`)
  - epoch: string | null (example values found: `IV`, `V`, etc.)
  - category: string | null (example values found: `FREIGHT_CAR`, `LOCOMOTIVE`, `PASSENGER_CAR`)
  - subCategory: string | null (example: `RAILWAY_POST_OFFICE`)
  - depot: string | null (example: `Cuneo`) — free-form but can be normalized later
  - length: number | null + unit (`mm`) (example: `133` -> interpreted as `133 mm`)
  - livery: string | null (example: `grigio ardesia`)
  - serviceLevel: string | null (example: `2cl`)
  - control: enum(string) | null (example values: `DCC_READY`, `DCC_DECODER_INSTALLED`, etc.)
  - dccInterface: string | null (example: `MTC_21`)

- PurchaseInfo
  - id: UUID (primary)
  - railwayModelId: UUID (reference)
  - date: ISO date | null (example: `2011-03-19`)
  - price: object { value: number, currency: string } (example: `{ value: 35.00, currency: "EUR" }`)
  - shop: string | null (example: `Treni&Treni`)

- WishList
  - id: UUID
  - name: string
  - description: string | null
  - entries: WishListEntry[] (ordered list)

- WishListEntry
  - id: UUID
  - wishlistId: UUID
  - referencedItemNumber: string | null (catalog number to map to an RailwayModel)
  - note: string | null
  - priority: integer | null

- MaintenanceEvent
  - id: UUID
  - rollingStockId: UUID
  - date: ISO date
  - description: string
  - cost: object { value: number, currency: string } | null
  - performedBy: string | null
  - nextDue: ISO date | null

## Field types, formats & validation rules

- Identifiers
  - `id`: UUID v4 for all persisted entities.
  - `itemNumber`: optional, stored as string; not used as primary key.

- Dates
  - Use ISO 8601 date strings (`YYYY-MM-DD`) where full date is known. Allow year-only strings when only year is available but prefer canonical ISO where possible.

- Price
  - Store price as an object: `{ value: number, currency: string }` using numeric value (decimal) and ISO 4217 currency code (example: `35.00 EUR`).

- Measures
  - `length` and other measures are stored as numeric values with an explicit unit. Default for `length` is millimetres (`mm`). Example: `length.value = 133`, `length.unit = "mm"`.

- Required vs optional (minimum viable set)
  - RailwayModel: `id`, `brand`, `description`, `scale`, `rollingStocks` (length >= 1), `count` (defaults to 1) — required.
  - RollingStock: `id`, `typeName` — required; most other fields optional.
  - PurchaseInfo: optional.
  - WishList: `id`, `name`, `entries` (may be empty).

## Non-functional requirements (brief)

- Single-user local-first storage; allow manual export for backups and migration.
- Responsive UI able to handle thousands of items; indexing for search filters (brand, scale, epoch, category).
- Data integrity: use UUIDs, strict price and measure canonicalization to avoid locale-dependent parsing issues.
