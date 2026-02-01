# Quickstart: Digital Rolling Stock Management

**Feature**: 006-digital-rolling-stocks  
**Date**: 2026-01-30

## Prerequisites

- Rust toolchain (1.93.0+)
- Node.js (v20+) and pnpm
- SQLite development libraries
- Tauri CLI

## Development Setup

```bash
# 1. Clone and checkout feature branch
git checkout 006-digital-rolling-stocks

# 2. Install dependencies
pnpm install

# 3. Verify Rust setup
cd src-tauri && cargo check && cd ..

# 4. Run development mode
pnpm tauri dev
```

## Implementation Order

### Phase 1: Backend (Rust)

#### 1.1 Extend Views (`src-tauri/src/dcc_inventory/application/views.rs`)

Add new fields to `DigitalRollingStockView`:

```rust
// Add to DigitalRollingStockView struct
pub category: RollingStockCategory,
pub railway_company_name: Option<String>,
pub scale: Option<Scale>,
pub power_method: Option<PowerMethod>,
pub road_number: Option<String>,
pub series_code: Option<String>,
pub description: Option<String>,
```

#### 1.2 Add New View Types

Create `DigitalSummary` and `CheckDuplicateAddressResult` in views.rs.

#### 1.3 Extend Repository Query

Update `find_all_digital_rolling_stocks` in `sqlite_digital_rolling_stock_repository.rs` to:

- JOIN with catalog tables
- Filter out Function decoders
- Populate enriched fields

#### 1.4 Add New Repository Methods

```rust
// In DigitalRollingStockRepository trait
async fn get_digital_summary(&mut self) -> Result<DigitalSummary, DomainError>;
async fn check_address_exists(
    &mut self,
    address: DccAddress,
    exclude_id: Option<&DigitalRollingStockId>,
) -> Result<bool, DomainError>;
async fn find_installable_rolling_stocks(
    &mut self,
) -> Result<Vec<InstallableRollingStockView>, DomainError>;
```

#### 1.5 Add Use Cases

Create in `src-tauri/src/dcc_inventory/application/`:

- `get_digital_summary.rs`
- `get_decoders.rs`
- `check_duplicate_address.rs`
- `get_installable_rolling_stocks.rs`

#### 1.6 Add Tauri Commands

Update `src-tauri/src/dcc_inventory/interface/`:

- Add Args types in `command_args.rs`
- Add handlers in `command_handlers.rs`
- Register commands in `src-tauri/src/lib.rs`

#### 1.7 Generate Bindings

```bash
pnpm tauri dev  # Auto-regenerates bindings.ts
```

### Phase 2: Frontend (Svelte)

#### 2.1 Add Paraglide Messages

Update `messages/en.json` and `messages/it.json` with new keys.

```bash
pnpm prepare  # Regenerates paraglide files
```

#### 2.2 Create Feature Module

```bash
mkdir -p src/lib/features/digital-roster/components
```

Create:

- `DigitalRosterController.svelte.ts` - Svelte 5 runes controller
- `DigitalRosterState.svelte.ts` - Context provider
- `components/DigitalSummary.svelte`
- `components/DigitalRosterTable.svelte`
- `components/DccAddressEditor.svelte`
- `components/DecoderInstallDrawer.svelte`
- `index.ts` - Public exports

#### 2.3 Create Route

```bash
mkdir -p src/routes/my-digital-roster
```

Create:

- `+page.svelte`
- `+page.server.ts` (SSR stub)

#### 2.4 Update Navigation

Add nav items to:

- `src/lib/components/SidebarNavigation.svelte`
- `src/lib/components/BottomNavigation.svelte`

### Phase 3: Testing

#### Backend Tests

```bash
cd src-tauri
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

#### Frontend Tests

```bash
pnpm test
pnpm lint
pnpm check
```

## Verification Checklist

- [ ] `pnpm rust:check` passes
- [ ] `pnpm rust:clippy` passes (no warnings)
- [ ] `pnpm rust:test` passes
- [ ] `pnpm lint` passes
- [ ] `pnpm check` passes
- [ ] `pnpm test` passes
- [ ] New page accessible at `/my-digital-roster`
- [ ] Summary shows correct percentage
- [ ] Digital roster table displays and filters correctly
- [ ] DCC address change works with duplicate warning
- [ ] Decoder installation drawer opens and submits correctly
- [ ] All UI strings use Paraglide (no hardcoded text)

## Key Files Reference

| File                                                        | Purpose                    |
| ----------------------------------------------------------- | -------------------------- |
| `src-tauri/src/dcc_inventory/application/views.rs`          | View types                 |
| `src-tauri/src/dcc_inventory/interface/command_handlers.rs` | Tauri commands             |
| `src-tauri/src/lib.rs`                                      | Command registration       |
| `src/lib/bindings.ts`                                       | Generated TypeScript types |
| `src/lib/features/digital-roster/`                          | Feature module             |
| `src/routes/my-digital-roster/+page.svelte`                 | Page component             |
| `messages/en.json`                                          | English translations       |

## Common Issues

### Bindings not updating

```bash
pnpm tauri dev  # Restart to regenerate
```

### Missing Paraglide messages

```bash
pnpm prepare  # Regenerate message files
```

### SQLite foreign key errors

Ensure database migrations are up to date:

```bash
cd src-tauri && cargo sqlx migrate run
```
