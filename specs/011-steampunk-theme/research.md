# Research: Modern Steampunk Theme System

**Feature**: 011-steampunk-theme  
**Created**: 2026-01-30  
**Status**: Complete

## Research Tasks

### 1. Settings Schema Extension

**Question**: Does the existing `user_settings` table support the new theme field?

**Finding**: The current settings table (`0005_create_settings.sql`) has a fixed schema without a theme column:

```sql
CREATE TABLE IF NOT EXISTS settings (
  id                      INTEGER PRIMARY KEY CHECK (id = 1),
  currency                TEXT NOT NULL,
  length_unit             TEXT NOT NULL,
  favorite_scale          TEXT NOT NULL,
  favorite_power_method   TEXT NOT NULL,
  language_code           TEXT NOT NULL
);
```

**Decision**: Create migration `0007_add_theme_setting.sql` to add `theme TEXT NOT NULL DEFAULT 'system'`

**Rationale**:

- Follows existing migration pattern in project
- Uses TEXT to allow for future theme values
- Default 'system' matches expected behavior
- Single-row settings table (id=1) remains unchanged

**Alternatives Considered**:

- JSON column for flexible settings → Rejected: Over-engineering, existing fields are explicit
- Separate appearance_settings table → Rejected: Adds complexity, current pattern works

---

### 2. Skeleton UI 4.x Theming Best Practices

**Question**: How to implement custom themes in Skeleton UI 4.x without breaking defaults?

**Finding**: Skeleton 4.x uses CSS custom properties with a `data-theme` attribute on the document body. Themes are imported via CSS `@import` statements.

Current implementation in `layout.css`:

```css
@import '@skeletonlabs/skeleton';
@import '@skeletonlabs/skeleton/themes/cerberus';
```

**Decision**:

1. Create custom theme CSS files following Skeleton's token structure
2. Use `document.body.dataset.theme = 'steampunk-light' | 'steampunk-dark'` for switching
3. Define new `variant-steampunk-*` classes that don't conflict with Skeleton defaults
4. Import base Skeleton styles but replace cerberus with custom themes

**Rationale**:

- Follows Skeleton's documented theming approach
- `data-theme` switching is the official pattern
- Custom variants avoid breaking existing Skeleton components

**Alternatives Considered**:

- Override Skeleton defaults directly → Rejected: Fragile, breaks on Skeleton updates
- Use Tailwind-only theming → Rejected: Loses Skeleton component styling

---

### 3. Tailwind CSS 4 @theme Integration

**Question**: How does Tailwind CSS 4's `@theme` directive work with custom design tokens?

**Finding**: Tailwind 4 uses `@theme` to define CSS custom properties that integrate with utility classes:

```css
@theme {
  --color-primary-500: #b8860b;
  --breakpoint-md: 768px;
}
```

Current `layout.css` uses this pattern for breakpoints and LayerChart colors.

**Decision**:

- Define steampunk color tokens in `@theme` blocks within theme files
- Use theme-scoped selectors (`[data-theme="steampunk-light"]`) to switch token values
- Maintain existing `@theme` structure for breakpoints

**Rationale**:

- Native Tailwind 4 pattern already in use
- Theme-scoped selectors allow dynamic switching without JavaScript manipulation of CSS

---

### 4. Font Loading Strategy

**Question**: Best approach for loading Google Fonts (Cinzel Decorative, Courier Prime, Spectral)?

**Finding**: Current `app.html` doesn't include custom fonts. Google Fonts with `display=swap` is the standard approach.

**Decision**:

1. Add preconnect links for `fonts.googleapis.com` and `fonts.gstatic.com`
2. Load critical font (Courier Prime 400) synchronously
3. Load decorative fonts (Cinzel Decorative) with `display=swap`
4. Define fallback stacks in CSS custom properties

**Rationale**:

- `display=swap` prevents invisible text during load
- Preconnect speeds up font download
- Critical body font loaded first for readability

**Font Loading HTML**:

```html
<link rel="preconnect" href="https://fonts.googleapis.com" />
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
<link
  href="https://fonts.googleapis.com/css2?family=Cinzel+Decorative:wght@400;700&family=Courier+Prime:wght@400;700&family=Spectral:wght@400;500;600&display=swap"
  rel="stylesheet"
/>
```

---

### 5. CSS-Only Texture Performance

**Question**: Are CSS gradient textures performant on lower-end devices?

**Finding**: Complex CSS gradients (especially `repeating-linear-gradient` with many stops) can impact paint performance. The Tauri WebView uses system WebKit/Blink which handles gradients efficiently on desktop.

**Decision**:

1. Disable complex textures on mobile (`@media (max-width: 1023px)`)
2. Use `will-change: transform` sparingly on animated elements
3. Keep texture gradients simple (2-4 stops max for repeating patterns)
4. Apply textures to large surfaces only (body, sidebar), not every component

**Rationale**:

- Mobile devices have less GPU power for gradient rendering
- Limiting texture complexity prevents jank
- Desktop Tauri app can handle full texture experience

---

### 6. System Theme Detection

**Question**: How to detect and react to OS theme changes in Tauri?

**Finding**: Standard Web API `window.matchMedia('(prefers-color-scheme: dark)')` works in Tauri WebView. The `change` event fires when OS theme changes.

**Decision**:

```typescript
const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
const handleChange = (e: MediaQueryListEvent) => {
  if (currentTheme === 'system') {
    resolvedTheme = e.matches ? 'dark' : 'light';
  }
};
mediaQuery.addEventListener('change', handleChange);
// Cleanup on destroy
```

**Rationale**:

- Standard Web API, no Tauri-specific code needed
- Event listener pattern matches Svelte lifecycle

---

### 7. Existing Component Integration

**Question**: Which existing components need modification for theme integration?

**Finding**: Key components using color tokens:

- `SidebarNavigation.svelte` - Uses surface colors
- `BottomNavigation.svelte` - Uses primary colors
- `ItemCard.svelte` - Uses TAG_META gradients
- Charts (LayerChart) - Uses surface-\* custom properties in `layout.css`

**Decision**:

1. RivetedCard wraps existing card layouts, doesn't replace
2. Update TAG_META gradients in `src/lib/data/tags.ts` to use theme tokens
3. LayerChart colors already use CSS custom properties—update token values
4. Navigation components use Tailwind utilities that reference theme tokens

**Rationale**:

- Minimal changes to existing components
- Theme tokens flow through existing Tailwind utilities
- Chart compatibility maintained via CSS custom properties

---

### 8. Paraglide Localization

**Question**: How to add theme labels to Paraglide?

**Finding**: Messages are in `messages/en.json` and `messages/it.json` with flat key structure.

**Decision**: Add theme-related keys:

```json
{
  "settings_theme_label": "Theme",
  "settings_theme_system": "System",
  "settings_theme_light": "Parchment & Brass",
  "settings_theme_dark": "Iron & Copper"
}
```

**Rationale**:

- Follows existing flat key pattern
- Thematic names match design philosophy

---

## Summary of Decisions

| Topic                | Decision                                          | Migration/File                       |
| -------------------- | ------------------------------------------------- | ------------------------------------ |
| Settings storage     | Add `theme` column to settings table              | `0007_add_theme_setting.sql`         |
| Theme switching      | Use `data-theme` attribute on body                | `layout.css`, `themeStore.svelte.ts` |
| Skeleton integration | Custom theme files, `variant-steampunk-*` classes | `steampunk-*.css`                    |
| Fonts                | Google Fonts with preconnect + display=swap       | `app.html`                           |
| Textures             | CSS gradients, disabled on mobile                 | `steampunk-base.css`                 |
| System detection     | `matchMedia` API with event listener              | `themeStore.svelte.ts`               |
| Localization         | Flat Paraglide keys for theme names               | `messages/*.json`                    |
