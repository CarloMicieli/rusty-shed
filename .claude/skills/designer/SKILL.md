---
name: designer
description: Use this skill when building Tauri 2 views or Svelte 5 components. It enforces a "Mechanical Precision" aesthetic using a dynamic Steampunk palette (Iron/Copper or Parchment/Brass) centered on high-contrast data display and industrial textures.
---

### 🛠️ Agent Activation Instructions

Apply the [designer] skill whenever the user's request involves:

* **New View Creation:** Use a 3-column dashboard layout: `[Sidebar (Nav) | Main Content | Command Center (Quick Actions)]`. Use `bg-background` for the app base and `bg-card` for elevated panels.
* **Data Visualization:** Default to **Primary-on-Surface** (Amber/Copper on Charcoal) using circular progress rings (`variant-steampunk-gauge`) or high-contrast numeric cards.
* **Component Styling:** Use `border-border` (1px), `bg-card`, and `rounded-sm` (steampunk favors sharper, 2-4px corners over soft ones).
* **Technical Metadata:** Display physical specs (Scale, Road Number, DCC Address) using the **"Three-Column Footer"** pattern—`text-muted-foreground` uppercase labels over `text-foreground` monospaced values.
* **Svelte 5 Patterns:** Utilize `{#snippet}` for repeatable UI elements like Table Rows or Metadata Badges to ensure theme consistency.

---

### 🚦 Decision Logic (The "Theme-Variable" Check)

**CRITICAL:** Never use hex codes (e.g., #D48A42). Map all intent to Tailwind 4 theme variables:

1. **Is it a Brand Action?** Use `bg-primary` and `text-primary-foreground`.
2. **Is it a Secondary Detail?** Use `text-muted-foreground` and `font-mono` (JetBrains Mono) for numbers.
3. **Is it Interactive?** Apply `variant-steampunk-lever` for buttons to trigger the mechanical press effect.
4. **Is it a Container?** Use `variant-steampunk-riveted` for main dashboard panels to apply corner rivets.

---

### 1. The "Dynamic Accent" Rule

The action color (`--primary`) adapts based on the active `data-theme`.

* **Active States:** Use `bg-primary/15` for sidebar highlights with a `2px` solid `border-primary` on the left.
* **Buttons:** * *Primary:* `bg-primary text-primary-foreground` (Solid brass/copper).
    * *Secondary:* `border-primary text-primary hover:bg-primary/10`.
* **Mechanical Feel:** Use `transition-all duration-150 ease-out` to mimic physical switchgear.

### 2. Layout & Information Hierarchy

* **The "Card in Card" Look:** Use `bg-card` for main containers. For sub-grouping (e.g., "Yard Statistics"), use an inner `div` with `bg-background/50` and a `border-border`.
* **Technical Specs:** Labels must be `text-[10px] uppercase tracking-tighter text-muted-foreground`. Values must be `text-sm font-mono text-foreground`.
* **Status Badges:** Use the "Pill" style snippet. Position them `absolute top-2 right-2` on images using `bg-background/80` with a `border-primary`.

### 3. Empty States & Feedback

* **Center Aligned:** Keep empty states perfectly centered in their container.
* **Monochromatic Icons:** Use 2px stroke icons (Lucide or similar) in `text-muted-foreground`.
* **Toast/Signals:** Use `.toast-signal` for background errors, ensuring the `border-primary` or `border-warning` is visible.

---

## 🎨 Design Tokens (Tailwind 4 Variable Mapping)
- **Background:** `bg-background` (Mapped to `--background`)
- **Surface:** `bg-card` (Mapped to `--card`)
- **Border:** `border-border` (Mapped to `--border`)
- **Primary:** `bg-primary` (Mapped to `--primary` — Amber/Brass)
- **Primary-Muted:** `bg-primary/15` (For hovers and active states)
- **Text-Main:** `text-foreground` (Mapped to `--foreground`)
- **Text-Muted:** `text-muted-foreground` (Mapped to `--muted-foreground`)
- **Font-Heading:** `font-bebas` (Bebas Neue)
- **Font-Data:** `font-mono` (JetBrains Mono / Courier Prime)

---

## 🛠️ Component-Specific Instructions

### 1. Model Cards (Svelte Snippet)
- **Header:** Brand name in `font-bebas text-muted-foreground`.
- **Image:** Contained in a `border-b border-border` frame.
- **Footer:** A 3-column flex/grid. Labels: `text-[10px] text-muted-foreground`. Values: `text-xs font-mono`.

### 2. Dashboard Gauges
- **Circular Progress:** Use `stroke-primary` for the active value and `stroke-muted/20` for the background track.
- **Typography:** Numerical values inside gauges must be `font-mono`.

### 3. Interactive Inputs
- **Fields:** Use `bg-background` with `border-border`. 
- **Focus State:** On focus, use `ring-1 ring-primary` and `border-primary`.
- **Toggle/Valve:** Apply the `.variant-steampunk-valve` class for checkbox/switch inputs.
