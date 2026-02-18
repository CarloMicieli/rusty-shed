---
name: designer
description: Use this skill when building Tauri-based, Svelte 5 views, or data grids that require a high-contrast, dark-mode interface centered around a charcoal-and-amber color palette, mechanical precision.
---

### 🛠️ Agent Activation Instructions

Apply the [designer] skill whenever the user's request involves:

* **New View Creation:** If the user asks for a new page (e.g., "Add a 'Repair History' screen" or "Build a 'Marketplace' view"), use the 3-column dashboard layout (Sidebar | Main Content | Command Center) or the Collection Grid layout.
* **Data Visualization:** When displaying stats, costs, or inventory counts, default to **Amber-on-Charcoal** bar charts, circular progress rings, or high-contrast numeric cards.
* **Component Styling:** If the user asks to "Style a button" or "Create a modal," use a `1px` border of `#1F1F1F`, a background of `#0F0F0F`, and `8px` rounded corners.
* **Inventory/Technical Details:** When displaying physical specs (Scale, Era, Road Number, DCC Address), format them in the **"Three-Column Footer"** pattern seen in the Collection cards—tiny muted headers over bright, centered values.
* **State Management UI:** For "Empty," "Loading," or "Error" states, use the centered monochromatic icon approach found in the *Digital (DCC)* and *Maintenance* screens.

---

### 🚦 Decision Logic (The "Style Check")

Add this logic to the agent's system prompt to maintain consistency:

1. **Is it a Brand Element?** If yes, use **Amber (`#D48A42`)** for primary actions and **Deep Charcoal (`#0F0F0F`)** for surfaces.
2. **Is it a Secondary Detail?** Use **Muted Gray (`#808080`)** and **Uppercase** typography for labels.
3. **Is it an Interactive Element?** Use a **Subtle Glow or 15% Opacity Amber** background for hover/active states in the sidebar and navigation.
4. **Is it a Model Attribute?** Always use the **Pill Badge** (Top Right) or the **Metadata Grid** (Bottom) to display it.

---

### 1. The "Amber" Accent Rule

The amber/copper color (`#D48A42`) is your only "action" color.

* **Active States:** Use a low-opacity amber background for sidebar items and a vertical 2px line on the left or a full rounded background.
* **Buttons:** Primary buttons are solid amber with black text. Secondary buttons are outlined with amber text.
* **Progress:** Circular charts and gauges use the amber color to represent "fullness" or "remaining value."

### 2. Layout & Information Hierarchy

* **The "Card in Card" Look:** Use cards to group logical data (e.g., "Yard Statistics"). Inside those cards, use subtle dividers or further sub-grouping with smaller, even darker containers.
* **Technical Specs:** When displaying model data (like Scale  or Era ), use a 3-column footer layout within the card. Label titles should be tiny, uppercase, and muted, while the values are bright and prominent.
* **Status Badges:** Use "Pill" style tags (like the **AC/DC** or **ERA** badges) in the top right of images. These should have a slight background color to pop against the imagery.

### 3. Empty States & Illustrations

* Keep empty states centered.
* Use thin-stroke (2px) monochromatic icons in the muted text color.
* The "Command Center" buttons on the right of the dashboard should remain consistent: a subtle dashed or solid border with a centered icon and text.

### 4. Data Visualization

* **Charts:** Use simple bar or line charts. Avoid gradients inside bars; keep them solid amber.
* **Gridlines:** Keep chart gridlines extremely faint () so they don't clutter the UI.

---

## 🎨 Design Tokens (Tailwind 4 / CSS)
- **Background (Base):** `#050505` (True Dark)
- **Surface (Cards/Modals):** `#0F0F0F` (Deep Charcoal)
- **Border:** `#1F1F1F` (Subtle separator)
- **Primary (Accent):** `#D48A42` (Amber/Copper)
- **Primary-Muted:** `rgba(212, 138, 66, 0.15)` (Active states/Hover)
- **Text-Main:** `#E0E0E0` (High readability)
- **Text-Muted:** `#808080` (Labels/Secondary info)

---

## 📐 Layout & Structure
- **Sidebar:** Fixed width, integrated with background. Active items use `primary-muted` background with a left-accent border or rounded-pill highlight.
- **Card Pattern:** All cards use `bg-[#0F0F0F]`, `border-[#1F1F1F]`, and `rounded-[8px]`. No drop shadows; use borders for depth.
- **Grid Layouts:** Collection items should follow a responsive grid. Each card must include a header (Brand/ID) and a metadata footer.
- **The "Command Center":** Right-hand utility column for quick actions (Add, Log, etc.) using vertical stacked buttons with distinctive icons.

---

## 🛠️ Component-Specific Instructions

### 1. Model Cards (Collection View)
- **Header:** Brand name (e.g., A.C.M.E.) in small caps/muted, Model name in bold.
- **Badges:** Use absolute positioning in the top-right corner for "AC/DC" or "Scale" badges (Pill shape).
- **Metadata Footer:** A 3-column flex/grid row. Labels are `text-[10px] uppercase text-[#808080]`. Values are `text-[12px] text-[#E0E0E0]`.

### 2. Data Viz (Dashboard)
- **Progress Rings:** Use `#D48A42` for the progress stroke and `#1F1F1F` for the trail.
- **Charts:** Bar charts use solid `#D48A42`. Grid lines must be barely visible (`#1F1F1F`).
- **Typography:** Use monospaced fonts (e.g., JetBrains Mono) for numerical values like "Road Number" or "DCC Address" to emphasize the mechanical feel.

### 3. Interactive Elements
- **Buttons:**
    - *Primary:* Solid `#D48A42` background, black text.
    - *Secondary/Ghost:* Border `#1F1F1F`, text `#E0E0E0`, hover background `primary-muted`.
- **Inputs:** Minimalist with `border-[#1F1F1F]`. On focus, border changes to `#D48A42`.

---