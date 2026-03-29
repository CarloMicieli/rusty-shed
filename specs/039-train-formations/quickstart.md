# Quickstart: Train Formations

**Branch**: `039-train-formations` | **Phase**: 1 | **Date**: 2026-03-29

This guide describes the complete implementation path for the Train Formations feature. Follow these phases in order. Validation gates are listed at each phase boundary.

---

## Prerequisites

- Branch `039-train-formations` checked out.
- `pnpm install` and `cargo build` pass cleanly on main.
- Read: `specs/039-train-formations/spec.md`, `research.md`, `data-model.md`, `contracts/tauri-ipc.md`.

---

## Phase A — Backend: Database & Domain Layer

### A1. Migration

Create `src-tauri/migrations/0009_create_train_formations_schema.sql` using the SQL in `data-model.md`.

Verify: `sqlx migrate run` succeeds. Check tables exist in SQLite with correct columns and indexes.

### A2. Domain Entities (Rust)

Create module `src-tauri/src/trains/` following the hexagonal pattern used in `src-tauri/src/catalog/`.

```
src-tauri/src/trains/
├── mod.rs
├── domain/
│   ├── mod.rs
│   ├── prototype/
│   │   ├── mod.rs
│   │   ├── prototype.rs              # Prototype aggregate
│   │   └── repositories.rs          # PrototypeRepository trait
│   ├── formation/
│   │   ├── mod.rs
│   │   ├── train_formation.rs        # TrainFormation aggregate
│   │   ├── train_formation_event.rs  # TrainFormationEvent enum (separate file)
│   │   ├── formation_element.rs      # FormationElement value object
│   │   └── repositories.rs          # TrainFormationRepository trait
│   └── formation_category/
│       ├── mod.rs
│       └── formation_category.rs
├── application/
│   ├── mod.rs
│   ├── get_train_formations.rs
│   ├── get_train_formation.rs
│   ├── create_train_formation.rs
│   ├── update_train_formation.rs
│   ├── delete_train_formation.rs
│   ├── add_formation_element.rs
│   ├── remove_formation_element.rs
│   ├── reorder_formation_elements.rs
│   ├── assign_rolling_stock_to_element.rs
│   ├── set_traction_override.rs
│   ├── get_prototypes.rs
│   ├── create_custom_prototype.rs
│   ├── get_formation_categories.rs
│   └── create_formation_category.rs
├── infrastructure/
│   ├── mod.rs
│   ├── entities.rs              # SQLx row structs
│   ├── mappers.rs               # Row → Domain → View
│   ├── train_formation_repo.rs  # SQLx impl of TrainFormationRepository
│   ├── prototype_repo.rs        # SQLx impl of PrototypeRepository
│   └── seed_data.rs             # Prototype + category seed insertion
└── interface/
    ├── mod.rs
    ├── command_args.rs          # All *Args structs (specta + garde)
    └── command_handlers.rs      # #[tauri::command] functions
```

### A2a. `TrainFormationEvent` (in `train_formation_event.rs`)

Follow the `WishlistEvent` pattern: separate file, `#[serde(tag = "type", content = "payload", rename_all = "camelCase")]`.

```rust
use serde::{Deserialize, Serialize};
use crate::trains::domain::formation::formation_element::FormationElement;

/// Domain events emitted by the `TrainFormation` aggregate.
///
/// Events are serialisable, immutable records of state changes. Repositories
/// drain `pending_events` via `take_events()` after each mutation and handle
/// side-effects (e.g. position-order updates) in the same transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum TrainFormationEvent {
    Created {
        id: String,
        name: String,
    },
    Renamed {
        name: String,
    },
    MetadataUpdated {
        category_id: Option<String>,
        start_year: Option<i32>,
        end_year: Option<i32>,
        epoch: Option<String>,
        notes: Option<String>,
    },
    Deleted {
        id: String,
    },
    ElementAdded {
        element: FormationElement,
    },
    ElementRemoved {
        element_id: String,
    },
    ElementsReordered {
        ordered_element_ids: Vec<String>,
    },
    RollingStockAssigned {
        element_id: String,
        owned_rolling_stock_id: String,
    },
    RollingStockUnassigned {
        element_id: String,
    },
    TractionOverrideSet {
        element_id: String,
        traction_override: i32,
    },
}
```

### A2b. `TrainFormation` aggregate (in `train_formation.rs`)

Follow the `Collection` / `MaintenanceCard` pattern: constructor emits `Created`, every mutating method emits an event + calls `apply_event()`, `take_events()` drains `pending_events`.

```rust
use crate::core::domain::Metadata;
use crate::trains::domain::formation::formation_element::FormationElement;
use crate::trains::domain::formation::train_formation_event::TrainFormationEvent;
use crate::core::domain::DomainError;

/// Aggregate root for a train formation (a named, ordered consist).
pub struct TrainFormation {
    pub id: String,
    pub name: String,
    pub category_id: Option<String>,
    pub start_year: Option<i32>,
    pub end_year: Option<i32>,
    pub epoch: Option<String>,
    pub notes: Option<String>,
    /// Ordered composition slots.
    pub elements: Vec<FormationElement>,
    /// Unpersisted events; drained by the repository after each operation.
    pub pending_events: Vec<TrainFormationEvent>,
    pub metadata: Metadata,
}

impl TrainFormation {
    /// Construct a new formation and emit a `Created` event.
    pub fn create(id: String, name: String) -> Result<Self, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::Validation("name must not be empty".into()));
        }
        let mut formation = TrainFormation {
            id: id.clone(),
            name: name.clone(),
            category_id: None,
            start_year: None,
            end_year: None,
            epoch: None,
            notes: None,
            elements: Vec::new(),
            pending_events: Vec::new(),
            metadata: Metadata::default(),
        };
        formation.pending_events.push(TrainFormationEvent::Created { id, name });
        Ok(formation)
    }

    /// Rename the formation.
    pub fn rename(&mut self, name: String) -> Result<(), DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::Validation("name must not be empty".into()));
        }
        let ev = TrainFormationEvent::Renamed { name };
        self.pending_events.push(ev.clone());
        self.apply_event(&ev);
        Ok(())
    }

    /// Update optional metadata fields (epoch, years, notes, category).
    /// Validates year range if both years are provided.
    pub fn update_metadata(
        &mut self,
        category_id: Option<String>,
        start_year: Option<i32>,
        end_year: Option<i32>,
        epoch: Option<String>,
        notes: Option<String>,
    ) -> Result<(), DomainError> {
        if let (Some(s), Some(e)) = (start_year, end_year) {
            if s > e {
                return Err(DomainError::BusinessRule(
                    "start_year cannot exceed end_year".into(),
                ));
            }
        }
        let ev = TrainFormationEvent::MetadataUpdated {
            category_id,
            start_year,
            end_year,
            epoch,
            notes,
        };
        self.pending_events.push(ev.clone());
        self.apply_event(&ev);
        Ok(())
    }

    /// Append a new element slot to the composition.
    pub fn add_element(&mut self, element: FormationElement) {
        let ev = TrainFormationEvent::ElementAdded { element };
        self.pending_events.push(ev.clone());
        self.apply_event(&ev);
    }

    /// Remove an element slot by ID. Returns `NotFound` if absent.
    pub fn remove_element(&mut self, element_id: &str) -> Result<(), DomainError> {
        if !self.elements.iter().any(|e| e.id == element_id) {
            return Err(DomainError::NotFound {
                resource: "FormationElement".into(),
                identifier: element_id.into(),
            });
        }
        let ev = TrainFormationEvent::ElementRemoved { element_id: element_id.into() };
        self.pending_events.push(ev.clone());
        self.apply_event(&ev);
        Ok(())
    }

    /// Validate and record a reorder intent. The actual `position_order` DB
    /// update is performed atomically by the repository.
    ///
    /// Returns `ElementIdsMismatch` if the supplied list doesn't match the
    /// current element set (count or contents).
    pub fn reorder_elements(&mut self, ordered_ids: Vec<String>) -> Result<(), DomainError> {
        let current_ids: std::collections::HashSet<_> =
            self.elements.iter().map(|e| e.id.as_str()).collect();
        let supplied_ids: std::collections::HashSet<_> =
            ordered_ids.iter().map(String::as_str).collect();
        if current_ids != supplied_ids {
            return Err(DomainError::BusinessRule(
                "ordered_element_ids must contain exactly the current element set".into(),
            ));
        }
        let ev = TrainFormationEvent::ElementsReordered {
            ordered_element_ids: ordered_ids,
        };
        self.pending_events.push(ev.clone());
        self.apply_event(&ev);
        Ok(())
    }

    /// Assign a physical model to an element slot.
    pub fn assign_rolling_stock(
        &mut self,
        element_id: &str,
        owned_rolling_stock_id: String,
    ) -> Result<(), DomainError> {
        if !self.elements.iter().any(|e| e.id == element_id) {
            return Err(DomainError::NotFound {
                resource: "FormationElement".into(),
                identifier: element_id.into(),
            });
        }
        let ev = TrainFormationEvent::RollingStockAssigned {
            element_id: element_id.into(),
            owned_rolling_stock_id,
        };
        self.pending_events.push(ev.clone());
        self.apply_event(&ev);
        Ok(())
    }

    /// Unassign the physical model from an element slot.
    pub fn unassign_rolling_stock(&mut self, element_id: &str) -> Result<(), DomainError> {
        if !self.elements.iter().any(|e| e.id == element_id) {
            return Err(DomainError::NotFound {
                resource: "FormationElement".into(),
                identifier: element_id.into(),
            });
        }
        let ev = TrainFormationEvent::RollingStockUnassigned { element_id: element_id.into() };
        self.pending_events.push(ev.clone());
        self.apply_event(&ev);
        Ok(())
    }

    /// Set the per-slot traction override (`0`, `1`, or `-1`).
    pub fn set_traction_override(
        &mut self,
        element_id: &str,
        traction_override: i32,
    ) -> Result<(), DomainError> {
        if ![-1, 0, 1].contains(&traction_override) {
            return Err(DomainError::BusinessRule(
                "traction_override must be -1, 0, or 1".into(),
            ));
        }
        if !self.elements.iter().any(|e| e.id == element_id) {
            return Err(DomainError::NotFound {
                resource: "FormationElement".into(),
                identifier: element_id.into(),
            });
        }
        let ev = TrainFormationEvent::TractionOverrideSet {
            element_id: element_id.into(),
            traction_override,
        };
        self.pending_events.push(ev.clone());
        self.apply_event(&ev);
        Ok(())
    }

    /// Drain pending events. Called by the repository after persisting changes.
    pub fn take_events(&mut self) -> Vec<TrainFormationEvent> {
        std::mem::take(&mut self.pending_events)
    }

    // ── private ──────────────────────────────────────────────────────────────

    fn apply_event(&mut self, event: &TrainFormationEvent) {
        match event {
            TrainFormationEvent::Created { name, .. } => {
                self.name = name.clone();
            }
            TrainFormationEvent::Renamed { name } => {
                self.name = name.clone();
            }
            TrainFormationEvent::MetadataUpdated {
                category_id, start_year, end_year, epoch, notes,
            } => {
                self.category_id = category_id.clone();
                self.start_year = *start_year;
                self.end_year = *end_year;
                self.epoch = epoch.clone();
                self.notes = notes.clone();
            }
            TrainFormationEvent::ElementAdded { element } => {
                self.elements.push(element.clone());
            }
            TrainFormationEvent::ElementRemoved { element_id } => {
                self.elements.retain(|e| e.id != *element_id);
            }
            TrainFormationEvent::ElementsReordered { ordered_element_ids } => {
                // Re-sort the in-memory slice to match supplied order.
                let order_map: std::collections::HashMap<_, _> = ordered_element_ids
                    .iter()
                    .enumerate()
                    .map(|(i, id)| (id.as_str(), i as i32))
                    .collect();
                self.elements.sort_by_key(|e| order_map.get(e.id.as_str()).copied().unwrap_or(i32::MAX));
            }
            TrainFormationEvent::RollingStockAssigned { element_id, owned_rolling_stock_id } => {
                if let Some(el) = self.elements.iter_mut().find(|e| e.id == *element_id) {
                    el.owned_rolling_stock_id = Some(owned_rolling_stock_id.clone());
                }
            }
            TrainFormationEvent::RollingStockUnassigned { element_id } => {
                if let Some(el) = self.elements.iter_mut().find(|e| e.id == *element_id) {
                    el.owned_rolling_stock_id = None;
                }
            }
            TrainFormationEvent::TractionOverrideSet { element_id, traction_override } => {
                if let Some(el) = self.elements.iter_mut().find(|e| e.id == *element_id) {
                    el.traction_override = *traction_override;
                }
            }
            TrainFormationEvent::Deleted { .. } => {}
        }
    }
}
```

**Key aggregate invariants** (enforced in the methods above):

- `name` must be non-empty (checked in `create()` and `rename()`).
- `start_year ≤ end_year` when both are `Some` (checked in `update_metadata()`).
- `traction_override` must be one of `{-1, 0, 1}` (checked in `set_traction_override()`).
- `reorder_elements()` rejects lists where the ID set doesn't exactly match the current elements.
- All element-targeting methods return `DomainError::NotFound` if the element ID is absent.

**Repository contract** (`TrainFormationRepository` trait in `repositories.rs`):

- After calling any mutating method, the repository must call `formation.take_events()` to drain `pending_events` and handle side-effects (e.g. bulk `position_order` update in a single transaction for `ElementsReordered`).
- The repository is responsible for persisting derived state; the aggregate only asserts business rules and records intent.

In `seed_data.rs`, implement `insert_default_prototypes()` called during app startup (after migrations). Use `INSERT OR IGNORE` so re-runs are idempotent.

Include minimum viable seed (see `data-model.md` Seed Data section). Use FS, SBB, DB, SNCF entries as the initial set.

### A4. Register Commands

In `src-tauri/src/lib.rs` (or equivalent command registration file), register all 14 commands from `contracts/tauri-ipc.md`.

**Gate A**: `cargo test` passes. `cargo clippy` emits zero warnings. `cargo fmt --check` passes.

---

## Phase B — Backend: Specta Bindings

Run `pnpm tauri dev` once to trigger `tauri-specta` type generation and update `src/lib/bindings.ts`.

Verify `bindings.ts` contains all types listed in `contracts/tauri-ipc.md` (view models + Args structs).

**Gate B**: `pnpm check` passes with no TypeScript errors related to bindings.

---

## Phase C — Frontend: Feature Module

### C1. Route

Create `src/routes/train-formations/+page.svelte` (list/overview page) and `src/routes/train-formations/[id]/+page.svelte` (formation detail / builder).

### C2. Feature Module

Create `src/lib/features/train-formations/` following the structure of `src/lib/features/collection/`:

```
src/lib/features/train-formations/
├── index.ts
├── TrainFormationState.svelte.ts     # Svelte 5 class using $state, $derived
├── components/
│   ├── FormationList.svelte          # List of formation cards
│   ├── FormationCard.svelte          # Summary card
│   ├── FormationBuilder.svelte       # Full builder view (host component)
│   ├── IdentityCard.svelte           # Sticky left column
│   ├── FormationTrack.svelte         # Horizontal scroll + dndzone
│   ├── FormationCell.svelte          # Individual slot cell
│   ├── TractionWarning.svelte        # Warning icon/badge
│   ├── AddStockDrawer.svelte         # Side drawer (search prototypes)
│   ├── PrototypeSearchResults.svelte # Grouped search results
│   ├── OwnershipBadge.svelte         # "N owned" visual badge
│   ├── AssignModelPicker.svelte      # Modal to pick specific owned model
│   ├── FormationForm.svelte          # Create/Edit metadata form
│   ├── CreatePrototypeForm.svelte    # Inline custom prototype creation
│   └── icons/
│       ├── Locomotive.svelte         # SVG icon — cab + hood + bogies
│       ├── Coach.svelte              # SVG icon — elongated with window row
│       ├── Wagon.svelte              # SVG icon — freight, boxy / open-top
│       └── PrototypeIcon.svelte      # Dispatcher: maps car_type → icon + ownership styling
├── domain/
│   └── traction.ts                   # isTractionSlot() pure function
├── services/
│   └── formations.service.ts        # safeInvoke wrappers
└── types/
    └── index.ts                      # Re-exports from bindings
```

### C3. Icon Components

Create `src/lib/features/train-formations/components/icons/`. All icons use `stroke="currentColor"` so they inherit Tailwind text-color utilities from their parent (`text-blue-600`, `text-gray-400`, etc.) — zero extra CSS.

#### `Locomotive.svelte`

Focuses on the "power" look: cab, hood, two bogies.

```svelte
<script lang="ts">
  let { size = 'size-8', class: className = '' } = $props();
</script>

<svg
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="1.5"
  stroke-linecap="round"
  stroke-linejoin="round"
  class="{size} {className}"
>
  <path d="M2 17h20v-4H2v4Z" />
  <path d="M4 13V7h8v6" />
  <path d="M12 9h10v4" />
  <rect x="6" y="9" width="4" height="2" />
  <circle cx="6" cy="18" r="1.5" />
  <circle cx="18" cy="18" r="1.5" />
</svg>
```

#### `Coach.svelte`

Elongated with window row — passenger service.

```svelte
<script lang="ts">
  let { size = 'size-8', class: className = '' } = $props();
</script>

<svg
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="1.5"
  class="{size} {className}"
>
  <rect x="2" y="8" width="20" height="9" rx="1" />
  <rect x="5" y="10" width="2" height="3" />
  <rect x="9" y="10" width="2" height="3" />
  <rect x="13" y="10" width="2" height="3" />
  <rect x="17" y="10" width="2" height="3" />
  <path d="M5 18a1 1 0 1 0 0-2 1 1 0 0 0 0 2Z" />
  <path d="M19 18a1 1 0 1 0 0-2 1 1 0 0 0 0 2Z" />
</svg>
```

#### `Wagon.svelte`

Low-profile boxy freight car.

```svelte
<script lang="ts">
  let { size = 'size-8', class: className = '' } = $props();
</script>

<svg
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="1.5"
  class="{size} {className}"
>
  <path d="M2 16h20" />
  <rect x="3" y="9" width="18" height="7" />
  <path d="M8 9v7M16 9v7" />
  <circle cx="6" cy="17" r="1" />
  <circle cx="18" cy="17" r="1" />
</svg>
```

#### `PrototypeIcon.svelte` — unified dispatcher

Maps `Prototype.car_type` → icon component and applies ownership styling. The `isOwned` prop drives colour: blue tint when owned, grey/dashed when planned.

```svelte
<script lang="ts">
  import Locomotive from './Locomotive.svelte';
  import Coach from './Coach.svelte';
  import Wagon from './Wagon.svelte';
  import type { Component } from 'svelte';

  let {
    type,
    isOwned = false,
    class: className = ''
  }: { type: string; isOwned?: boolean; class?: string } = $props();

  const iconMap: Record<string, Component> = {
    Locomotive,
    PowerCar: Locomotive,
    Coach,
    Couchette: Coach,
    Dining: Coach,
    Sleeping: Coach,
    ControlCar: Coach,
    BaggageCar: Wagon,
    FreightWagon: Wagon
  } as const;

  const SelectedIcon = $derived(iconMap[type] ?? Wagon);
</script>

<div
  class="relative flex items-center justify-center p-1
    {isOwned ? 'text-blue-600' : 'text-gray-400 opacity-60'} {className}"
>
  <SelectedIcon size="size-10" />

  {#if !isOwned}
    <div class="absolute inset-0 rounded-sm border border-dashed border-gray-300"></div>
  {/if}
</div>
```

**Usage in `FormationCell.svelte`**:

```svelte
<PrototypeIcon
  type={element.prototype.car_type}
  isOwned={element.owned_rolling_stock_id !== null}
/>
<span class="mt-1 font-mono text-[10px]">{element.prototype.series_code}</span>
```

**Key constraints**:

- All icon props use Svelte 5 `$props()` rune — no legacy `export let`.
- `stroke="currentColor"` is mandatory — enables Tailwind color utilities on parent.
- `PrototypeIcon` uses `$derived` for the component selection — reactive to `type` prop changes.
- The `iconMap` covers all 9 `car_type` enum values from `data-model.md` (unknown types fall back to `Wagon`).
- Do **not** add new `car_type` values to the map without updating the `data-model.md` enum.
- Railway company badge overlay (e.g., `FS`, `SBB` text) is `absolute` positioned top-left inside the `relative` wrapper — implement in `FormationCell.svelte`, not inside `PrototypeIcon`.

### C4. Navigation

In `src/lib/components/navigation/config.ts`, add:

```ts
{
  id: 'train-formations',
  label: () => m.app_train_formations(),
  icon: Combine,                // or TrainFront variant
  href: '/train-formations',
  isPrimary: false,             // → appears in "More" menu on mobile
  usePrefixMatch: true
}
```

### C4. i18n Keys

Add all `formations_*` keys to `messages/en.json` and `messages/it.json` (Italian translations required).

Minimum key set:

```json
"app_train_formations": "Train Formations",
"formations_page_title": "Train Formations",
"formations_add_stock": "Add Stock",
"formations_no_traction_warning": "No Traction",
"formations_traction_warning_tooltip": "This formation has no motorized unit",
"formations_owned_badge": "{n} owned",
"formations_planned_status": "Planned",
"formations_prototype_series_code": "Series Code",
"formations_prototype_car_type": "Car Type",
"formations_prototype_service_level": "Service Level",
"formations_search_placeholder": "Search prototypes...",
"formations_add_prototype_action": "+ Add Prototype",
"formations_drawer_group_by_operator": "Operator",
"formations_empty_composition": "Add your first stock unit to start building this formation.",
"formations_element_removed": "Unit removed from formation",
"formations_quick_assign": "Quick Assign",
"formations_assign_model": "Assign a model",
"formations_unassign_model": "Unassign",
"formations_stock_not_found": "Model no longer in inventory"
```

**Gate C**: `pnpm check` passes. `pnpm lint` passes. All Paraglide message keys resolve.

---

## Phase D — Drag-and-Drop

### D1. Install `svelte-dnd-action`

> **Approved**: `svelte-dnd-action` has been formally approved (2026-03-29). Add it directly.

```bash
pnpm add svelte-dnd-action
```

### D2. Implement Optimistic DnD in `FormationTrack.svelte`

```svelte
<script lang="ts">
  import { dndzone } from 'svelte-dnd-action';
  import { flip } from 'svelte/animate';

  let items = $state([...]);

  function handleConsider(e: CustomEvent) {
    items = e.detail.items;  // local state only — no DB call
  }

  async function handleFinalize(e: CustomEvent) {
    items = e.detail.items;
    await reorderFormationElements({ formation_id, ordered_element_ids: items.map(i => i.id) });
  }
</script>

<div
  use:dndzone={{ items, flipDurationMs: 200 }}
  onconsider={handleConsider}
  onfinalize={handleFinalize}
>
  {#each items as item (item.id)}
    <div animate:flip={{ duration: 200 }}>
      <FormationCell {item} />
    </div>
  {/each}
</div>
```

**Gate D**: Drag reorders items visually. `reorder_formation_elements` is called only on drop (not during drag). DB persists new order after reload.

---

## Phase E — Ownership Badge

### E1. Ownership count in view query

The `owned_count_for_prototype` field is computed in the Rust query:

```sql
SELECT
    fe.*,
    (SELECT COUNT(*) FROM owned_rolling_stocks ors
     WHERE ors.prototype_id = fe.prototype_id) AS owned_count_for_prototype
FROM formation_elements fe
WHERE fe.formation_id = ?
ORDER BY fe.position_order
```

### E2. `OwnershipBadge.svelte` logic

```svelte
<script lang="ts">
  let { ownedCount, assignedId }: { ownedCount: number; assignedId: string | null } = $props();
  let label = $derived(
    assignedId ? 'Assigned' : ownedCount > 0 ? `${ownedCount} owned` : 'Planned'
  );
  let variant = $derived(assignedId ? 'assigned' : ownedCount > 0 ? 'owned' : 'planned');
</script>
```

### E3. Quick Assign

When `ownedCount === 1`, render a "Quick Assign" button that calls `assignRollingStockToElement` with the single matching model's ID without opening the full picker.

**Gate E**: FormationCell renders correct badge. Owned cells show tinted background. Unowned cells show "Planned" state.

---

## Phase F — Tests

> **Constitution requirement**: Unit tests MUST exercise business logic in isolation; integration tests MUST validate cross-layer contracts (repo ↔ DB, use-case ↔ transport). Coverage targets: domain/use-case layer ≥80%, UI components ≥60%.

---

### F1. Rust Unit Tests — Domain Logic

Located inline in each domain module (standard Rust `#[cfg(test)]` blocks).

#### `formation/train_formation.rs` — aggregate invariants

| Test                                      | Scenario                               | Expected                |
| ----------------------------------------- | -------------------------------------- | ----------------------- |
| `test_formation_name_must_not_be_empty`   | Create with empty name                 | `Err(ValidationError)`  |
| `test_formation_start_after_end_rejected` | `start_year=1985, end_year=1980`       | `Err(InvalidYearRange)` |
| `test_formation_same_year_allowed`        | `start_year=1975, end_year=1975`       | `Ok`                    |
| `test_formation_null_years_allowed`       | Both years `None`                      | `Ok`                    |
| `test_formation_open_ended_allowed`       | `start_year=Some(1975), end_year=None` | `Ok`                    |

#### `trains/domain/traction.rs` (or inline in `formation_element.rs`) — traction evaluation

| Test                                   | Scenario                                      | Expected               |
| -------------------------------------- | --------------------------------------------- | ---------------------- |
| `test_traction_coach_only`             | 3 elements, all `car_type=Coach`, no override | `has_traction = false` |
| `test_traction_locomotive_counts`      | 1 Locomotive + 3 Coaches, no override         | `has_traction = true`  |
| `test_traction_power_car_counts`       | 1 PowerCar only                               | `has_traction = true`  |
| `test_traction_default_dummy_excluded` | Locomotive with `default_is_dummy=true`       | `has_traction = false` |
| `test_traction_override_force_include` | Coach with `traction_override=1`              | `has_traction = true`  |
| `test_traction_override_force_exclude` | Locomotive with `traction_override=-1`        | `has_traction = false` |
| `test_traction_override_exclude_all`   | Mixed Loco+Coach, all overridden to -1        | `has_traction = false` |
| `test_traction_empty_composition`      | No elements                                   | `has_traction = false` |

---

### F2. Rust Integration Tests — Repository ↔ SQLite

Located in `src-tauri/src/trains/infrastructure/` as a `#[cfg(test)]` module using an in-memory SQLite pool.

Setup helper: `async fn setup_db() -> SqlitePool` (runs migrations on `:memory:`, inserts minimal seed data).

#### `train_formation_repo`

| Test                                         | Scenario                                                          | Expected                                    |
| -------------------------------------------- | ----------------------------------------------------------------- | ------------------------------------------- |
| `test_create_formation_roundtrip`            | Insert → fetch by ID                                              | All fields match                            |
| `test_create_formation_duplicate_name`       | Two formations with same name                                     | Second insert → `DUPLICATE_NAME` error      |
| `test_update_formation_metadata`             | Update epoch + notes → fetch                                      | Updated fields; `version` incremented       |
| `test_delete_formation_cascades_elements`    | Create formation + 2 elements → delete formation → query elements | 0 elements remain                           |
| `test_list_formations_returns_traction_flag` | Formation with Locomotive element                                 | `TrainFormationSummary.has_traction = true` |

#### `formation_element_repo`

| Test                                         | Scenario                           | Expected                                                              |
| -------------------------------------------- | ---------------------------------- | --------------------------------------------------------------------- |
| `test_add_element_appended_at_end`           | Add 3 elements                     | `position_order` = 0, 1, 2                                            |
| `test_remove_element_shifts_positions`       | 3 elements → remove middle → fetch | Positions 0, 1 (no gap)                                               |
| `test_reorder_elements_atomic`               | 3 elements → reverse order         | All 3 position_orders updated; checked in single transaction          |
| `test_reorder_mismatched_ids_rejected`       | Send wrong/extra element IDs       | `Err(ElementIdsMismatch)`                                             |
| `test_duplicate_prototype_in_formation`      | Same `prototype_id` twice          | Both rows exist (FR-016)                                              |
| `test_assign_rolling_stock_to_element`       | Assign then fetch                  | `owned_rolling_stock_id` populated                                    |
| `test_unassign_rolling_stock`                | Assign then unassign (None)        | `owned_rolling_stock_id = NULL`                                       |
| `test_owned_rolling_stock_deleted_sets_null` | Delete `owned_rolling_stocks` row  | `FormationElement.owned_rolling_stock_id = NULL` (ON DELETE SET NULL) |
| `test_set_traction_override_values`          | Set 0, 1, −1 in sequence           | Persists each correctly                                               |

#### `prototype_repo`

| Test                                           | Scenario                                        | Expected                                              |
| ---------------------------------------------- | ----------------------------------------------- | ----------------------------------------------------- |
| `test_seed_prototypes_idempotent`              | Run seed twice                                  | Row count unchanged (INSERT OR IGNORE)                |
| `test_create_custom_prototype`                 | Insert with `is_custom=true`                    | `PrototypeView.is_custom = true`                      |
| `test_search_prototypes_filters_by_query`      | Search `"Gran Comfort"`                         | Returns only matching rows                            |
| `test_search_prototypes_grouped_by_company`    | Mixed companies                                 | Groups correct                                        |
| `test_prototype_delete_restricted_when_in_use` | Element references prototype → delete prototype | `Err(ForeignKeyViolation)` or custom `RESTRICT` error |

#### `formation_category_repo`

| Test                                             | Scenario                      | Expected              |
| ------------------------------------------------ | ----------------------------- | --------------------- |
| `test_seed_categories_idempotent`                | Run seed twice                | Row count unchanged   |
| `test_create_custom_category_duplicate_rejected` | Two categories with same name | `Err(DUPLICATE_NAME)` |

---

### F3. Rust Use-Case Boundary Tests

Located in `src-tauri/src/trains/application/`. Use repo mock traits or in-memory SQLite pool.

| Command                      | Test                                 | Expected                                             |
| ---------------------------- | ------------------------------------ | ---------------------------------------------------- |
| `create_train_formation`     | Missing name → Args validation fails | `400`-equivalent before repo call                    |
| `create_train_formation`     | Valid args                           | Returns `TrainFormationView`                         |
| `update_train_formation`     | ID not found                         | `NOT_FOUND` error                                    |
| `delete_train_formation`     | ID not found                         | `NOT_FOUND` error                                    |
| `add_formation_element`      | Invalid `prototype_id`               | `PROTOTYPE_NOT_FOUND`                                |
| `reorder_formation_elements` | Valid reorder                        | `reorder_formation_elements` repo method called once |
| `create_custom_prototype`    | Invalid `car_type` value             | `INVALID_CAR_TYPE` error                             |
| `create_custom_prototype`    | Unknown `railway_company_id`         | `COMPANY_NOT_FOUND` error                            |

---

### F4. Frontend Unit Tests (Vitest — `src/__tests__/lib/features/train-formations/`)

Follow the **svelte-test-writer** skill rules:

- `cleanup()` + `vi.clearAllMocks()` in every `beforeEach`
- All mocks use `.mockResolvedValue()` — never unresolved promises
- All post-render assertions use `await waitFor(…, { timeout: 2000 })`
- No raw Proxy mocks; mock Tauri commands via `vi.mock('$lib/services/formations.service')`

#### `domain/traction.test.ts` — pure function (no DOM)

```
describe('isTractionSlot')
  ✓ returns true for Locomotive with no override
  ✓ returns true for PowerCar with no override
  ✓ returns false for Coach regardless of override=0
  ✓ returns false for Locomotive with default_is_dummy=true
  ✓ returns true for Coach with traction_override=1
  ✓ returns false for Locomotive with traction_override=-1
  ✓ returns false when entry is null/undefined

describe('hasTraction(elements)')
  ✓ returns false for empty array
  ✓ returns false for array of all-coach elements
  ✓ returns true for at least one non-dummy locomotive
  ✓ returns true for dummy locomotive with override=1
```

#### `components/icons/PrototypeIcon.test.ts`

```
describe('PrototypeIcon — icon dispatch')
  ✓ renders Locomotive icon for car_type="Locomotive"
  ✓ renders Locomotive icon for car_type="PowerCar"
  ✓ renders Coach icon for car_type="Coach"
  ✓ renders Coach icon for car_type="Couchette"
  ✓ renders Coach icon for car_type="Dining"
  ✓ renders Wagon icon for car_type="FreightWagon"
  ✓ renders Wagon icon for car_type="BaggageCar"
  ✓ falls back to Wagon icon for unknown car_type string

describe('PrototypeIcon — ownership styling')
  ✓ applies text-blue-600 class when isOwned=true
  ✓ applies text-gray-400 and opacity-60 when isOwned=false
  ✓ renders dashed border overlay div when isOwned=false
  ✓ does NOT render dashed border div when isOwned=true
  ✓ forwards extra class prop to wrapper element

describe('base icons — SVG contract')
  ✓ Locomotive renders svg with stroke="currentColor"
  ✓ Coach renders svg with stroke="currentColor"
  ✓ Wagon renders svg with stroke="currentColor"
  ✓ all three icons accept size prop and apply it as class
```

> **Implementation note**: query the SVG element with `getByRole('img')` or a data-testid; do NOT test internal SVG path strings — test only the observable DOM structure (presence of `<svg>`, class attributes, wrapper classes).

#### `components/OwnershipBadge.test.ts`

```
describe('OwnershipBadge')
  ✓ renders "Planned" when ownedCount=0 and no assignment
  ✓ renders "1 owned" when ownedCount=1 and no assignment
  ✓ renders "Assigned" when assignedId is set
  ✓ applies owned CSS variant class when ownedCount > 0
  ✓ applies planned CSS variant class when ownedCount = 0
```

#### `components/FormationCell.test.ts`

```
describe('FormationCell')
  ✓ renders series code from prototype
  ✓ renders PrototypeIcon with correct type and isOwned props
  ✓ renders OwnershipBadge with correct ownedCount
  ✓ renders "Quick Assign" button when ownedCount === 1 and no assignment
  ✓ does NOT render "Quick Assign" when ownedCount !== 1
  ✓ renders stock-not-found indicator when prototype_id resolves to null
  ✓ does not throw when owned_rolling_stock_id is null
```

#### `components/TractionWarning.test.ts`

```
describe('TractionWarning')
  ✓ renders warning icon when hasTraction=false
  ✓ does not render when hasTraction=true
  ✓ renders tooltip text from Paraglide key (formations_traction_warning_tooltip)
```

#### `components/FormationForm.test.ts`

```
describe('FormationForm')
  ✓ submit button disabled when name is empty
  ✓ year range error shown when startYear > endYear
  ✓ year range error clears when endYear corrected
  ✓ submit button disabled when year range invalid
  ✓ renders category picker populated with categories
  ✓ calls onSubmit with correct args when form is valid
```

#### `components/AddStockDrawer.test.ts`

```
describe('AddStockDrawer')
  setup: mock getPrototypes to return groups [{ company: 'FS', prototypes: [...] }]

  ✓ renders without throwing when open=false
  ✓ shows prototype groups when open=true (await waitFor)
  ✓ groups results by railway company heading
  ✓ filters results as user types in search box
  ✓ shows "+ Add Prototype" action when search returns zero results
  ✓ calls addFormationElement with correct prototype_id when user clicks a result
  ✓ closes drawer after successful add
  ✓ shows inline CreatePrototypeForm when "+ Add Prototype" clicked
```

#### `TrainFormationState.test.ts`

```
describe('TrainFormationState')
  ✓ initialises with empty formations list
  ✓ hasTraction is false on empty formation
  ✓ hasTraction updates to true after locomotive element added
  ✓ hasTraction updates to false after locomotive element removed
  ✓ reorder updates local element order before DB call (optimistic)
  ✓ reorder triggers reorderFormationElements service exactly once on finalize
  ✓ reorder does NOT call service during consider phase
```

#### `components/FormationList.test.ts`

```
describe('FormationList')
  ✓ renders empty-state prompt when formations=[]
  ✓ renders one FormationCard per formation
  ✓ FormationCard shows TractionWarning when has_traction=false
  ✓ FormationCard shows owned/planned counts
```

---

### F5. Edge Case Coverage (maps to spec "Edge Cases" section)

| Spec Edge Case                                                                  | Covered by                                                                                       |
| ------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| Rolling stock deleted from inventory → composition entry remains with indicator | `test_owned_rolling_stock_deleted_sets_null` (Rust) + `FormationCell stock-not-found` (frontend) |
| Two formations same name → rejected                                             | `test_create_formation_duplicate_name` (Rust) + `FormationForm` name field (frontend)            |
| `start_year > end_year` → validation blocked                                    | `test_formation_start_after_end_rejected` (Rust) + `FormationForm` range validation (frontend)   |
| Empty composition → empty-state placeholder                                     | `FormationList` empty state test + `test_traction_empty_composition` (Rust)                      |
| Missing railway badge → fallback text                                           | `FormationCell` + `OwnershipBadge` render tests                                                  |
| 50+ units → scroll remains performant                                           | Manual/SC-002; note in Vitest: render 50-element list and assert no render throw                 |
| Duplicate prototype slots allowed                                               | `test_duplicate_prototype_in_formation` (Rust)                                                   |

---

**Gate F**: `cargo test` passes. `pnpm test` passes. No skipped or deleted tests. Coverage report (`pnpm test:coverage`) shows domain/use-case ≥80%, UI components ≥60%.

---

## Phase G — Final Validation

```bash
cargo fmt --check
cargo clippy -- -D warnings
pnpm format --check
pnpm lint
pnpm check
pnpm test
cargo test --manifest-path src-tauri/Cargo.toml
```

All commands pass with zero errors/warnings. Feature is ready for `/speckit.tasks`.

---

## Design Notes (Steampunk Aesthetic)

- Formation builder should follow the `designer` skill: Iron/Copper palette for detail pages; Parchment/Brass for overview cards.
- `FormationCell` uses the mechanical precision style: monospace series codes, industrial icon borders.
- `TractionWarning` uses a glowing amber/copper warning icon (matches the signal-failure aesthetic elsewhere).
- `IdentityCard` is the "locomotive nameplate" — prominent typography, epoch badge.

> Apply the `designer` skill when implementing Svelte components.
