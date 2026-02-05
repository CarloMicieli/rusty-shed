# Collection Overview UI Breakdown

## Layout Structure (NEW)

```
┌─────────────────────────────────────────────────────────────────┐
│  STICKY HEADER (sticky top-0 z-40)                              │
│ ┌──────────────────────────────────────────────────────────────┤
│ │ Collection  |  €68,853.60  |  |  507 Units                   │
│ │ My Collection                                                  │
│ └──────────────────────────────────────────────────────────────┤
│ │ [Locomotives] [Passenger] [Freight] [Train Sets] ...  →→→    │
│ └──────────────────────────────────────────────────────────────┤
│ │ [Search...    ] [🔍] [+ Add Model]                           │
│ └──────────────────────────────────────────────────────────────┘
│
│ MAIN CONTENT AREA (flex-1 overflow-y-auto)
│ ┌──────────────────────────────────────────────────────────────┐
│ │                                                               │
│ │  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐        │
│ │  │ Item 1  │  │ Item 2  │  │ Item 3  │  │ Item 4  │        │
│ │  └─────────┘  └─────────┘  └─────────┘  └─────────┘        │
│ │                                                               │
│ │  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐        │
│ │  │ Item 5  │  │ Item 6  │  │ Item 7  │  │ Item 8  │        │
│ │  └─────────┘  └─────────┘  └─────────┘  └─────────┘        │
│ │                                                               │
│ │  ... (scrollable)                                            │
│ │                                                               │
│ └──────────────────────────────────────────────────────────────┘
│
│ FILTER DRAWER (Sheet, right-side overlay, z-50)
│ ┌──────────────┐
│ │ FILTERS      │
│ │ ┌──────────┐ │
│ │ │ [Search] │ │
│ │ ├──────────┤ │
│ │ │ SCALES   │ │
│ │ │ [All] ...│ │
│ │ ├──────────┤ │
│ │ │ TAGS     │ │
│ │ │ [Tag 1]..│ │
│ │ └──────────┘ │
│ └──────────────┘
```

## Component Hierarchy

```
CollectionDashboard.svelte (Main Page)
├── Sticky Header
│   ├── Title Section (left)
│   ├── KPI Section (right)
│   │   ├── Collection Value
│   │   └── Total Units
│   ├── Horizontal Stat Chips (scrollable)
│   │   └── StatChip Snippet × 6
│   └── Search & Filter Bar
│       ├── Input (Search)
│       ├── Button (Filter icon)
│       └── Button (Add Model)
│
├── Main Content Area
│   ├── LoadingSkeleton Snippet
│   ├── EmptyState Snippet
│   ├── NoResults Snippet
│   └── Grid of ItemCard components
│
├── AddModelDrawer (Modal)
├── Sheet Component (Filter Drawer)
│   └── FilterPanel Component
│       ├── Search Input
│       ├── Scales Filter Section
│       └── Tags Filter Section
│
└── DeleteModal (Modal)
```

## Responsive Breakpoints

### Mobile (< 640px)
- Header: Single column
- Title stacked above KPIs
- Search bar full width
- Filter button visible
- Stat chips scroll horizontally (overflow-x-auto)
- Grid: `grid-cols-1` → 1 item per row
- Sheet drawer: `w-3/4` of screen width

### Tablet (640px - 1024px)
- Header: Two-column layout starts
- Title left, KPIs right
- Stat chips on separate line
- Grid: `sm:grid-cols-2` → 2 items per row
- Sheet drawer: `sm:w-96` (384px fixed)

### Desktop (1024px+)
- Header: Full multi-element layout
- All KPIs visible inline
- Horizontal stat chips fully visible
- Grid: `lg:grid-cols-3` → 3 items per row
- Extra large: `xl:grid-cols-4` → 4 items per row
- Recovered 280px width from removed sidebar

## Color Palette

### Backgrounds
- Page: `bg-surface-950` (darkest)
- Header: `bg-surface-900/95` with `backdrop-blur-sm`
- Stat Chips: `bg-surface-800/60`
- Content: transparent

### Text
- Page Title: `text-surface-50` (brightest)
- Labels: `text-surface-400` (muted)
- Values: `text-primary-200` (accent)
- Secondary: `text-surface-300` (light muted)

### Borders & Accents
- Primary borders: `border-surface-700/60`
- Hover state: `hover:border-primary-500/40`
- Accent: `text-accent-400` (buttons)

## Interactive Elements

### Sticky Header
- **Hover Effect:** None (informational only)
- **Behavior:** Stays fixed while content scrolls
- **Z-Index:** `z-40` (below modals)

### Stat Chips
- **Hover Effect:** `hover:border-primary-500/40` (border lightens)
- **Behavior:** Read-only, scrollable horizontally on small screens
- **Transition:** smooth `transition-colors`

### Search Input
- **Placeholder:** Collection search placeholder
- **Debounce:** 300ms for performance
- **Background:** `bg-surface-800`

### Filter Button
- **Icon:** Lucide `<Filter>` (20px)
- **Action:** Toggles Sheet drawer visibility
- **Hover:** `hover:bg-surface-800 transition-colors`

### Add Model Button
- **Style:** Primary variant with Icon + Text
- **Size:** `sm` (compact)
- **Action:** Opens AddModelDrawer modal

### Filter Drawer (Sheet)
- **Position:** Right-side overlay
- **Width:** `w-3/4` (mobile), `sm:w-96` (tablet+)
- **Z-Index:** `z-50` (above header)
- **Animation:** Smooth slide-in/out with `transition-transform`
- **Backdrop:** Semi-transparent black overlay that closes drawer on click
- **Dismiss:** Escape key or backdrop click

## State Management

### UI State (useCollectionUI)
```typescript
showDrawer: boolean          // Add/Edit model modal
showFilterSheet: boolean     // Filter drawer visibility
editing: CollectionItemView | null
confirmDeleteId: string | null
```

### Reactive Derived Values
```typescript
summaryData: CollectionSummary    // From service
totalValue: string                // Formatted currency
totalUnits: number                // Sum of all categories
filteredItems: array              // Search + filtered results
isLoading: boolean                // Loading state
filters: FilterState              // Current filter selections
```

## Performance Optimizations

1. **Grid Responsive**: `gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4`
   - Maximizes use of recovered horizontal space
   - Maintains visual balance across breakpoints

2. **Scrollable Stat Chips**: `overflow-x-auto pb-1 -mx-4 px-4`
   - Prevents layout shift on smaller screens
   - Negative margin balances padding for visual alignment

3. **Derived Values**: All computed values use `$derived` (not recalculated unless dependencies change)
   - `totalUnits` calculation
   - `totalValue` formatting

4. **Debounced Search**: 300ms debounce prevents excessive API calls

5. **Lazy Sheet Loading**: Filter drawer only renders when opened

## Accessibility Features

- ✅ Semantic HTML structure
- ✅ Proper heading hierarchy (h1 for page title)
- ✅ ARIA attributes on interactive elements
- ✅ Keyboard support: Escape closes Sheet drawer
- ✅ Focus management built into Sheet component
- ✅ Color contrast meets WCAG standards (light text on dark backgrounds)
- ✅ Touch targets meet 44x44px minimum (buttons with padding)

## Browser Support

- Modern browsers with ES2020+ support
- CSS Grid and Flexbox
- CSS custom properties (CSS variables)
- Backdrop filter support for header blur effect
