# Svelte 5 Page Component Hierarchy

This document provides a structural audit and component mapping for all "Page" components in the project (under `src/routes/`). Each page lists its custom Svelte component hierarchy down to the great-grandchild level, with summaries for each node.

---

## Dashboard (`src/routes/dashboard/+page.svelte`)

### Hierarchy

- **Dashboard Page**
  - **Location:** src/routes/dashboard/+page.svelte
  - **Summary:** Main dashboard view showing stats, quick actions, and purchase groups.
  - **Children:**
    - **PageHeader**  
       _Location:_ src/lib/components/PageHeader.svelte  
       _Summary:_ Renders the dashboard's main header.
    - **StatsCard**  
       _Location:_ src/lib/components/StatsCard.svelte  
       _Summary:_ Displays key statistics in card format.
    - **QuickActionButtons**  
       _Location:_ src/lib/components/QuickActionButtons.svelte  
       _Summary:_ Renders a set of quick action buttons for dashboard tasks.
    - **DashboardCharts**  
       _Location:_ src/lib/features/dashboard/DashboardCharts.svelte  
       _Summary:_ Visualizes dashboard data (charts/graphs).
    - **PurchaseGroupCard**  
       _Location:_ src/lib/features/dashboard/PurchaseGroupCard.svelte  
       _Summary:_ Shows grouped purchase information.
    - **DashboardAction**  
       _Location:_ src/lib/features/dashboard/components/DashboardAction.svelte  
       _Summary:_ Represents a single dashboard action.
    - **DashboardSectionHeader**  
       _Location:_ src/lib/features/dashboard/components/DashboardSectionHeader.svelte  
       _Summary:_ Section header for dashboard groupings.

---

## Error (`src/routes/error/+page.svelte`)

### Hierarchy

- **Error Page**
  - **Location:** src/routes/error/+page.svelte
  - **Summary:** Displays a signal failure/error state.
  - **Children:**
    - **SignalFailureView**  
       _Location:_ src/lib/components/signal-failure/SignalFailureView.svelte  
       _Summary:_ Renders error details and troubleshooting info.

---

## Search (`src/routes/search/+page.svelte`)

### Hierarchy

- **Search Page**
  - **Location:** src/routes/search/+page.svelte
  - **Summary:** Global search interface for models and wishlists.
  - **Children:**
    - **SearchResultCard**  
       _Location:_ src/lib/features/search/components/SearchResultCard.svelte  
       _Summary:_ Displays a single search result.
    - **SearchEmptyState**  
       _Location:_ src/lib/features/search/components/SearchEmptyState.svelte  
       _Summary:_ Shown when no search results are found.

---

## Finance (`src/routes/finance/+page.svelte`)

### Hierarchy

- **Finance Page**
  - **Location:** src/routes/finance/+page.svelte
  - **Summary:** Budget tracking and configuration.
  - **Children:**
    - **GaugeStatCard**  
       _Location:_ src/lib/components/GaugeStatCard.svelte  
       _Summary:_ Visualizes budget stats.
    - **BudgetConfigSheet**  
       _Location:_ src/lib/features/budget/components/BudgetConfigSheet.svelte  
       _Summary:_ Sheet/modal for configuring budget.
    - **BudgetMonthRow**  
       _Location:_ src/lib/features/budget/components/BudgetMonthRow.svelte  
       _Summary:_ Displays monthly budget data.
    - **ExtraBudgetModal**  
       _Location:_ src/lib/features/budget/components/ExtraBudgetModal.svelte  
       _Summary:_ Modal for adding extra budget.

---

## Digital DCC (`src/routes/digital-dcc/+page.svelte`)

### Hierarchy

- **Digital DCC Page**
  - **Location:** src/routes/digital-dcc/+page.svelte
  - **Summary:** Manages digital rolling stock and DCC addresses.
  - **Children:**
    - **DigitalSummary**  
       _Location:_ src/lib/features/digital-roster/components/DigitalSummary.svelte  
       _Summary:_ Summarizes digital roster stats.
    - **DigitalRosterTable**  
       _Location:_ src/lib/features/digital-roster/components/DigitalRosterTable.svelte  
       _Summary:_ Table of digital rolling stock.
    - **DecoderInstallDrawer**  
       _Location:_ src/lib/features/digital-roster/components/DecoderInstallDrawer.svelte  
       _Summary:_ Drawer for installing DCC decoders.

---

## Railway Tracks (`src/routes/railway-tracks/+page.svelte`)

### Hierarchy

- **Track Inventory Page**
  - **Location:** src/routes/railway-tracks/+page.svelte
  - **Summary:** Lists and manages track inventories.
  - **Children:**
    - **InventoryDetail**  
       _Location:_ src/lib/features/track-inventory/components/InventoryDetail.svelte  
       _Summary:_ Shows details for a selected inventory.
    - **TrackCommandBar**  
       _Location:_ src/lib/features/track-inventory/components/TrackCommandBar.svelte  
       _Summary:_ Command bar for inventory actions.
    - **CreateInventoryDialog**  
       _Location:_ src/lib/features/track-inventory/components/CreateInventoryDialog.svelte  
       _Summary:_ Dialog for creating new inventory.
    - **RenameInventoryDialog**  
       _Location:_ src/lib/features/track-inventory/components/RenameInventoryDialog.svelte  
       _Summary:_ Dialog for renaming inventory.
    - **DeleteInventoryDialog**  
       _Location:_ src/lib/features/track-inventory/components/DeleteInventoryDialog.svelte  
       _Summary:_ Dialog for deleting inventory.
    - **AddTracksPurchaseDrawer**  
       _Location:_ src/lib/features/track-inventory/components/AddTracksPurchaseDrawer.svelte  
       _Summary:_ Drawer for adding track purchases.

---

## Railway Track Detail (`src/routes/railway-tracks/[id]/+page.svelte`)

### Hierarchy

- **Track Inventory Detail Page**
  - **Location:** src/routes/railway-tracks/[id]/+page.svelte
  - **Summary:** Shows details for a specific track inventory.
  - **Children:**
    - **InventoryDetail**
    - **RenameInventoryDialog**
    - **DeleteInventoryDialog**
    - **AddTracksPurchaseDrawer**  
      _(See above for summaries; similar structure as main track inventory page.)_

---

## Train Formations (`src/routes/train-formations/+page.svelte`)

### Hierarchy

- **Train Formations Page**
  - **Location:** src/routes/train-formations/+page.svelte
  - **Summary:** Lists and manages train formations.
  - **Children:**
    - **FormationList**  
       _Location:_ src/lib/features/train-formations/components/FormationList.svelte  
       _Summary:_ Lists all train formations.

---

## Train Formation Detail (`src/routes/train-formations/[id]/+page.svelte`)

### Hierarchy

- **Train Formation Detail Page**
  - **Location:** src/routes/train-formations/[id]/+page.svelte
  - **Summary:** Edits a specific train formation.
  - **Children:**
    - **FormationBuilder**  
       _Location:_ src/lib/features/train-formations/components/FormationBuilder.svelte  
       _Summary:_ UI for building/editing a train formation.

---

## Maintenance (`src/routes/maintenance/+page.svelte`)

### Hierarchy

- **Maintenance Page**
  - **Location:** src/routes/maintenance/+page.svelte
  - **Summary:** Lists and manages maintenance cards.
  - **Children:**
    - **GaugeStatCard**
    - **PageHeader**
    - **MaintenanceCardList**  
       _Location:_ src/lib/features/maintenance/components/MaintenanceCardList.svelte  
       _Summary:_ Lists all maintenance cards.
    - **EmptyMaintenanceState**  
       _Location:_ src/lib/features/maintenance/components/EmptyMaintenanceState.svelte  
       _Summary:_ Shown when no maintenance cards exist.
    - **AddMaintenanceCardModal**  
       _Location:_ src/lib/features/maintenance/components/AddMaintenanceCardModal.svelte  
       _Summary:_ Modal for adding a new maintenance card.
    - **LogMaintenanceDrawer**  
       _Location:_ src/lib/features/maintenance/components/LogMaintenanceDrawer.svelte  
       _Summary:_ Drawer for logging maintenance events.

---

## Maintenance Detail (`src/routes/maintenance/[id]/+page.svelte`)

### Hierarchy

- **Maintenance Detail Page**
  - **Location:** src/routes/maintenance/[id]/+page.svelte
  - **Summary:** Shows and edits a specific maintenance card.
  - **Children:**
    - **MaintenanceEventTimeline**  
       _Location:_ src/lib/features/maintenance/components/MaintenanceEventTimeline.svelte  
       _Summary:_ Timeline of maintenance events.
    - **AddEventModal**  
       _Location:_ src/lib/features/maintenance/components/AddEventModal.svelte  
       _Summary:_ Modal for adding a maintenance event.

---

## Settings (`src/routes/settings/+page.svelte`)

### Hierarchy

- **Settings Page**
  - **Location:** src/routes/settings/+page.svelte
  - **Summary:** Application settings and cloud backup.
  - **Children:**
    - **SettingsForm**  
       _Location:_ src/lib/components/SettingsForm.svelte  
       _Summary:_ Main settings form.
    - **GoogleConnectButton**
    - **ConnectivityIndicator**
    - **SyncButton**
    - **BackupList**
    - **RestoreConfirmModal**
    - **ExportArchiveSection**  
      _(All above are under cloud-backup or export features; handle backup, sync, and export.)_

---

## Settings Import (`src/routes/settings/import/+page.svelte`)

### Hierarchy

- **Settings Import Page**
  - **Location:** src/routes/settings/import/+page.svelte
  - **Summary:** Import data into the app.
  - **Children:**
    - **PageHeader**
    - **ImportDropZone**
    - **ImportPreview**
    - **ImportReport**  
      _(All above are under import feature; handle file import, preview, and reporting.)_

---

## Depot (`src/routes/depot/+page.svelte`)

### Hierarchy

- **Depot Page**
  - **Location:** src/routes/depot/+page.svelte
  - **Summary:** Displays categorized depot inventory.
  - **Children:**
    - **DepotControls**
    - **DepotCategory**
    - **AddCollectionItemDrawer**
    - **EmptyState**  
      _(All above are under depot or collection features; handle depot controls, categories, and item addition.)_

---

## Collection Detail (`src/routes/collection/[itemId]/+page.svelte`)

### Hierarchy

- **Collection Item Detail Page**
  - **Location:** src/routes/collection/[itemId]/+page.svelte
  - **Summary:** Shows details for a specific collection item.
  - **Children:**
    - **RailwayModelCard**
    - **CollectionItemSidebar**  
      _(Show model details and sidebar for actions.)_

---

## Collection (`src/routes/collection/+page.svelte`)

### Hierarchy

- **Collection Page**
  - **Location:** src/routes/collection/+page.svelte
  - **Summary:** Dashboard for the user's collection.
  - **Children:**
    - **CollectionDashboard**  
       _Location:_ src/lib/features/collection/CollectionDashboard.svelte  
       _Summary:_ Main dashboard for collection overview.

---

## Wishlist Item Detail (`src/routes/wishlists/[wishlistId]/items/[itemId]/+page.svelte`)

### Hierarchy

- **Wishlist Item Detail Page**
  - **Location:** src/routes/wishlists/[wishlistId]/items/[itemId]/+page.svelte
  - **Summary:** Shows details for a specific wishlist item.
  - **Children:**
    - **RailwayModelCard**
    - **WishlistItemSidebar**
    - **PurchaseDialog**  
      _(Show model details, sidebar, and purchase dialog.)_

---

## Wishlists (`src/routes/wishlists/+page.svelte`)

### Hierarchy

- **Wishlists Page**
  - **Location:** src/routes/wishlists/+page.svelte
  - **Summary:** Dashboard for all wishlists.
  - **Children:**
    - **WishlistsDashboard**  
       _Location:_ src/lib/features/wishlists/WishlistsDashboard.svelte  
       _Summary:_ Main dashboard for wishlist overview.

---

**Note:**

- All components are Svelte 5, using Runes and feature-modular structure.
- Tauri 2 backend commands are invoked via `commands` or `@tauri-apps/api` in several pages (e.g., collection, wishlist, settings).
- Standard HTML elements are omitted; only custom Svelte components are mapped.
