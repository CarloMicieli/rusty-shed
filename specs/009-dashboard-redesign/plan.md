# Implementation Plan: Dashboard Redesign

**Branch**: `009-dashboard-redesign` | **Date**: 2026-01-30 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/009-dashboard-redesign/spec.md`

## Summary

Redesign the Dashboard page to provide a cohesive visual identity matching the My Collection view, with an integrated page title, streamlined Command Center with three quick actions, visual gallery for recently added models, functional Depot workspace with status badges, and widget-style statistics cards.

## Technical Context

**Language/Version**: TypeScript 5.x (Frontend), Rust (Backend - no changes needed)  
**Primary Dependencies**: Svelte 5, SvelteKit, Tailwind CSS 4, Skeleton UI  
**Storage**: N/A (uses existing DashboardState and data)  
**Testing**: Vitest (unit), Playwright (e2e - optional)  
**Target Platform**: Desktop/Mobile web (Tauri app)  
**Project Type**: Web application (Tauri 2 + Svelte 5)  
**Performance Goals**: Page render < 3 seconds, smooth scrolling  
**Constraints**: Must use Paraglide-JS for all user-facing strings  
**Scale/Scope**: Single page redesign, ~5 component updates

## Constitution Check

_GATE: Must pass before implementation._

- ✅ No new backend commands required (frontend-only changes)
- ✅ Uses existing data structures (DashboardDepotEntry, recentItems, totals)
- ✅ Follows existing component patterns from Collection view
- ✅ Paraglide-JS for all strings
- ✅ Svelte 5 Runes syntax

## Project Structure

### Documentation (this feature)

```text
specs/009-dashboard-redesign/
├── plan.md              # This file
├── spec.md              # Feature specification
├── checklists/          # Quality checklists
└── tasks.md             # Task breakdown (to be generated)
```

### Source Code (affected files)

```text
src/
├── routes/
│   └── my-dashboard/
│       └── +page.svelte              # Main dashboard page (MODIFY)
├── lib/
│   ├── components/
│   │   ├── StatsCard.svelte          # Stats widget (MODIFY for widget style)
│   │   ├── RecentItemCard.svelte     # Gallery card (MODIFY for larger visual)
│   │   ├── DepotView.svelte          # Depot section (MODIFY for status badges)
│   │   ├── DepotListCard.svelte      # Depot list item (MODIFY for status badges)
│   │   ├── DepotTable.svelte         # Depot table (MODIFY for status badges)
│   │   ├── QuickActionButtons.svelte # Quick actions (MODIFY for 3 buttons)
│   │   ├── PageHeader.svelte         # NEW: Reusable page header component
│   │   └── StatusBadge.svelte        # NEW: Color-coded status badge
│   └── features/
│       └── dashboard/
│           └── DashboardState.svelte.ts  # (no changes needed)
└── messages/
    ├── en.json                       # Add new translation keys
    └── it.json                       # Add new translation keys
```

**Structure Decision**: Frontend-only changes. No new routes, no backend modifications. Leverages existing Dashboard data fetching and state management.

## Component Design

### PageHeader Component (NEW)

- Displays page title with consistent styling matching Collection view
- Shows contextual breadcrumb/subtitle (e.g., "DASHBOARD / OVERVIEW")
- Optional description text
- Reusable across app pages

### StatusBadge Component (NEW)

- Color-coded badge for depot item status
- Supports: "in-service" (green), "under-repair" (orange)
- Small, inline display suitable for list/table rows
- **MVP Note**: `DashboardDepotEntry` lacks a `status` field. For MVP, all items show "In Service" as default. Future iteration will derive status from maintenance records.

### Command Center Section

- Renamed from "Quick Actions" to "Command Center"
- Contains three action buttons: Add to Collection, Add to Wishlist, Log Maintenance
- **Log Maintenance**: Shows toast "Maintenance logging coming soon" until maintenance feature is implemented

### Enhanced Components

1. **StatsCard**: Add subtle container styling to create "widget" appearance
2. **RecentItemCard**: Large visual cards with click navigation based on `source` field (Collection → `/my-collection/{id}`, Wishlist → `/my-wishlists/{id}`)
3. **DepotView/DepotListCard/DepotTable**: Integrate StatusBadge component (default "In Service" for MVP)
4. **QuickActionButtons**: Add third action for "Log Maintenance"

## Translation Keys Required

```json
{
  "dashboard_title": "Dashboard",
  "dashboard_subtitle": "DASHBOARD / OVERVIEW",
  "dashboard_description": "Your collection at a glance",
  "dashboard_command_center": "Command Center",
  "actions_log_maintenance": "Log Maintenance",
  "actions_maintenance_coming_soon": "Maintenance logging coming soon",
  "depot_status_in_service": "In Service",
  "depot_status_under_repair": "Under Repair"
}
```

## Complexity Tracking

No constitution violations. All changes follow existing patterns.
