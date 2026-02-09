# Quickstart: Dashboard Collector's Overview Redesign

**Feature**: 017-dashboard-redesign  
**Date**: February 9, 2026  
**Purpose**: Step-by-step implementation guide in recommended order

## Implementation Order

Follow these phases sequentially to build the feature incrementally with testable checkpoints.

---

## Phase 0: Setup & Foundation (30 minutes)

### Step 0.1: Add Paraglide Message Keys

**File**: `messages/en.json`

Add new message keys for purchase groups:

```json
{
  "dashboard_purchase_group_title": "Recent Acquisitions",
  "dashboard_purchase_on": "Purchased on {date}",
  "dashboard_seller_from": "from {seller}",
  "dashboard_seller_unknown": "Unknown source",
  "dashboard_purchase_notes": "Notes: {notes}",
  "dashboard_more_items": "+{count} more items",
  "dashboard_condition_new": "New",
  "dashboard_condition_preowned": "Pre-owned",
  "dashboard_condition_unknown": "Unknown",
  "dashboard_empty_purchases": "No recent acquisitions",
  "dashboard_add_first_purchase": "Add your first model to get started"
}
```

**File**: `messages/it.json`

Add Italian translations:

```json
{
  "dashboard_purchase_group_title": "Acquisti Recenti",
  "dashboard_purchase_on": "Acquistato il {date}",
  "dashboard_seller_from": "da {seller}",
  "dashboard_seller_unknown": "Fonte sconosciuta",
  "dashboard_purchase_notes": "Note: {notes}",
  "dashboard_more_items": "+{count} articoli in più",
  "dashboard_condition_new": "Nuovo",
  "dashboard_condition_preowned": "Usato",
  "dashboard_condition_unknown": "Sconosciuto",
  "dashboard_empty_purchases": "Nessun acquisto recente",
  "dashboard_add_first_purchase": "Aggiungi il tuo primo modello per iniziare"
}
```

**Verification**: Run `pnpm prepare` to regenerate Paraglide types.

---

## Phase 1: Backend Domain Layer (1 hour)

### Step 1.1: Create PurchaseCondition Enum

**File**: `src-tauri/src/dashboard/domain/purchase_condition.rs` (NEW)

```rust
use serde::{Deserialize, Serialize};

/// Condition status of a model at time of purchase
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PurchaseCondition {
    /// Brand new, unopened
    New,
    /// Previously owned, used
    PreOwned,
    /// Condition not specified
    Unknown,
}

impl Default for PurchaseCondition {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<Option<String>> for PurchaseCondition {
    fn from(value: Option<String>) -> Self {
        match value.as_deref() {
            Some("NEW") => Self::New,
            Some("PRE_OWNED") => Self::PreOwned,
            _ => Self::Unknown,
        }
    }
}
```

**Verification**: `cargo test` in dashboard module.

---

### Step 1.2: Create ModelCard Entity

**File**: `src-tauri/src/dashboard/domain/model_card.rs` (NEW)

```rust
use crate::catalog::domain::railway_model::RailwayModelId;
use serde::Serialize;
use super::purchase_condition::PurchaseCondition;

/// Compact view of a railway model for dashboard card display
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ModelCard {
    /// Unique model identifier
    pub id: RailwayModelId,

    /// Path to thumbnail image (relative to data directory)
    pub thumbnail_path: Option<String>,

    /// Manufacturer name
    pub manufacturer: String,

    /// Product code from manufacturer
    pub product_code: String,

    /// Purchase condition status
    pub condition: PurchaseCondition,

    /// Model description or auto-generated title
    pub description: String,
}
```

**Verification**: Add to `src-tauri/src/dashboard/domain/mod.rs`:

```rust
pub mod purchase_condition;
pub mod model_card;

pub use purchase_condition::PurchaseCondition;
pub use model_card::ModelCard;
```

---

### Step 1.3: Create PurchaseGroup Entity

**File**: `src-tauri/src/dashboard/domain/purchase_group.rs` (NEW)

```rust
use serde::Serialize;
use super::model_card::ModelCard;

/// A group of models acquired together (same purchase date + seller)
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseGroup {
    /// Unique identifier for display purposes
    pub id: String,

    /// Date when the models were purchased (ISO 8601 date string)
    pub purchase_date: String,

    /// Name of the seller/shop (optional)
    pub seller_name: Option<String>,

    /// User notes about this purchase transaction
    pub notes: Option<String>,

    /// List of model cards in this purchase (max 3 for display)
    pub model_cards: Vec<ModelCard>,

    /// Total number of models in this purchase
    pub total_count: usize,
}
```

**Verification**: Export in `mod.rs`:

```rust
pub mod purchase_group;
pub use purchase_group::PurchaseGroup;
```

---

### Step 1.4: Extend DashboardSummary

**File**: `src-tauri/src/dashboard/domain/dashboard_summary.rs`

Add purchase_groups field:

```rust
use super::purchase_group::PurchaseGroup;

#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DashboardSummary {
    pub totals: DashboardTotals,

    /// NEW: Recent purchase groups
    pub purchase_groups: Vec<PurchaseGroup>,

    pub depot_items: Vec<DashboardDepotEntry>,

    /// DEPRECATED: Legacy recent items
    #[deprecated(note = "Use purchase_groups instead")]
    pub recent_items: Vec<DashboardRecentItem>,
}
```

**Verification**: `cargo check`

---

## Phase 2: Backend Infrastructure Layer (2 hours)

### Step 2.1: Create Row Entities

**File**: `src-tauri/src/dashboard/infrastructure/entities.rs`

Add new row types:

```rust
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct PurchaseGroupRow {
    pub purchase_date: String,
    pub seller_id: Option<String>,
    pub seller_name: Option<String>,
    pub notes: Option<String>,
    pub model_count: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct ModelCardRow {
    pub model_id: String,
    pub manufacturer_id: String,
    pub manufacturer_name: String,
    pub product_code: String,
    pub description: String,
    pub image_path: Option<String>,
    pub purchase_condition: Option<String>,
    pub purchase_date: String,
    pub seller_id: Option<String>,
}
```

---

### Step 2.2: Implement Row Conversions

**File**: `src-tauri/src/dashboard/infrastructure/entities.rs`

Add TryFrom implementations:

```rust
use crate::dashboard::domain::{PurchaseGroup, ModelCard, PurchaseCondition};
use crate::catalog::domain::railway_model::RailwayModelId;
use std::str::FromStr;

impl TryFrom<(PurchaseGroupRow, Vec<ModelCardRow>)> for PurchaseGroup {
    type Error = DomainError;

    fn try_from(value: (PurchaseGroupRow, Vec<ModelCardRow>)) -> Result<Self, Self::Error> {
        let (group_row, card_rows) = value;

        let id = format!(
            "purchase-{}-{}",
            group_row.purchase_date,
            group_row.seller_id.as_deref().unwrap_or("unknown")
        );

        let model_cards: Vec<ModelCard> = card_rows
            .into_iter()
            .take(3)
            .map(|row| row.try_into())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(PurchaseGroup {
            id,
            purchase_date: group_row.purchase_date,
            seller_name: group_row.seller_name,
            notes: group_row.notes,
            model_cards,
            total_count: group_row.model_count as usize,
        })
    }
}

impl TryFrom<ModelCardRow> for ModelCard {
    type Error = DomainError;

    fn try_from(row: ModelCardRow) -> Result<Self, Self::Error> {
        Ok(ModelCard {
            id: RailwayModelId::from_str(&row.model_id)?,
            thumbnail_path: row.image_path,
            manufacturer: row.manufacturer_name,
            product_code: row.product_code,
            condition: PurchaseCondition::from(row.purchase_condition),
            description: row.description,
        })
    }
}
```

**Verification**: `cargo test` for conversions.

---

### Step 2.3: Add Repository Query Method

**File**: `src-tauri/src/dashboard/infrastructure/dashboard_repository.rs`

Add fetch_purchase_groups method:

```rust
impl DashboardRepository {
    pub async fn fetch_purchase_groups(
        &self,
        pool: &SqlitePool,
    ) -> Result<Vec<PurchaseGroup>, RepositoryError> {
        // Step 1: Group by purchase_date + seller_id
        let group_rows = sqlx::query_as::<_, PurchaseGroupRow>(
            r#"
            SELECT
                pi.purchase_date,
                pi.seller_id,
                s.name as seller_name,
                ci.notes,
                COUNT(*) as model_count
            FROM purchase_infos pi
            LEFT JOIN sellers s ON pi.seller_id = s.id
            LEFT JOIN collection_items ci ON pi.collection_item_id = ci.id
            WHERE ci.removed_date IS NULL
            GROUP BY pi.purchase_date, pi.seller_id
            ORDER BY pi.purchase_date DESC
            LIMIT 3
            "#
        )
        .fetch_all(pool)
        .await?;

        // Step 2: For each group, fetch up to 3 model cards
        let mut purchase_groups = Vec::new();

        for group_row in group_rows {
            let model_rows = sqlx::query_as::<_, ModelCardRow>(
                r#"
                SELECT
                    rm.id as model_id,
                    rm.manufacturer_id,
                    m.name as manufacturer_name,
                    rm.product_code,
                    rm.description,
                    rm.image_path,
                    ci.purchase_condition,
                    pi.purchase_date,
                    pi.seller_id
                FROM purchase_infos pi
                INNER JOIN collection_items ci ON pi.collection_item_id = ci.id
                INNER JOIN railway_models rm ON ci.railway_model_id = rm.id
                INNER JOIN manufacturers m ON rm.manufacturer_id = m.id
                WHERE pi.purchase_date = $1
                  AND (pi.seller_id = $2 OR (pi.seller_id IS NULL AND $2 IS NULL))
                  AND ci.removed_date IS NULL
                ORDER BY ci.added_date DESC
                LIMIT 3
                "#
            )
            .bind(&group_row.purchase_date)
            .bind(&group_row.seller_id)
            .fetch_all(pool)
            .await?;

            let purchase_group = PurchaseGroup::try_from((group_row, model_rows))?;
            purchase_groups.push(purchase_group);
        }

        Ok(purchase_groups)
    }
}
```

**Verification**: Write integration test with test database.

---

### Step 2.4: Update get_dashboard_summary Method

**File**: `src-tauri/src/dashboard/infrastructure/dashboard_repository.rs`

Modify existing method:

```rust
pub async fn get_dashboard_summary(
    &self,
    pool: &SqlitePool,
) -> Result<DashboardSummary, RepositoryError> {
    let totals = self.fetch_totals(pool).await?;
    let purchase_groups = self.fetch_purchase_groups(pool).await?;
    let depot_items = self.fetch_depot_items(pool).await?;

    Ok(DashboardSummary {
        totals,
        purchase_groups,
        depot_items,
        recent_items: vec![], // Deprecated
    })
}
```

**Verification**: `cargo test` for repository.

---

## Phase 3: Backend Application Layer (30 minutes)

### Step 3.1: Update Tauri Command Handler

**File**: `src-tauri/src/dashboard/application/get_dashboard_summary.rs`

No changes needed if repository is already called correctly. Verify command is registered:

**File**: `src-tauri/src/lib.rs`

```rust
tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        // ... existing commands
        dashboard::application::get_dashboard_summary,
    ])
```

**Verification**: `pnpm tauri dev` and check command registration.

---

### Step 3.2: Regenerate TypeScript Types

Run specta type generation:

```bash
pnpm run tauri:gen-types
```

**Verification**: Check `src/lib/bindings.ts` for new types:

- `PurchaseGroup`
- `ModelCard`
- `PurchaseCondition`

---

## Phase 4: Frontend Components (3 hours)

### Step 4.1: Create ModelCard Component

**File**: `src/lib/features/dashboard/components/ModelCard.svelte` (NEW)

See [ModelCard.contract.md](./contracts/ModelCard.contract.md) for full implementation.

**Key Points**:

- Use `convertFileSrc` for image loading
- Display placeholder for missing images
- Implement click navigation to model details
- Show condition badge with proper variant

**Verification**: Create Vitest component test.

---

### Step 4.2: Create PurchaseGroupCard Component

**File**: `src/lib/features/dashboard/components/PurchaseGroupCard.svelte` (NEW)

See [PurchaseGroupCard.contract.md](./contracts/PurchaseGroupCard.contract.md) for full implementation.

**Key Points**:

- Display purchase metadata (date, seller, notes)
- Render ModelCard grid (1-3 columns responsive)
- Show "+N more items" indicator

**Verification**: Create Vitest component test with mock data.

---

### Step 4.3: Export New Components

**File**: `src/lib/features/dashboard/index.ts`

Add exports:

```typescript
export { default as PurchaseGroupCard } from './components/PurchaseGroupCard.svelte';
export { default as ModelCard } from './components/ModelCard.svelte';
```

---

## Phase 5: Frontend Dashboard Page (1 hour)

### Step 5.1: Update Dashboard Page Layout

**File**: `src/routes/my-dashboard/+page.svelte`

Replace "Recently Added" section with purchase groups:

```svelte
<script lang="ts">
  import { PurchaseGroupCard } from '$lib/features/dashboard';

  const purchaseGroups = $derived(dashboard.data?.purchaseGroups ?? []);
</script>

<!-- Replace existing "Recently Added" section -->
<section>
  <div class="mb-4 flex items-center justify-between">
    <h3 class="h3 text-surface-300 text-sm font-bold tracking-wider uppercase">
      {m.dashboard_purchase_group_title()}
    </h3>
    <a href={resolve('/my-collection')} class="text-accent-500 text-sm font-bold hover:underline">
      {m.dashboard_view_all()}
    </a>
  </div>

  {#if dashboard.isLoading}
    <!-- Loading skeleton -->
    <div class="space-y-6">
      {#each Array(2) as _, i (i)}
        <div class="skeleton rounded-container h-64 w-full"></div>
      {/each}
    </div>
  {:else if purchaseGroups.length === 0}
    <!-- Empty state -->
    <div class="blueprint-panel card p-10 text-center">
      <p class="text-base font-semibold">
        {m.dashboard_empty_purchases()}
      </p>
      <Button variant="primary" onclick={() => goto(resolve('/catalogue/new-model'))}>
        <Plus class="mr-2" />
        {m.dashboard_add_first_purchase()}
      </Button>
    </div>
  {:else}
    <!-- Purchase Groups -->
    <div class="space-y-6">
      {#each purchaseGroups as group (group.id)}
        <PurchaseGroupCard {group} />
      {/each}
    </div>
  {/if}
</section>
```

**Verification**: Dashboard renders purchase groups correctly.

---

### Step 5.2: Implement Scroll Position Restoration

**File**: `src/routes/my-dashboard/+page.svelte`

Add scroll handling:

```svelte
<script lang="ts">
  import { afterNavigate } from '$app/navigation';
  import { onMount } from 'svelte';

  let scrollY = $state(0);

  onMount(() => {
    const savedScroll = sessionStorage.getItem('dashboard-scroll');
    if (savedScroll) {
      scrollY = parseInt(savedScroll, 10);
      window.scrollTo(0, scrollY);
      sessionStorage.removeItem('dashboard-scroll');
    }
  });

  afterNavigate(() => {
    sessionStorage.setItem('dashboard-scroll', window.scrollY.toString());
  });
</script>
```

**Verification**: Navigate to model → back to dashboard, scroll position preserved.

---

## Phase 6: Testing (2 hours)

### Step 6.1: Backend Unit Tests

**File**: `src-tauri/src/dashboard/infrastructure/dashboard_repository_test.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_purchase_groups_limits_to_3() {
        let pool = setup_test_pool().await;
        seed_4_purchase_groups(&pool).await;

        let repo = DashboardRepository::new();
        let groups = repo.fetch_purchase_groups(&pool).await.unwrap();

        assert_eq!(groups.len(), 3);
    }

    #[tokio::test]
    async fn test_model_cards_limited_to_3_per_group() {
        let pool = setup_test_pool().await;
        seed_purchase_with_5_models(&pool).await;

        let repo = DashboardRepository::new();
        let groups = repo.fetch_purchase_groups(&pool).await.unwrap();

        assert_eq!(groups[0].model_cards.len(), 3);
        assert_eq!(groups[0].total_count, 5);
    }
}
```

---

### Step 6.2: Frontend Component Tests

**File**: `src/lib/features/dashboard/components/__tests__/ModelCard.test.ts`

```typescript
import { describe, it, expect } from 'vitest';
import { render } from '@testing-library/svelte';
import ModelCard from '../ModelCard.svelte';

describe('ModelCard', () => {
  it('renders model information correctly', () => {
    const model = {
      id: 'trn:railway-model:roco:62150',
      thumbnailPath: null,
      manufacturer: 'Roco',
      productCode: '62150',
      condition: 'NEW',
      description: 'Electric locomotive BR 193 Vectron'
    };

    const { getByText } = render(ModelCard, { props: { model } });

    expect(getByText('Roco')).toBeTruthy();
    expect(getByText('62150')).toBeTruthy();
    expect(getByText('New')).toBeTruthy();
  });
});
```

---

## Phase 7: Integration & Verification (1 hour)

### Verification Checklist

- [ ] **Backend**: `cargo test` passes all tests
- [ ] **Backend**: `cargo clippy` shows no warnings
- [ ] **Backend**: `cargo fmt` formats correctly
- [ ] **Frontend**: `pnpm test` passes all component tests
- [ ] **Frontend**: `pnpm lint` shows no errors
- [ ] **Frontend**: `pnpm check` (svelte-check) passes
- [ ] **Frontend**: `pnpm format` formats correctly
- [ ] **Integration**: Dashboard loads within 2 seconds
- [ ] **Integration**: Purchase groups display correctly
- [ ] **Integration**: Model cards navigate to details page
- [ ] **Integration**: "+N more items" shows when needed
- [ ] **Integration**: Empty state shows when no purchases
- [ ] **Integration**: Scroll position preserved after navigation
- [ ] **Responsive**: Mobile layout (320px) works correctly
- [ ] **Responsive**: Tablet layout (768px) works correctly
- [ ] **Responsive**: Desktop layout (1920px) works correctly

---

## Common Issues & Solutions

### Issue: TypeScript types not updating

**Solution**: Run `pnpm run tauri:gen-types` to regenerate bindings.

### Issue: Images not loading

**Solution**: Check `convertFileSrc` import and verify image paths start from data directory.

### Issue: Purchase groups show old data

**Solution**: Clear cache or restart Tauri dev server (`pnpm tauri dev`).

### Issue: Grid layout breaks on mobile

**Solution**: Verify Tailwind classes: `grid-cols-1 sm:grid-cols-2 lg:grid-cols-3`.

### Issue: Scroll position not preserved

**Solution**: Check sessionStorage is available and key name matches.

---

## Time Estimates

| Phase                               | Duration | Cumulative   |
| ----------------------------------- | -------- | ------------ |
| Phase 0: Setup                      | 30 min   | 30 min       |
| Phase 1: Backend Domain             | 1 hour   | 1.5 hours    |
| Phase 2: Backend Infrastructure     | 2 hours  | 3.5 hours    |
| Phase 3: Backend Application        | 30 min   | 4 hours      |
| Phase 4: Frontend Components        | 3 hours  | 7 hours      |
| Phase 5: Frontend Dashboard Page    | 1 hour   | 8 hours      |
| Phase 6: Testing                    | 2 hours  | 10 hours     |
| Phase 7: Integration & Verification | 1 hour   | **11 hours** |

**Total Estimated Time**: 11 hours (1.5 developer days)

---

## Post-Implementation

### Performance Validation

1. Open DevTools Network tab
2. Load dashboard
3. Verify total load time < 2 seconds
4. Check query count (should be ≤ 10 queries)

### User Acceptance Testing

1. Add 3 models from different purchase dates
2. Verify grouping appears correctly
3. Add 5 models in single purchase
4. Verify "+2 more items" indicator
5. Click model card → verify navigation
6. Navigate back → verify scroll position

### Documentation Updates

- Update `docs/FEATURE_IMPLEMENTATION.md` with dashboard redesign notes
- Add screenshots to `specs/017-dashboard-redesign/` directory
- Update CHANGELOG.md with feature addition
