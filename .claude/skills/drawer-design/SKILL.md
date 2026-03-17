---
name: drawer-design
description: Use this skill when designing drawer components, organizing UI elements by sections, implementing consistent design patterns, and ensuring a cohesive user experience.
---

## 🎨 Drawer Design Guidelines

To maintain the "Mechanical Precision" aesthetic, all drawers must adhere to these structural rules:

### 1. Header Architecture
* **Iconography:** Use a `24px` icon on the far left, containerized in a subtle `#1F1F1F` rounded box.
* **Typography:** The **Title** should be `text-amber-500 uppercase tracking-wider`. The **Subtitle** should be `text-muted-foreground text-sm` directly below it.
* **Close Action:** A standard "X" close button in the top right, using `text-[#808080]` and a circular hover state.

### 2. Input & Selection Components
* **Text Boxes:** Use the high-contrast variant. `bg-[#0F0F0F]`, `border-[#1F1F1F]`, and `rounded-[8px]`. Placeholder text must be `text-[#808080]` and use specific examples (e.g., "e.g., Class 218 Diesel Locomotive").
* **Dropdowns (General):** Use the "Floating Label" style. Labels should be small and muted. Scale labels must include the ratio in parentheses: `H0 (1:87)`.
* **Dropdowns (Sellers/Searchable):** Use the Command-menu style (from "Add Track Purchase"). It must include a search bar at the top and a checkmark for the active selection.
* **Price Input:** Right-aligned currency symbol (€). The input field should have a subtle inner shadow or a darker background than the drawer surface to indicate a numeric well.

### 3. Sectioning & Layout
* **Collapsible Sections:** Use the "Purchase Information" pattern—a full-width bar with a `1px` border, a chevron on the right, and uppercase `text-muted-foreground`.
* **Grid System:** Standardize on a 2-column grid for short fields (Scale/Power, Manufacturer/Product Code) and 1-column for long fields (Description).

### 4. Color Logic & Actions
* **Priority Buttons:** Use the tri-state amber toggle. **Low/High** are ghost buttons; **Normal/Active** is a solid `#D48A42` background with black text.
* **Footer Actions:** Primary "Save/Add" buttons are always in the bottom right, solid Amber. "Cancel" is a ghost button to the left of the primary action.
