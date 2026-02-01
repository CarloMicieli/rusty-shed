# API Contracts: Budget Tracking

**Feature**: 001-budget-tracking  
**Date**: January 30, 2026  
**Transport**: Tauri IPC via `invoke()`  
**Type Generation**: specta/tauri-specta

---

## Commands Overview

| Command                      | Type    | Description                              |
| ---------------------------- | ------- | ---------------------------------------- |
| `get_budget_config`          | Query   | Get current budget configuration         |
| `set_budget_config`          | Command | Create or update budget configuration    |
| `get_budget_dashboard`       | Query   | Get budget summary for dashboard widgets |
| `get_monthly_budget_records` | Query   | Get all 12 months for current year       |
| `get_quarterly_summaries`    | Query   | Get 5-year historical quarterly data     |
| `add_extra_budget`           | Command | Add one-time budget to a month           |
| `remove_extra_budget`        | Command | Remove extra budget entry                |
| `get_extra_budgets`          | Query   | List extra budgets for a year            |

---

## Command Specifications

### 1. `get_budget_config`

**Purpose**: Retrieve the current budget configuration.

**Args**: None (or empty criteria)

```typescript
// Request
invoke('get_budget_config', {});

// Response: BudgetConfigDto | null
interface BudgetConfigDto {
  id: number;
  mode: 'YEARLY' | 'MONTHLY';
  baseAmount: number; // Minor currency units (cents)
  monthlyAmount: number; // Calculated monthly amount
  yearlyAmount: number; // Calculated yearly amount
  currency: Currency;
  lastResetYear: number;
  createdAt: string; // ISO 8601
  updatedAt: string; // ISO 8601
  version: number;
}
```

**Errors**:

- None (returns `null` if not configured)

---

### 2. `set_budget_config`

**Purpose**: Create or update the budget configuration.

**Args**:

```typescript
interface SetBudgetConfigArgs {
  mode: 'YEARLY' | 'MONTHLY';
  baseAmount: number; // Minor currency units
  currency?: Currency; // Optional, inherits from settings if not provided
}
```

**Validation** (server-side):

- `mode` must be 'YEARLY' or 'MONTHLY'
- `baseAmount` must be >= 0
- `currency` must be valid ISO 4217 code

```typescript
// Request
invoke('set_budget_config', {
  args: {
    mode: 'YEARLY',
    baseAmount: 120000 // $1,200.00
  }
});

// Response: BudgetConfigDto
```

**Errors**:

- `ValidationError`: Invalid input values
- `DatabaseError`: Persistence failure

---

### 3. `get_budget_dashboard`

**Purpose**: Get budget summary for dashboard widgets (donut, bar, heatmap).

**Args**: None (uses current date context)

```typescript
// Request
invoke('get_budget_dashboard', {});

// Response: BudgetDashboardSummary | null
interface BudgetDashboardSummary {
  // Donut chart data
  remainingAmount: number;
  remainingPercentage: number; // 0.0 to 1.0
  totalAvailable: number;
  currency: Currency;

  // Bar chart data
  monthlySpending: MonthlySpendingPoint[]; // 12 items
  monthlyGoal: number; // Horizontal line value

  // Heatmap data
  quarterlyActivity: QuarterlyActivityPoint[]; // Up to 20 items
}

interface MonthlySpendingPoint {
  month: number; // 1-12
  amount: number;
  currency: Currency;
}

interface QuarterlyActivityPoint {
  year: number;
  quarter: 'Q1' | 'Q2' | 'Q3' | 'Q4';
  spendingLevel: 'NONE' | 'LOW' | 'MEDIUM' | 'HIGH';
  amount: number;
}
```

**Errors**:

- Returns `null` if budget not configured

---

### 4. `get_monthly_budget_records`

**Purpose**: Get detailed budget records for all 12 months of the current (or specified) year.

**Args**:

```typescript
interface GetMonthlyBudgetRecordsArgs {
  year?: number; // Optional, defaults to current year
}
```

```typescript
// Request
invoke('get_monthly_budget_records', { args: { year: 2026 } });

// Response: MonthlyBudgetRecord[]
interface MonthlyBudgetRecord {
  year: number;
  month: number; // 1-12
  baseBudget: number;
  extraBudget: number;
  actualSpend: number;
  rolloverIn: number;
  rolloverOut: number;
  status: 'PROJECTED' | 'IN_PROGRESS' | 'COMPLETED';
  available: number; // Computed: base + extra + rolloverIn
  remaining: number; // Computed: available - actualSpend
  remainingPercentage: number; // 0.0 to 1.0+
  currency: Currency;
}
```

**Errors**:

- `NotConfigured`: Budget not set up
- `ValidationError`: Invalid year

---

### 5. `get_quarterly_summaries`

**Purpose**: Get historical quarterly spending summaries for the past 5 years.

**Args**:

```typescript
interface GetQuarterlySummariesArgs {
  years?: number; // Optional, defaults to 5
}
```

```typescript
// Request
invoke('get_quarterly_summaries', { args: {} });

// Response: QuarterlySummary[]
interface QuarterlySummary {
  year: number;
  quarter: 'Q1' | 'Q2' | 'Q3' | 'Q4';
  totalSpending: number;
  currency: Currency;
  categoryBreakdown: CategorySpending[];
}

interface CategorySpending {
  category: Category; // From catalog domain
  amount: number;
  currency: Currency;
}
```

**Errors**:

- Returns empty array if no historical data

---

### 6. `add_extra_budget`

**Purpose**: Add a one-time budget injection to a specific month.

**Args**:

```typescript
interface AddExtraBudgetArgs {
  year: number;
  month: number; // 1-12
  amount: number; // Minor currency units, must be > 0
  reason?: string; // Optional description (max 500 chars)
}
```

**Validation** (server-side):

- `year` must be between 2000 and 2100
- `month` must be between 1 and 12
- `amount` must be > 0
- `reason` must be <= 500 characters if provided

```typescript
// Request
invoke('add_extra_budget', {
  args: {
    year: 2026,
    month: 3,
    amount: 5000, // $50.00
    reason: 'Birthday gift'
  }
});

// Response: ExtraBudgetEntry
interface ExtraBudgetEntry {
  id: string; // UUID
  year: number;
  month: number;
  amount: number;
  currency: Currency;
  reason: string | null;
  createdAt: string;
  version: number;
}
```

**Errors**:

- `NotConfigured`: Budget not set up
- `ValidationError`: Invalid input values

---

### 7. `remove_extra_budget`

**Purpose**: Remove an extra budget entry.

**Args**:

```typescript
interface RemoveExtraBudgetArgs {
  id: string; // UUID of the extra budget entry
}
```

```typescript
// Request
invoke('remove_extra_budget', {
  args: { id: '550e8400-e29b-41d4-a716-446655440000' }
});

// Response: void (success) or error
```

**Errors**:

- `NotFound`: Entry does not exist
- `DatabaseError`: Persistence failure

---

### 8. `get_extra_budgets`

**Purpose**: List all extra budget entries for a specific year.

**Args**:

```typescript
interface GetExtraBudgetsArgs {
  year: number;
}
```

```typescript
// Request
invoke('get_extra_budgets', { args: { year: 2026 } });

// Response: ExtraBudgetEntry[]
```

**Errors**:

- `ValidationError`: Invalid year

---

## Error Response Format

All commands follow the standard error format:

```typescript
interface CommandError {
  code: string;
  message: string;
  field?: string; // For validation errors
  details?: Record<string, unknown>;
}

// Error codes
type ErrorCode =
  | 'VALIDATION_ERROR'
  | 'NOT_FOUND'
  | 'NOT_CONFIGURED'
  | 'DATABASE_ERROR'
  | 'CURRENCY_MISMATCH';
```

---

## TypeScript Bindings

These types will be auto-generated in `src/lib/bindings.ts` via specta:

```typescript
// Auto-generated - DO NOT EDIT
export type BudgetMode = 'YEARLY' | 'MONTHLY';
export type MonthStatus = 'PROJECTED' | 'IN_PROGRESS' | 'COMPLETED';
export type SpendingLevel = 'NONE' | 'LOW' | 'MEDIUM' | 'HIGH';
export type Quarter = 'Q1' | 'Q2' | 'Q3' | 'Q4';

export interface BudgetConfigDto { ... }
export interface BudgetDashboardSummary { ... }
export interface MonthlyBudgetRecord { ... }
export interface QuarterlySummary { ... }
export interface ExtraBudgetEntry { ... }
export interface CategorySpending { ... }

// Args types
export interface SetBudgetConfigArgs { ... }
export interface GetMonthlyBudgetRecordsArgs { ... }
export interface GetQuarterlySummariesArgs { ... }
export interface AddExtraBudgetArgs { ... }
export interface RemoveExtraBudgetArgs { ... }
export interface GetExtraBudgetsArgs { ... }
```

---

## Rust Command Handlers

```rust
// src-tauri/src/budget/interface/command_handlers.rs

#[tauri::command]
#[specta::specta]
pub async fn get_budget_config(
    state: State<'_, AppState>,
) -> Result<Option<BudgetConfigDto>, CommandError> { ... }

#[tauri::command]
#[specta::specta]
pub async fn set_budget_config(
    args: SetBudgetConfigArgs,
    state: State<'_, AppState>,
) -> Result<BudgetConfigDto, CommandError> { ... }

#[tauri::command]
#[specta::specta]
pub async fn get_budget_dashboard(
    state: State<'_, AppState>,
) -> Result<Option<BudgetDashboardSummary>, CommandError> { ... }

#[tauri::command]
#[specta::specta]
pub async fn get_monthly_budget_records(
    args: GetMonthlyBudgetRecordsArgs,
    state: State<'_, AppState>,
) -> Result<Vec<MonthlyBudgetRecord>, CommandError> { ... }

#[tauri::command]
#[specta::specta]
pub async fn get_quarterly_summaries(
    args: GetQuarterlySummariesArgs,
    state: State<'_, AppState>,
) -> Result<Vec<QuarterlySummary>, CommandError> { ... }

#[tauri::command]
#[specta::specta]
pub async fn add_extra_budget(
    args: AddExtraBudgetArgs,
    state: State<'_, AppState>,
) -> Result<ExtraBudgetEntry, CommandError> { ... }

#[tauri::command]
#[specta::specta]
pub async fn remove_extra_budget(
    args: RemoveExtraBudgetArgs,
    state: State<'_, AppState>,
) -> Result<(), CommandError> { ... }

#[tauri::command]
#[specta::specta]
pub async fn get_extra_budgets(
    args: GetExtraBudgetsArgs,
    state: State<'_, AppState>,
) -> Result<Vec<ExtraBudgetEntry>, CommandError> { ... }
```
