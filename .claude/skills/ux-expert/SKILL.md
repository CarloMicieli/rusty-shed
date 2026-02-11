---
name: UX Expert
description: Use this skill when generating UI components to ensure they follow desktop-native usability standards.
---

# Rusty Shed Project Guidelines

## UX Philosophy: Desktop-Native, Not Web-Document

- **Affordance over Aesthetics:** Every interactive element must look like a physical button. No "naked" text links.
- **Elevation over Transparency:** Never use transparency that results in text-on-text bleeding. Use solid backgrounds (`bg-card` or `bg-zinc-950`) for all overlays.
- **Tauri Integration:** This is a desktop app. Use system-native patterns (standard title bars, predictable navigation, high contrast).

## Design System (Shadcn + Tailwind)

- **Primary Color:** Always use `primary` (Orange #f97316) for positive actions.
- **Modals:** Must include a solid, high-opacity backdrop (`bg-black/90`) and `backdrop-blur`.
- **Buttons:** Prefer `default`, `outline`, or `secondary` variants. Avoid `ghost` for core actions.

To make a web-based Tauri app feel like a native desktop application, the AI agent needs to move away from "web-document" thinking and toward "interface-application" thinking.

# "Desktop-First UX Architect"

**Core Objective:** Translate web-based Svelte/Tailwind code into a robust, high-affordance desktop experience that prioritizes legibility, tactile feedback, and structural integrity over "airy" web aesthetics.

## 1. Affordance & Signifiers (The "Button" Rule)

- **Principle:** If an element is interactive, it must have a clear "signifier". On desktop, users expect defined boundaries.
- **Constraint:** Never use plain text for primary or secondary actions. Every clickable action must use a `Button` component or a background-filled container with a `:hover` state change.
- **Tactile Feedback:** Apply `active:scale-95` or a subtle background darkening on click to simulate a physical button press.

## 2. Physicality vs. Transparency (The "Anti-Bleed" Rule)

- **Principle:** Background transparency (alpha) must never compromise legibility.
- **The Layering Rule:** Use **solid elevations**. If a Modal or Popover is open, the background must be obscured by a high-opacity overlay (`bg-black/80`) or a heavy backdrop-blur (`backdrop-blur-md`).
- **Consistency:** Use the Shadcn `Card` or `Dialog` background variables (`bg-card`, `bg-popover`) rather than custom `bg-opacity` classes. This ensures the "Rusty Shed" industrial feel remains solid and heavy, like steel, not light like a webpage.

## 3. Desktop Ergonomics & Navigation

- **Fitts’s Law:** Important actions (like "Add Railway Model") should be large or placed in consistent "anchors" (top-right or bottom-right).
- **Keyboard First:** Desktop users use shortcuts. The agent must always include `focus-visible:ring-2` for accessibility and suggest `Shortcut` tags (e.g., `⌘+S`) in tooltips or button labels.
- **Cursor States:** Ensure all interactive elements explicitly use `cursor-pointer`.

## 4. Visual Hierarchy (The "Krug" Method)

- **Don't Make Me Think:** The user should know exactly which part of the screen is "the active work area" versus "the navigation".
- **Instructions:**
- Use high-contrast headers for page titles (`text-white`).
- Use muted colors for metadata (`text-muted-foreground`).
- Primary actions must use the brand accent color (`#f97316`).

## Summary of UX Principles to Enforce

If you are using a "Coding Agent Skill," make sure it includes these two specific principles from usability textbooks:

| Principle           | Source                                   | Application in Rusty Shed                                                                                       |
| ------------------- | ---------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| **Signifiers**      | Don Norman (_Design of Everyday Things_) | Buttons must have a "3D" or bordered appearance so the user knows they can be pushed.                           |
| **Visual Saliency** | Steve Krug (_Don't Make Me Think_)       | Use your orange accent color to make the most important button on the screen "pop" against the dark background. |
