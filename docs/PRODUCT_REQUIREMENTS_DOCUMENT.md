# Product Requirements: Rusty Shed

## 1. Product Purpose

Rusty Shed is a unified digital workspace for railway collectors to manage the lifecycle of their hobby. It moves beyond simple lists to provide **confidence and clarity** through:

- **Centralized Records:** Every model, rolling stock detail, and purchase history in one place.
- **Seamless Transitions:** Moving items from wishlist to ownership while preserving context.
- **Proactive Management:** Integrated maintenance, budgeting, and data backups to prevent lost tasks or data.

---

## 2. User Experience & Interface

### Layout & Adaptability

The system maintains a consistent identity while optimizing for the user's device:

- **Desktop:** Features a persistent left sidebar for rapid navigation between all sections.
- **Mobile:** Prioritizes core tasks via a five-slot bottom bar, tucking secondary features into a "More" panel.
- **Contextual Editing:** Uses cards, side panels, and drawers to allow information entry without losing the user’s current place in the app.

### Product Feel

- **Fast & Direct:** In-place editing with automatic saving.
- **Immersive:** Uses collector-friendly terminology (e.g., "Signal Failure" for errors).
- **Calm:** Background operations (sync/search) run without interrupting active work.

---

## 3. Feature Pillars

### A. Collection & Inventory Management

- **Model Registry:** Add models with full technical data to the collection or wishlist.
- **Visual Browsing:** Use preview cards with thumbnails, badges, and status labels to scan the fleet.
- **Digital Roster:** Manage decoders and DCC addresses with filterable operational views.
- **Track & Depot:** Track bulk inventory (track pieces, accessories) with required quantities and stock levels.
- **Train Formations:** Create "consists" (train sets) with composition planning and readiness checks.

### B. Acquisition & Finance

- **Wishlist to Owned:** Track desired items with priority/price; convert to "Owned" with a single click.
- **Hobby Budgeting:** Set monthly/yearly limits with rollover tracking and spend visualization.
- **Acquisition History:** Review recent purchases grouped by event (date, seller, and batch notes).
- **Batch Entry:** Record multiple items at once via a single drawer with smart defaults.

### C. Maintenance & Operations

- **Maintenance Tracker:** A dedicated view for upcoming and overdue tasks, ranked by visual urgency.
- **Integrity Rules:** The system prevents duplicate maintenance cards per item to keep schedules clean.
- **Formatted Notes:** Write rich-text notes (bold, italics, lists) for specific model history or repair logs.

### D. Search & Organization

- **Global Search:** A single search bar to find items across both the collection and wishlist.
- **Categorized Browsing:** Filter the "Depot" by category with collapsible technical rows.
- **Localization:** Support for custom currencies, units (scale/power), and multi-language descriptions.

---

## 4. System Intelligence & Media

- **Smart Media:** Upload images via drag-and-drop; includes a built-in cropper to standardize photo framing.
- **Background Continuity:** Syncing and saving happen in the background with non-blocking status toasts.
- **Proactive Defaults:** The system remembers favorite scales, common sellers, and recent dates to speed up data entry.
- **Error Recovery:** Critical issues trigger a "Signal Failure" view with clear instructions on how to retry or seek support.

---

## 5. Data Privacy & Trust

- **Local First:** Core collection data stays on the user's device by default.
- **Cloud Sync:** Optional cloud backup only triggers after explicit account connection.
- **Portability:** Users can export their entire collection (including images) into a single archive for migration or safekeeping.
- **Data Integrity:** The system automatically prevents "orphaned" records (e.g., a maintenance log existing without a model).

---

> **Source Scope:** This specification is derived from technical requirements and architectural decisions (ADRs) to provide a non-technical overview of system behavior.

**Would you like me to create a "Minimum Viable Product" (MVP) checklist based on these reorganized features?**
