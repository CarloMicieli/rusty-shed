# Research: Digital Rolling Stock Management

**Feature**: 006-digital-rolling-stocks  
**Date**: 2026-01-30  
**Purpose**: Resolve technical unknowns and document design decisions

## Research Summary

All technical questions have been resolved through codebase analysis. No external research was required as the existing implementation provides clear patterns to follow.

---

## 1. Enriched Digital Rolling Stock View

**Question**: How to add rolling stock details (category, railway company, scale, power method, road number) to `DigitalRollingStockView`?

**Decision**: Extend the existing query in `sqlite_digital_rolling_stock_repository.rs` to JOIN with `rolling_stocks`, `railway_companies`, and `railway_models` tables.

**Rationale**:

- The `DepotRollingStockView` already demonstrates this pattern with similar fields
- `owned_rolling_stocks` links to `rolling_stocks` via `rolling_stock_id`
- `rolling_stocks` links to `railway_companies` and contains category, scale info
- `railway_models` contains power_method via the manufacturer relationship

**Alternatives Considered**:

1. Create a separate enriched view type → Rejected: duplicates query logic, increases API surface
2. Fetch in two calls (digital + catalog) → Rejected: N+1 problem, poor performance

**Implementation Notes**:

```sql
-- Extend existing find_all_digital_rolling_stocks query with:
LEFT JOIN owned_rolling_stocks ors ON drs.owned_rolling_stock_id = ors.id
LEFT JOIN rolling_stocks rs ON ors.rolling_stock_id = rs.id
LEFT JOIN railway_companies rc ON rs.railway_company_id = rc.id
LEFT JOIN railway_models rm ON rs.railway_model_id = rm.id
```

---

## 2. Digital Summary Calculation

**Question**: How to calculate the percentage of digital rolling stock, excluding dummies and counting factory-fitted (DCC_SOUND/DCC_FITTED)?

**Decision**: Create a new query use case `GetDigitalSummaryUseCase` that returns `DigitalSummary { total_non_dummy: u32, digital_count: u32, percentage: f32 }`.

**Rationale**:

- Single query for efficiency
- Count `is_dummy = false` rolling stocks as denominator
- Count `control IN ('DCC_SOUND', 'DCC_FITTED') OR digital_rolling_stock_id IS NOT NULL` as numerator
- Frontend receives pre-calculated percentage

**SQL Logic**:

```sql
SELECT
  COUNT(*) FILTER (WHERE is_dummy = false) as total_non_dummy,
  COUNT(*) FILTER (
    WHERE is_dummy = false
    AND (control IN ('DCC_SOUND', 'DCC_FITTED') OR drs.id IS NOT NULL)
  ) as digital_count
FROM owned_rolling_stocks ors
LEFT JOIN rolling_stocks rs ON ors.rolling_stock_id = rs.id
LEFT JOIN digital_rolling_stocks drs ON drs.owned_rolling_stock_id = ors.id
```

**Alternatives Considered**:

1. Calculate in frontend → Rejected: Constitution requires domain logic in Rust
2. Store pre-computed value → Rejected: stale data risk, unnecessary complexity

---

## 3. Duplicate DCC Address Detection

**Question**: How to check if a DCC address is already in use and warn the user?

**Decision**: Add `check_address_exists(address: DccAddress, exclude_id: Option<DigitalRollingStockId>) -> bool` to the repository.

**Rationale**:

- Query can exclude current item when editing (to avoid self-match)
- Returns simple boolean for frontend warning display
- Soft warning only - does not block save (per spec: "display a warning")

**Implementation Notes**:

- Called from frontend before save to display warning
- Also useful during address edit to show real-time feedback
- New Tauri command: `check_dcc_address_duplicate`

**Alternatives Considered**:

1. Check in use case and return error → Rejected: spec says soft warning, not hard block
2. Frontend maintains address list → Rejected: stale data, domain logic leakage

---

## 4. Exclude Function Decoders from Roster

**Question**: How to exclude rolling stocks with Function decoders from the main roster list?

**Decision**: Add filter clause to `find_all_digital_rolling_stocks` query: `WHERE d.decoder_type != 'FUNCTION'`.

**Rationale**:

- Function decoders are for accessories (lights, sounds on passenger cars)
- Main roster should show traction units with proper DCC control
- Filtering at query level is most efficient

**Implementation Notes**:

- Already have `DecoderType::Function` enum variant
- Decoders table has `decoder_type` column
- Simple WHERE clause addition

---

## 5. Expose Decoder List for Dropdown

**Question**: How to provide the decoder list for the installation dropdown?

**Decision**: Create new Tauri command `get_decoders` that calls existing `find_all_decoders()`.

**Rationale**:

- Repository method already exists
- Just need to wire up Tauri command handler
- Return `Vec<DecoderView>` for frontend consumption

**Implementation Notes**:

- Create `GetDecodersUseCase` (thin wrapper for consistency)
- Add command handler `get_decoders` in `command_handlers.rs`
- Frontend uses for dropdown population

---

## 6. Right-Sliding Drawer Component

**Question**: What component pattern to use for the decoder installation panel?

**Decision**: Use Skeleton UI's Drawer component with `position: right`.

**Rationale**:

- Skeleton UI provides built-in Drawer component
- Consistent with existing app patterns (spec says "from the right side")
- Handles overlay, animation, accessibility

**Implementation Notes**:

- Import from `@skeletonlabs/skeleton`
- Use drawer store for open/close state
- Form fields: rolling stock dropdown, decoder dropdown, date picker, DCC address input

**Alternatives Considered**:

1. Custom modal → Rejected: reinventing existing component
2. Navigate to new page → Rejected: spec explicitly says popup/drawer

---

## 7. Navigation Icon

**Question**: What icon to use for "My Digital Rolling Stocks" navigation item?

**Decision**: Use `Cpu` icon from lucide-svelte.

**Rationale**:

- Represents digital/electronic nature of DCC
- Distinct from other nav icons (LayoutDashboard, Library, Heart, Box)
- Available in lucide-svelte (already used in project)

**Alternatives Considered**:

1. `Zap` (lightning) → Could work but less specific
2. `Radio` → Too generic
3. `CircuitBoard` → Too technical/abstract

---

## 8. Paraglide Message Keys

**Decision**: Use consistent naming pattern for new messages.

**Keys to add**:

```json
{
  "app_digital_roster": "My Digital Rolling Stocks",
  "digital_roster_summary_title": "Digital Overview",
  "digital_roster_percentage": "{percentage}% digitalized",
  "digital_roster_empty": "No digital rolling stocks found",
  "digital_roster_filter_placeholder": "Search by DCC address or road number...",
  "digital_roster_install_decoder": "Install Decoder",
  "digital_roster_change_address": "Change DCC Address",
  "digital_roster_duplicate_warning": "Warning: DCC address {address} is already in use",
  "digital_roster_confirm_replace": "This rolling stock already has a decoder installed. Replace it?",
  "digital_roster_decoder_label": "Decoder",
  "digital_roster_rolling_stock_label": "Rolling Stock",
  "digital_roster_date_label": "Installation Date",
  "digital_roster_address_label": "DCC Address"
}
```

---

## Conclusion

All technical decisions have been made based on existing patterns in the codebase. The implementation will:

1. **Extend** existing views and queries (not replace)
2. **Add** new Tauri commands following ADR 8 patterns
3. **Create** new frontend feature module following existing structure
4. **Reuse** Skeleton UI components and Paraglide messaging

No blocking unknowns remain. Ready for Phase 1: Data Model & Contracts.
