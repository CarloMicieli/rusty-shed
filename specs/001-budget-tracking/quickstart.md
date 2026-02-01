# Quickstart: Budget Tracking Feature

**Feature**: 001-budget-tracking  
**Date**: January 30, 2026  
**Estimated Effort**: 3-4 days

---

## Prerequisites

Before starting implementation, ensure:

1. ✅ Branch `001-budget-tracking` is checked out
2. ✅ Dependencies are installed: `pnpm install`
3. ✅ Rust toolchain is ready: `rustup show` (Rust 1.93.0+)
4. ✅ Database is accessible: `pnpm tauri dev` starts without errors

---

## Implementation Order

### Phase 1: Backend Foundation (Day 1)

#### Step 1.1: Create Migration File

```bash
# Create new migration
touch src-tauri/migrations/0007_create_budget_schema.sql
```

Add schema from [data-model.md](./data-model.md):

- `budget_config` table
- `extra_budgets` table
- Required indexes

#### Step 1.2: Create Budget Module Structure

```bash
# Create directory structure
mkdir -p src-tauri/src/budget/{domain,application,infrastructure,interface}

# Create module files
touch src-tauri/src/budget/mod.rs
touch src-tauri/src/budget/domain/{mod.rs,budget_configuration.rs,budget_events.rs,extra_budget_entry.rs,repository.rs}
touch src-tauri/src/budget/application/{mod.rs,set_budget.rs,add_extra_budget.rs,budget_query.rs}
touch src-tauri/src/budget/infrastructure/{mod.rs,entities.rs,mappers.rs,database.rs,repositories.rs}
touch src-tauri/src/budget/interface/{mod.rs,command_args.rs,command_handlers.rs}
```

#### Step 1.3: Register Module

Add to `src-tauri/src/lib.rs`:

```rust
pub mod budget;
```

#### Step 1.4: Implement Domain Layer

1. `BudgetConfiguration` aggregate with:
   - `monthly_amount()` calculation
   - `yearly_amount()` calculation
   - `pending_events` for domain event pattern

2. `ExtraBudgetEntry` value object

3. `BudgetEvent` enum for domain events

4. `BudgetRepository` trait

**Verification**:

```bash
cd src-tauri && cargo check
```

### Phase 2: Backend Infrastructure (Day 1-2)

#### Step 2.1: Implement Database Layer

1. Row types in `entities.rs`
2. Pure mappers in `mappers.rs`
3. SQL queries in `database.rs`
4. Repository impl in `repositories.rs`

#### Step 2.2: Implement Application Layer

1. `SetBudgetUseCase` for configuration
2. `AddExtraBudgetUseCase` for injections
3. `BudgetQueryService` for reads

#### Step 2.3: Implement Interface Layer

1. Transport DTOs in `command_args.rs`
2. Tauri handlers in `command_handlers.rs`

**Verification**:

```bash
cd src-tauri && cargo test
pnpm rust:clippy
```

### Phase 3: Backend Query Integration (Day 2)

#### Step 3.1: Monthly Budget Calculation

Implement rollover chain calculation:

```rust
fn calculate_monthly_records(
    year: i32,
    config: &BudgetConfiguration,
    spending: &[MonthlySpending],
    extras: &[ExtraBudgetEntry],
) -> Vec<MonthlyBudgetRecord>
```

#### Step 3.2: Dashboard Summary

Add to existing `DashboardSummary`:

```rust
pub budget: Option<BudgetDashboardSummary>,
```

#### Step 3.3: Historical Quarterly Query

Implement SQL aggregation for 5-year quarterly view.

**Verification**:

```bash
pnpm rust:test
pnpm rust:clippy
```

### Phase 4: Frontend State & Service (Day 2-3)

#### Step 4.1: Generate Bindings

```bash
pnpm tauri dev  # Triggers specta binding generation
```

Verify new types in `src/lib/bindings.ts`.

#### Step 4.2: Create Budget Service

```typescript
// src/lib/features/budget/services/budget.service.ts
export async function getBudgetConfig(): Promise<BudgetConfigDto | null>;
export async function setBudgetConfig(args: SetBudgetConfigArgs): Promise<BudgetConfigDto>;
export async function getBudgetDashboard(): Promise<BudgetDashboardSummary | null>;
// ... etc
```

#### Step 4.3: Create Budget State Controller

```typescript
// src/lib/features/budget/BudgetState.svelte.ts
export class BudgetState {
  #config = $state<BudgetConfigDto | null>(null);
  #dashboard = $state<BudgetDashboardSummary | null>(null);
  #monthlyRecords = $state<MonthlyBudgetRecord[]>([]);
  // ...
}
```

**Verification**:

```bash
pnpm check
pnpm lint
```

### Phase 5: Frontend Components (Day 3-4)

#### Step 5.1: Dashboard Widgets

1. `BudgetDonutChart.svelte` - Remaining budget gauge
2. `YearlySpendingChart.svelte` - Monthly bar chart
3. `ActivityHeatmap.svelte` - 5-year quarterly grid

#### Step 5.2: Budget Management Page

1. Create route: `src/routes/my-budget/+page.svelte`
2. `BudgetTable.svelte` - 12-month detail table
3. `ExtraBudgetModal.svelte` - Add extra budget popup
4. `QuarterlySummaryModal.svelte` - Historical drill-down

#### Step 5.3: Integrate with Dashboard

Update `src/routes/my-dashboard/+page.svelte` to include budget widgets.

**Verification**:

```bash
pnpm check
pnpm test
pnpm lint
```

### Phase 6: Localization & Polish (Day 4)

#### Step 6.1: Add Paraglide Messages

Add to `messages/en.json` and `messages/it.json`:

```json
{
  "budget.title": "Budget",
  "budget.remaining": "Remaining",
  "budget.monthlySpend": "Monthly Spending",
  "budget.extraBudget": "Extra Budget"
  // ... etc
}
```

#### Step 6.2: Final Testing

```bash
# Full verification
pnpm format
pnpm lint
pnpm check
pnpm test
pnpm rust:format
pnpm rust:clippy
pnpm rust:test
```

---

## Key Files to Create/Modify

### New Files (Backend)

| Path                                       | Purpose         |
| ------------------------------------------ | --------------- |
| `migrations/0007_create_budget_schema.sql` | Database schema |
| `src/budget/mod.rs`                        | Module root     |
| `src/budget/domain/*.rs`                   | Domain entities |
| `src/budget/application/*.rs`              | Use cases       |
| `src/budget/infrastructure/*.rs`           | Persistence     |
| `src/budget/interface/*.rs`                | Tauri commands  |

### New Files (Frontend)

| Path                                                 | Purpose          |
| ---------------------------------------------------- | ---------------- |
| `src/lib/features/budget/index.ts`                   | Public exports   |
| `src/lib/features/budget/BudgetState.svelte.ts`      | State controller |
| `src/lib/features/budget/services/budget.service.ts` | API calls        |
| `src/lib/features/budget/components/*.svelte`        | UI components    |
| `src/routes/my-budget/+page.svelte`                  | Management page  |

### Modified Files

| Path                                   | Change               |
| -------------------------------------- | -------------------- |
| `src-tauri/src/lib.rs`                 | Add `pub mod budget` |
| `src/lib/bindings.ts`                  | Auto-generated types |
| `src/routes/my-dashboard/+page.svelte` | Add budget widgets   |
| `messages/en.json`                     | Budget strings       |
| `messages/it.json`                     | Budget strings (IT)  |

---

## Testing Strategy

### Unit Tests (Rust)

```rust
#[cfg(test)]
mod tests {
    // Domain: rollover calculation
    #[test]
    fn test_rollover_surplus_carries_forward() { ... }

    #[test]
    fn test_rollover_deficit_reduces_next_month() { ... }

    #[test]
    fn test_yearly_mode_divides_by_12() { ... }
}
```

### Integration Tests (Rust)

```rust
#[sqlx::test]
async fn test_budget_config_persistence() { ... }

#[sqlx::test]
async fn test_extra_budget_aggregation() { ... }
```

### Component Tests (Vitest)

```typescript
describe('BudgetDonutChart', () => {
  it('shows green when 75% remaining', () => { ... });
  it('shows red when below 20%', () => { ... });
});

describe('BudgetState', () => {
  it('loads config on initialization', () => { ... });
  it('recalculates monthly records when config changes', () => { ... });
});
```

---

## Common Pitfalls

1. **Currency mismatch**: Ensure all spending queries filter by budget currency
2. **Rollover chain**: Remember to recalculate from earliest modified month forward
3. **January reset**: Check `last_reset_year` on every dashboard load
4. **Type generation**: Run `pnpm tauri dev` after adding new Rust types to regenerate bindings
5. **Paraglide**: All user-visible strings must use message keys, not hardcoded text

---

## Success Criteria Checklist

- [ ] Budget configuration saves and loads correctly
- [ ] Monthly rollover calculates accurately for surplus and deficit
- [ ] Dashboard donut chart shows correct remaining percentage
- [ ] Dashboard bar chart displays 12 months with goal line
- [ ] Activity heatmap shows 5 years of quarterly data
- [ ] Extra budget modal allows adding/removing entries
- [ ] Budget management page shows full year table
- [ ] Historical accordion shows quarterly summaries
- [ ] All strings use Paraglide messages
- [ ] All Rust tests pass
- [ ] All TypeScript tests pass
- [ ] clippy passes with no warnings
- [ ] ESLint passes with no errors
