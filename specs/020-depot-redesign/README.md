# Depot Page Redesign - Documentation Index

**Feature Branch**: `020-depot-redesign`
**Date**: 2026-02-12
**Status**: ✅ Planning Complete - Ready for Implementation

## Overview

This feature redesigns the depot page to organize rolling stock into four collapsible accordion categories with improved table view and Era column.

## Documentation Files

### Planning Documents

1. **[spec.md](./spec.md)** - Feature Specification
   - User scenarios and acceptance criteria
   - Functional requirements (FR-001 through FR-015)
   - Success criteria and assumptions
   - **Read this first** to understand what needs to be built

2. **[plan.md](./plan.md)** - Implementation Plan
   - Technical context and constitution compliance
   - Project structure and file organization
   - Phase 0 and Phase 1 planning details
   - **Read this second** for technical approach

### Research & Design (Phase 0 & 1)

3. **[research.md](./research.md)** - Research Findings
   - Depot query analysis (epoch availability)
   - Category mapping investigation
   - Accordion component usage patterns
   - Ownership/soft-delete filtering verification

4. **[data-model.md](./data-model.md)** - Data Model Design
   - Backend entity changes (DepotRollingStockView + epoch)
   - Frontend category grouping types
   - Database schema notes (no migrations needed)
   - Data flow diagrams

5. **[quickstart.md](./quickstart.md)** - Developer Guide
   - Step-by-step implementation instructions
   - Testing procedures
   - Troubleshooting tips
   - **Read this to start implementing**

### Contracts (Phase 1)

6. **[contracts/depot-view.ts](./contracts/depot-view.ts)** - TypeScript Type Contracts
   - DepotRollingStockView interface (with epoch field)
   - DepotView interface
   - RollingStockCategory enum

7. **[contracts/depot-categories.ts](./contracts/depot-categories.ts)** - Frontend Category Types
   - DepotCategoryKey type (4 UI categories)
   - CategoryGroup interface
   - CATEGORY_MAPPING configuration
   - Helper functions for grouping

## Implementation Checklist

### Backend (Rust)

- [ ] Add `pub epoch: Option<Epoch>` to `DepotRollingStockView` (depot_view.rs)
- [ ] Update mapper to include epoch (mappers.rs)
- [ ] Rebuild backend (`cargo build`)
- [ ] Run tests (`cargo test`)
- [ ] Run clippy (`cargo clippy`)

### Frontend (TypeScript/Svelte)

- [ ] Add 6 new i18n message keys (Paraglide)
- [ ] Update DepotState.svelte.ts (4 category groups)
- [ ] Update +page.svelte (Accordion instead of grid/table toggle)
- [ ] Update DepotTable.svelte (add Era column)
- [ ] Remove grid view components (LocomotiveCard, TrainCard, CarCard)
- [ ] Run linters (`pnpm lint`, `pnpm check`)
- [ ] Run tests (`pnpm test`)

### Testing

- [ ] Manual UI testing with 4 categories
- [ ] Test search filtering across categories
- [ ] Test accordion expand/collapse
- [ ] Test sticky headers
- [ ] Test with 100+ items (performance)
- [ ] Verify ownership filtering
- [ ] Verify soft-delete filtering
- [ ] Verify duplicates shown

## Key Changes Summary

| Change                                 | Files Affected                     | Complexity |
| -------------------------------------- | ---------------------------------- | ---------- |
| **Add epoch field**                    | depot_view.rs, mappers.rs          | Low        |
| **Reorganize categories (3→4)**        | DepotState.svelte.ts, +page.svelte | Medium     |
| **Replace grid toggle with Accordion** | +page.svelte                       | Medium     |
| **Add Era column to table**            | DepotTable.svelte                  | Low        |
| **Add i18n messages**                  | Paraglide messages                 | Low        |
| **Remove grid view components**        | 3 component files                  | Low        |

## Architecture Decisions

### Why 4 categories?

Splits "Cars" into "Passenger Cars" and "Freight Cars" for better organization of large collections.

### Why Accordion instead of tabs?

- Allows viewing multiple categories simultaneously
- Sticky headers for better navigation
- Better for large collections (500+ items)

### Why add epoch to backend?

- Epoch is already in database (railway_models table)
- No SQL changes needed - just mapper update
- Provides consistent data in depot view

### Why remove grid view?

- Spec requires table-first design
- Reduces code complexity
- Accordion provides better organization than grid

## Next Steps

1. ✅ **Planning Complete** - All design artifacts generated
2. ⏭️ **Task Generation** - Run `/speckit.tasks` to create task breakdown
3. ⏭️ **Implementation** - Follow quickstart.md for step-by-step guide
4. ⏭️ **Testing** - Manual and automated test execution
5. ⏭️ **Review** - Code review and approval
6. ⏭️ **Merge** - Merge to main after approval

## Questions or Issues?

- **Requirements unclear?** → Check [spec.md](./spec.md)
- **Technical approach unclear?** → Check [plan.md](./plan.md) and [research.md](./research.md)
- **How do I implement?** → Check [quickstart.md](./quickstart.md)
- **Type definitions needed?** → Check [contracts/](./contracts/)

---

**Ready to implement?** Start with [quickstart.md](./quickstart.md)
