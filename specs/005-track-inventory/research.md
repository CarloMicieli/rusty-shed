# Research: Track Inventory Feature

**Feature**: 005-track-inventory  
**Created**: 2026-01-30

## Rust Backend Changes

### 1. Add `track_type` to `TrackProduct`

**Decision**: Add `track_type: TrackType` field to the `TrackProduct` struct.

**Rationale**: The `TrackType` enum (Straight, Curve, Turnout, FlexTrack) classifies track pieces by their geometric type. This is distinct from `TrackCode` which describes the rail profile (Code70, Code75, etc.). Both are needed for proper track management.

**Implementation**:

- Add `track_type: TrackType` field to `TrackProduct` struct in `domain/track_product.rs`
- Create migration `0007_add_track_type_to_products.sql` to add column
- Update `TrackProductRow` in `infrastructure/entities.rs`
- Update repository queries in `sqlite_track_product_repository.rs`

**Alternatives considered**:

- Deriving track type from product code: Rejected - not reliable across manufacturers
- Making it optional: Rejected - track type is a fundamental characteristic

### 2. Query Handlers with View Structs

**Decision**: Create query handlers that return View structs defined in the application module.

**Rationale**: Following CQRS pattern, queries return read-optimized View structs separate from domain aggregates. Defining Views in the application layer (rather than domain) keeps the domain pure and allows query-specific projections.

**View Structs to Create** (in `application/views.rs`):

```rust
/// Summary view of a track inventory for list display
pub struct TrackInventoryListItem {
    pub id: TrackInventoryId,
    pub name: String,
    pub description: Option<String>,
    pub total_items: i64,      // Count of distinct track types
    pub total_quantity: i64,   // Sum of all quantities
}

/// Detailed view of a track inventory with items
pub struct TrackInventoryView {
    pub id: TrackInventoryId,
    pub name: String,
    pub description: Option<String>,
    pub items: Vec<TrackInventoryItemView>,
    pub purchases: Vec<TrackPurchaseView>,
}

/// View of a single inventory item (track + quantity)
pub struct TrackInventoryItemView {
    pub track_id: TrackId,
    pub track_product: TrackProductView,
    pub quantity: i64,
    pub required: i64,         // Required quantity for planning
}

/// Track product view for display
pub struct TrackProductView {
    pub track_id: TrackId,
    pub manufacturer_name: String,
    pub product_code: String,
    pub description: String,
    pub track_type: TrackType,
    pub track_code: TrackCode,
    pub with_roadbed: bool,
    pub length: Option<Length>,
    pub radius: Option<Length>,
}

/// Purchase history view
pub struct TrackPurchaseView {
    pub id: TrackPurchaseId,
    pub track_product: TrackProductView,
    pub quantity: i64,
    pub price: MonetaryAmount,
    pub seller_name: Option<String>,
    pub purchase_date: NaiveDate,
}
```

**Query Handlers to Create**:

| Command                 | Returns                       | Description                               |
| ----------------------- | ----------------------------- | ----------------------------------------- |
| `get_track_inventories` | `Vec<TrackInventoryListItem>` | List all inventories                      |
| `get_track_inventory`   | `TrackInventoryView`          | Single inventory with items and purchases |
| `get_track_products`    | `Vec<TrackProductView>`       | List all track products                   |

## Frontend Patterns

### 1. State Controller Pattern

**Decision**: Use `TrackInventoryService.svelte.ts` class with Svelte 5 runes.

**Rationale**: Follows established patterns in `CollectionService.svelte.ts`, `WishlistService.svelte.ts`.

**Pattern**:

```typescript
export class TrackInventoryService {
  // Private reactive state
  #inventories = $state<TrackInventoryListItem[]>([]);
  #selectedInventory = $state<TrackInventoryView | null>(null);
  #isLoading = $state(false);

  // Public getters
  get inventories() { return this.#inventories; }
  get selectedInventory() { return this.#selectedInventory; }
  get isLoading() { return this.#isLoading; }

  // Derived state
  totalCount = $derived(this.#inventories.length);

  // Use cases
  async fetchInventories(): Promise<void> { ... }
  async fetchInventory(id: string): Promise<void> { ... }
  async createInventory(input: NewTrackInventoryArgs): Promise<string | null> { ... }
  async addPurchase(input: AddTrackPurchaseArgs): Promise<string | null> { ... }
}
```

### 2. Context Provider Pattern

**Decision**: Use Svelte context for dependency injection.

**Rationale**: Follows established pattern with `setContext`/`getContext` for service access.

### 3. Navigation Integration

**Decision**: Add "My Tracks" between "My Wishlists" and "My Depot" in navigation.

**Rationale**: Logical grouping - tracks are a collectible category alongside rolling stock.

**Icon**: Use `RailSymbol` or `Tram` from lucide-svelte (or similar track-themed icon).

### 4. Localization

**Decision**: Add message keys with `track_` prefix.

**Required Keys**:

- `app_tracks`: "My Tracks" / "I Miei Binari"
- `track_inventory_title`: "Track Inventories" / "Inventari Binari"
- `track_inventory_empty_*`: Empty state messages
- `track_purchase_*`: Purchase-related strings
- `track_product_*`: Product-related strings

## Database Migration

### 0007_add_track_type_to_products.sql

```sql
ALTER TABLE track_products ADD COLUMN track_type TEXT;
-- Default existing rows to 'STRAIGHT'
UPDATE track_products SET track_type = 'STRAIGHT' WHERE track_type IS NULL;
```

**Note**: Column is nullable initially to support migration of existing data.

## Dependencies

### Lucide Icon Selection

**Decision**: Use `Tram` icon for track inventory navigation.

**Alternatives considered**:

- `RailSymbol`: Not available in lucide-svelte
- `Train`: Already used for app logo
- `Tram`: Available and suggests rail transport

## Open Questions (Resolved)

1. ✅ **Track type vs Track code**: Both are needed - type is geometry (straight/curve), code is rail profile (70/75/83/100)
2. ✅ **Required quantity tracking**: Will be stored in `track_inventory_items` table - needs schema update in future migration
3. ✅ **View struct location**: Application module per user request
