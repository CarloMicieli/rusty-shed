# Research: Centralized Entity Management (041)

**Date**: 2026-05-17  
**Status**: Complete

## Decision 1: 041 is a follow-up to 040, not a prerequisite

**Decision**: Treat feature 041 as an extension over the already-delivered quick-add foundation from feature 040.

**Rationale**:
- The repository already includes quick-add create flows and shared quick-add form behavior.
- The Settings Library now extends that form contract with `mode=FULL` while preserving `mode=QUICK` compatibility.

**Alternatives considered**:
- Keep 041 as prerequisite for 040.
- Rejected because implementation history and current code state show 040 has already shipped core quick-add capabilities.

## Decision 2: Buyer and Seller are distinct aggregates over one shared table

**Decision**: Keep Buyers and Sellers as distinct application/domain command surfaces while persisting both in the same underlying table.

**Rationale**:
- Domain intent remains clear by context (`buyer` vs `seller`) without duplicating storage.
- Prevents model divergence for the same party acting in both roles.
- Matches clarified requirement to avoid separate data-model split for shops acting in both contexts.

**Alternatives considered**:
- Separate `buyers` table.
- Rejected due to duplication risk and unnecessary migration complexity.

## Decision 3: Canonical shared party record appears in both tabs immediately

**Decision**: Buyers and Sellers tabs show the same canonical shared party records with role-context labeling.

**Rationale**:
- Ensures edits from either tab propagate consistently.
- Eliminates stale data between tab projections.
- Supports creation from either tab with immediate visibility in both tabs.

**Alternatives considered**:
- Separate per-tab projections.
- Rejected due to synchronization complexity and user confusion.

## Decision 4: Protection and locking are enforced by total usage across both contexts

**Decision**: For shared buyer/seller records, `usage_count` and delete eligibility use total references across buyer and seller contexts.

**Rationale**:
- Prevents deletion when a record is still referenced in the opposite role context.
- Aligns UI lock indicators with backend safety checks.

**Alternatives considered**:
- Context-only usage counts by active tab.
- Rejected because it allows unsafe deletion paths for still-referenced records.

## Decision 5: Merge for shared buyer/seller records is canonical and atomic

**Decision**: Merge operates on canonical party records and relinks all buyer and seller references in one transaction before deleting the source record.

**Rationale**:
- Ensures referential integrity and avoids partial merge outcomes.
- Keeps merge semantics identical regardless of entry tab.

**Alternatives considered**:
- Tab-scoped merge (only buyer refs or only seller refs).
- Rejected because it can leave split references and ambiguous post-merge state.

## Decision 6: Distinct Buyer/Seller command contracts over shared repository logic

**Decision**: Expose distinct backend command entry points (`create_buyer`, `create_seller`, etc.) even when implementation shares repository and table logic.

**Rationale**:
- Preserves explicit API semantics and auditability by use case.
- Allows future policy divergence without breaking the persistence model.

**Alternatives considered**:
- Single generic `party` command surface.
- Rejected due to weaker domain clarity and harder role-specific evolution.

## Decision 7: Library contract style

**Decision**: Define a REST-shaped OpenAPI contract in planning artifacts to drive implementation and tests, while actual runtime transport remains Tauri IPC commands with specta-generated TypeScript bindings.

**Rationale**:
- OpenAPI contract is concise for behavior specification and test planning.
- Maintains constitution requirement that runtime boundary is typed IPC in this codebase.

**Alternatives considered**:
- GraphQL schema contract.
- Rejected because current app architecture and tooling are IPC command oriented, not GraphQL.
