---
name: mobile-design
description: Use this skill every time you need to design the mobile version of a UI component 
---

# Mobile Design 

### 1. Tactile Affordance & Interaction Design
The desktop application relies heavily on a physical, industrial feel. Mobile must adapt this without relying on hover states, leaning entirely into tactile touch feedback.
 * **The "Button" Rule:** Every clickable action on mobile must look like a physical trigger. Do not use plain text links or "ghost" buttons for primary actions. Ensure all mobile touch targets have defined boundaries.
 * **Mechanical Press Feedback:** Extend the active:scale-95 rule to simulate a physical mechanical lever. For secondary interactive cards or rows, use active:bg-muted/50 active:scale-[0.98] to provide immediate, heavy feedback when a user taps.
 * **Visual Saliency:** The primary accent color (#f97316 / primary) must be used sparingly to draw the user's eye to the most critical action on the screen, such as the FAB on the Collection and Wishlist pages.
### 2. Elevation & The "Anti-Bleed" Rule
On desktop, layers are thick and opaque. Mobile viewports naturally stack elements (like bottom sheets), making visual separation critical.
 * **Solid Elevations:** Background transparency must never compromise legibility. When converting the right-panel DrawerShell into a mobile bottom sheet, enforce a high-opacity backdrop (bg-black/80 or bg-background/90) combined with backdrop-blur-md.
 * **Distinct Sheet Layering:** For nested sheets (e.g., QuickAddShell opening over DrawerShell), rely on standard system tokens (bg-card and border-border) to create clear, overlapping physical layers rather than relying on drop shadows alone.
### 3. Mechanical Precision Typography
The desktop typography scale dictates the application's character. These rules must be strictly enforced at the mobile breakpoints to prevent the UI from looking like a generic web document.
 * **Headings (T1/T2):** Continue using font-bebas uppercase tracking-widest. Because Bebas Neue renders wide, do not apply it to anything smaller than 16 px (text-base) on mobile to maintain legibility.
 * **Data Values (T8):** Any exact data—such as Product Codes, Scales, Eras, or Financial Values—must use the font-mono token. This keeps tabular data readable even when using horizontal scroll on small screens.
 * **Labels:** Stick to text-xs uppercase tracking-wider text-muted-foreground. Do not hardcode arbitrary sizes like text-[9px]; always round up to the nearest token (text-[10px] minimum) to pass contrast and readability constraints.
### 4. Svelte 5 & Shadcn Architecture
To keep the codebase unified between desktop and mobile, the underlying component architecture must remain predictable.
 * **Component Composition:** When overriding shadcn components for mobile constraints (e.g., forcing a button to h-11), always use cn() or twMerge(). Never use string interpolation for classes in reusable primitives.
 * **Drawer Registry State:** When implementing the module-level DrawerRegistry for stacking bottom sheets, ensure the logic relies entirely on Svelte 5 runes ($state for the stack array, $derived for the depth/transform calculations). Avoid $effect blocks for visual state changes that can be derived directly from the stack array.
### Recommended Additions to Your Pre-Code Checklist
You can append these specific UI/UX verifications to the bottom of your existing checklist:
 * [ ] **Affordance Audit:** Verify no "naked" text links exist on mobile; all actions have an active:scale or background shift.
 * [ ] **Anti-Bleed Check:** Confirm DrawerShell and MoreMenu bottom sheets utilize opaque backgrounds (bg-card or bg-popover) and a blurred backdrop overlay to obscure main content.
 * [ ] **Typography Verification:** Check that all numeric data utilizes font-mono and that font-bebas is strictly reserved for text-base sizes or larger.
 * [ ] **Runes Architecture:** Ensure the DrawerRegistry handles sheet-stacking transforms via $derived logic without relying on $effect DOM mutations.
Would you like to drill down into the exact Tailwind classes and Svelte 5 component structure needed to build the DrawerRegistry for the nested mobile bottom sheets?
