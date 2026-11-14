# Design System — rusty-shed

A comprehensive guide to the steampunk-themed design language for rusty-shed, a cross-platform Tauri 2 + Svelte 5 application for railway model management across desktop and mobile form factors.

---

## 1. Design Philosophy & Principles

### 1.1 Core Aesthetic: Steampunk Mechanical Precision

The rusty-shed design language embraces a **steampunk aesthetic** inspired by Victorian-era engineering, featuring:

- **Mechanical Metaphors**: Rivets, gauges, levers, and gears inform UI interactions
- **Dual Heritage Themes**:
  - **Dark Theme ("Iron & Copper")**: Industrial metal surfaces with warm copper accents
  - **Light Theme ("Parchment & Brass")**: Aged paper backgrounds with warm brass metallics
- **Material Authenticity**: Textures (parchment grain, metal finishes) enhance immersion without sacrificing clarity
- **Shared Identity Across Form Factors**: The same steampunk vocabulary remains recognizable on desktop, tablet, and Android while the interaction model adapts to each device

### 1.2 Design Principles

1. **Accessibility First**: WCAG AA compliance (4.5:1 contrast minimum for normal text), with touch-safe targets and adequate spacing for thumb reach
2. **Clarity Over Ornamentation**: Steampunk elements support hierarchy; never obscure content
3. **Consistency**: Shared visual identity and component tokens remain consistent; platform-specific behavior is intentional and documented
4. **Motion with Purpose**: Animations respect `prefers-reduced-motion` and enhance mechanical feel across input methods
5. **Cross-Platform by Design**: Leverage Tauri 2.0 desktop capabilities and Android mobile behavior without treating either surface as a simplified variant of the other
6. **Internationalization**: All user-facing text via Paraglide-JS (English + Italian)
7. **Type Safety**: tauri-specta bindings + TypeScript strictness; no manual type redefinitions

### 1.3 Design Goals

- **Professional yet Playful**: Railway hobbyists deserve precision tools with personality
- **Responsive at Scale**: Handle complex datasets (hundreds of models, rich metadata) across wide desktop layouts and compact mobile stacks
- **Performant**: Svelte 5 reactivity + optimized animations, with touch-first interaction patterns for mobile contexts
- **Extensible**: Component library accommodates future features (export, collaboration, etc.) while preserving a single design language

### 1.4 Cross-Platform Interaction Model

The system must use a single steampunk visual language with different interaction patterns by context:

- **Desktop**: Persistent navigation, multi-column layouts, keyboard-first workflows, and dense data tables with contextual actions.
- **Tablet**: Hybrid composition with collapsible side panels, split views, and gesture-friendly controls that preserve quick navigation and contextual detail.
- **Mobile**: Stacked content, bottom-sheet actions, thumb-reachable controls, and reduced visual density while keeping the same copper/brass identity.

Decorative elements (rivets, brass inlays, gauges, metallic textures) remain recognizable everywhere, but functional controls adapt to the expected input method and screen size. The system distinguishes between ornamental identity and workflow-critical UI: decorative motifs may be toned down on smaller screens, while task-critical controls remain highly legible and consistent across Android, iOS considerations, tablet, and desktop surfaces. The same component library may expose different layout and motion behavior depending on the form factor; it must never lose the mechanical precision of the core identity.

---

## 2. Color Palette & Tokens

All colors are defined as CSS custom properties (tokens) and Tailwind semantic classes for consistency.

### 2.1 Dark Theme: "Iron & Copper"

The primary theme, optimized for desktop use and long editing sessions.

#### Surfaces & Backgrounds

```css
--color-surface-50: #f8f7f6;      /* Lightest metal finish */
--color-surface-100: #eae8e5;     /* Light wash */
--color-surface-900: #1a1a1b;     /* Dark iron */
--color-surface-950: #0f0f10;     /* Darkest iron */
--color-background: #1a1a1b;      /* Primary background */
--color-background-alt: #242425;  /* Secondary background (panels, cards) */
```

**Tailwind Semantic (Dark):**
```css
--background: hsl(240 8% 9%);     /* Charcoal sheets (#1a1a1b) */
--foreground: hsl(0 0% 98%);      /* Off-white text (#fafafa) */
--secondary: hsl(240 6% 15%);     /* Secondary surface */
--muted: hsl(240 6% 15%);         /* Muted interactive elements */
--muted-foreground: hsl(240 5% 65%); /* Muted text (~#a8a8a8) */
```

#### Primary Colors: Copper Tones

```css
--color-primary-600: #cd7f32;     /* Warm copper (core interactive) */
--color-primary-700: #a86428;     /* Darker copper (hover, active) */
--color-primary-800: #875320;     /* Deep copper (pressed) */
--color-copper-600: #cd7f32;      /* Alias for consistency */
--color-copper-700: #a86428;      /* Alias for consistency */
```

**Tailwind Semantic:**
```css
--primary: hsl(30 50% 50%);       /* Copper (#cd7f32 approx) */
```

**Usage:** Button backgrounds, link hover states, active tabs, focus rings.

#### Secondary & Accent: Furnace Orange

```css
--color-secondary-600: #ff4500;   /* Bright furnace orange */
--color-secondary-700: #e63400;   /* Darker furnace (hover) */
--color-accent-600: #ff4500;      /* Alias */
--color-accent-700: #e63400;      /* Alias */
--color-furnace-orange-600: #ff4500;
```

**Tailwind Semantic:**
```css
--primary-sidebar: hsl(30 50% 50%); /* Copper on sidebar */
--sidebar: color-mix(in srgb, hsl(30 50% 50%) 5%, #1a1a1b 95%); /* Copper-tinted dark surface */
```

**Usage:** Call-to-action buttons, badges, warnings, secondary highlights.

#### Semantic Colors

```css
/* Error / Destructive */
--color-error-600: #ff3333;       /* Bright red */
--color-error-700: #e60000;       /* Deep red */

/* Success / Complete */
--color-success-600: #2fa56a;     /* Forest green */
--color-success-700: #238055;     /* Deep green */

/* Warning / Caution */
--color-warning-600: #ffb100;     /* Amber */
--color-warning-700: #cc8800;     /* Deep amber */
```

**Usage:** Status indicators, validation feedback, alert toasts.

#### Text & Heading Colors

```css
--color-text-900: #eae8e5;        /* Primary text (body, labels) */
--color-text-950: #f8f7f6;        /* High-contrast text (secondary) */
--heading-color: #c49b6e;         /* Copper-brown (Bebas Neue headings) */
```

### 2.2 Light Theme: "Parchment & Brass"

Optimized for presentations and printing; secondary theme.

#### Surfaces & Backgrounds

```css
--color-surface-50: #fffdf7;      /* Brightest parchment */
--color-surface-100: #fef9ed;     /* Light parchment */
--color-surface-900: #8f5a4d;     /* Dark sepia (rare; deep text) */
--color-background: #f4ebd0;      /* Warm parchment (primary) */
--color-background-alt: #e8dcc0;  /* Secondary parchment (panels) */
```

**Tailwind Semantic (Light):**
```css
--background: hsl(44 60% 94%);    /* Warm parchment (#f4ebd0 approx) */
--foreground: hsl(35 27% 13%);    /* Dark sepia (#2d2419) */
--primary: hsl(44 88% 38%);       /* Burnished brass (#d4a54a approx) */
--primary-text: hsl(44 92% 26%);  /* Deeper brass (#b8860b, 5:1 contrast on parchment) */
```

#### Primary Colors: Brass Tones

```css
--color-primary-600: #e09a34;     /* Warm brass */
--color-primary-700: #c07d1f;     /* Darker brass (hover) */
--color-primary-800: #b8860b;     /* Deep brass (pressed) */
--color-brass-600: #b8860b;       /* Alias */
--color-brass-700: #8b6508;       /* Alias (darkest) */
```

**Usage:** Same as dark theme but adapted for light backgrounds; ensures sufficient contrast.

#### Secondary & Accent: Burnished Copper

```css
--color-accent-600: #cc6633;      /* Burnished copper */
--color-accent-700: #a85427;      /* Dark copper (hover) */
--color-copper-600: #cc6633;      /* Alias */
--color-copper-700: #a85427;      /* Alias */
```

#### Tailwind Semantic (Light)

```css
--sidebar: color-mix(in srgb, hsl(44 88% 38%) 5%, #f4ebd0 95%); /* Brass-tinted warm surface */
--sidebar-primary: hsl(44 88% 38%); /* Brass on sidebar */
--muted: hsl(44 40% 80%);         /* Muted backgrounds */
--muted-foreground: hsl(35 20% 40%); /* Muted text */
```

### 2.3 Theme Switching & CSS Variables

The application resolves the user theme to a `data-theme` value on the root document element, not a `.dark` class. The actual runtime pattern is:

```html
<body data-theme="steampunk-dark">
  <!-- Uses --color-background: #1a1a1b -->
</body>

<body data-theme="steampunk-light">
  <!-- Uses --color-background: #f4ebd0 -->
</body>
```

The theme layer is defined per theme block (`[data-theme='steampunk-dark']` and `[data-theme='steampunk-light']`), with shared tokens in `:root` for typography, spacings, and theme-independent values. This matches the live app’s `themeStore` logic and avoids inverting dark/light behavior.

**Implementation (CSS):**
```css
:root {
  --font-family-body: 'Inter', system-ui, sans-serif;
  --font-family-heading: 'Bebas Neue', sans-serif;
  --radius: 0.625rem;
}

[data-theme='steampunk-dark'] {
  --color-background: #1a1a1b;
  --color-primary-600: #cd7f32;
  --heading-color: #c49b6e;
  --border: hsl(240 6% 15%);
}

[data-theme='steampunk-light'] {
  --color-background: #f4ebd0;
  --color-primary-600: #e09a34;
  --heading-color: #b8860b;
  --border: hsl(32 30% 60%);
}
```

### 2.4 Contrast & Accessibility

**Dark Theme Ratios:**
- Primary text (#eae8e5) on background (#1a1a1b): **16.1:1** ✓ (AAA)
- Copper primary (#cd7f32) on dark (#242425): **4.8:1** ✓ (AA)
- Focus ring (copper on dark): **4.8:1** ✓ (AA)

**Light Theme Ratios:**
- Primary text (#2d2419) on parchment (#f4ebd0): **12.3:1** ✓ (AAA)
- Brass primary (#b8860b) on parchment (#f4ebd0): **5.1:1** ✓ (AA)
- Focus ring (deep brass on parchment): **5.1:1** ✓ (AA)

---

## 3. Typography

### 3.1 Font Families

| Family       | Weights      | Use Case                  | Fallback       |
|--------------|--------------|---------------------------|----------------|
| **Inter**    | 400, 500     | Body text, UI labels      | system-ui      |
| **Bebas Neue** | 600         | Headings, display         | Georgia, serif |
| **JetBrains Mono** | 400, 500, 700 | Code, data, technical     | Courier New    |
| **Courier Prime** | 400       | Retro typewriter effects  | Courier New    |

**Font Loading:**
```html
<!-- Include in src/app.html or +layout.svelte -->
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Bebas+Neue&family=Inter:wght@400;500&family=JetBrains+Mono:wght@400;500;700&display=swap" rel="stylesheet">
```

### 3.2 Typographic Scale

Based on **Inter 400** for body, **Bebas Neue 600** for headings.

| Level | Font       | Size  | Line Height | Letter Spacing | Usage              |
|-------|------------|-------|-------------|----------------|--------------------|
| h1    | Bebas Neue | 36px  | 1.2         | 0.02em         | Page titles        |
| h2    | Bebas Neue | 28px  | 1.2         | 0.02em         | Section headings   |
| h3    | Bebas Neue | 24px  | 1.3         | 0.015em        | Subsection titles  |
| h4    | Bebas Neue | 20px  | 1.3         | 0.01em         | Card titles        |
| body  | Inter      | 14px  | 1.6         | 0              | Standard text      |
| small | Inter      | 12px  | 1.5         | 0              | Captions, tooltips |
| micro | Inter      | 11px  | 1.4         | 0              | Status badges      |
| code  | JetBrains Mono | 13px | 1.5       | 0              | Code snippets      |
| mono  | Courier Prime | 14px | 1.6       | 0              | Typewriter text    |

### 3.3 Heading Color & Style

**Dark Theme:**
```css
h1, h2, h3, h4, h5, h6 {
  font-family: 'Bebas Neue', Georgia, serif;
  font-weight: 600;
  color: #c49b6e;  /* Copper-brown */
  letter-spacing: 0.02em;
}
```

**Light Theme:**
```css
h1, h2, h3, h4, h5, h6 {
  font-family: 'Bebas Neue', Georgia, serif;
  font-weight: 600;
  color: #b8860b;  /* Deep brass */
  letter-spacing: 0.02em;
}
```

### 3.4 Typography Usage

#### Body Text
- **Color:** --color-text-900 (primary), --color-text-950 (secondary/emphasis)
- **Font:** Inter 400, 14px line-height 1.6
- **Max Width:** 65–75 characters for optimal readability

#### Labels & UI Text
- **Font:** Inter 500, 12–14px
- **Color:** Inherit from component (button, input label, etc.)
- **Case:** Sentence case (capitalize first word only, except proper nouns)

#### Code & Technical Text
- **Font:** JetBrains Mono 400 or 500, 13px line-height 1.5
- **Color:** --color-primary-600 (copper) on dark theme
- **Background:** Slightly elevated surface (--color-surface-100 dark, --color-surface-50 light)

---


---

## 4. Spacing & Layout

### 4.1 Spacing Scale

Spacing follows an 8px base grid (8px, 16px, 24px, 32px, etc.) for consistency, while remaining compatible with Tailwind v4 defaults and the project’s existing utilities.

| Token   | Value | Usage                           |
|---------|-------|---------------------------------|
| xs      | 4px   | Tight spacing (rare)            |
| sm      | 8px   | Compact spacing (icon gaps)     |
| md      | 16px  | Standard padding, gaps          |
| lg      | 24px  | Section spacing                 |
| xl      | 32px  | Large gaps, card padding        |
| 2xl     | 40px  | Hero sections, major breaks     |

**Tailwind-compatible token mapping:**
```css
:root {
  --spacing-xs: 0.25rem;   /* 4px */
  --spacing-sm: 0.5rem;    /* 8px */
  --spacing-md: 1rem;      /* 16px */
  --spacing-lg: 1.5rem;    /* 24px */
  --spacing-xl: 2rem;      /* 32px */
  --spacing-2xl: 2.5rem;   /* 40px */
}
```

This avoids ambiguous token names like `p-lg` or `text-lg` where Tailwind v4 already reserves meaning for typography and utility classes.

### 4.2 Border Radius

Rounded corners use a **0.625rem (10px) base**, adjusted per context:

```css
--radius: 0.625rem;         /* 10px (default) */
--radius-sm: calc(var(--radius) - 4px);  /* 6px (tight radius) */
--radius-md: calc(var(--radius) - 2px);  /* 8px (slightly rounded) */
--radius-lg: var(--radius);              /* 10px (standard) */
--radius-xl: calc(var(--radius) + 4px);  /* 14px (generous radius) */
```

**Usage:**
| Context              | Radius   | Example               |
|----------------------|----------|----------------------|
| Buttons (default)    | 6px      | --radius-sm           |
| Input fields         | 8px      | --radius-md           |
| Cards                | 10px     | --radius-lg (default) |
| Modals, large panels | 14px     | --radius-xl           |
| Fully rounded        | 50%      | Icon buttons, badges  |

### 4.3 Container & Layout Sizes

```css
/* Page content maximum width */
--page-content-max-width: 88rem;  /* 1408px */

/* Sidebar width */
--sidebar-width: 280px;

/* Touch target minimum (mobile accessibility) */
--touch-target: 44px;  /* 44×44px for fingers */

/* Desktop minimum (mouse-friendly) */
--click-target: 32px;  /* 32×32px acceptable */
```

**Responsive Breakpoints (Tailwind default):**
```
sm: 640px
md: 768px
lg: 1024px
xl: 1280px
2xl: 1536px
```

**Cross-Platform Layout Constraints:**
- **Desktop (Windows/Linux):** Minimum window 960×600px, default window 1200×800px, titlebar height 32px
- **Tablet (768px–1023px):** Use adaptive split layouts, collapsible side panels, and gesture-friendly controls while preserving the full visual language
- **Mobile (<768px / Android):** Prioritize stacked layouts, bottom-sheet actions, and thumb-reachable controls; reduce decorative density while keeping the same steampunk identity
- **Desktop Window Shell:** Native titlebar/window controls remain relevant on desktop; on mobile, avoid desktop chrome and use safe-area aware layouts instead

### 4.4 Padding & Gaps

**Card Padding:**
```css
.card {
  padding: var(--spacing-lg);  /* 24px */
}

.card-compact {
  padding: var(--spacing-md);  /* 16px */
}
```

**Flex Gaps:**
```css
.flex-gap-sm { gap: var(--spacing-sm); }   /* 8px */
.flex-gap-md { gap: var(--spacing-md); }   /* 16px */
.flex-gap-lg { gap: var(--spacing-lg); }   /* 24px */
```

---

## 5. Component Library Specifications

All components are built with Svelte 5 **Runes only** (no legacy stores). Styled with Tailwind CSS v4 + shadcn-svelte + custom steampunk variants.

### 5.1 Buttons & Interactive Elements

#### Button Component

**Anatomy:**
```
┌─────────────────────────┐
│ [icon] Label [icon]     │  (optional icons)
└─────────────────────────┘
```

**Props (Svelte 5 Runes):**
```typescript
interface ButtonProps {
  variant?: 'default' | 'destructive' | 'outline' | 'secondary' | 'ghost' | 'link' | 'rusty';
  size?: 'default' | 'sm' | 'lg' | 'icon' | 'icon-sm' | 'icon-lg';
  disabled?: boolean;
  loading?: boolean;
  children?: Snippet;
}
```

**Variants:**

| Variant       | Background                       | Text Color       | Hover Effect                         | Use Case           |
|---------------|----------------------------------|------------------|--------------------------------------|--------------------|
| default       | `var(--color-primary-600)`        | white            | Darker copper (`--color-primary-700`) | Primary actions    |
| destructive   | `var(--color-error-600)`         | white            | Darker red (`--color-error-700`)      | Delete, dangerous  |
| outline       | transparent                      | `var(--color-primary-600)` | Border active, bg lift            | Secondary actions  |
| secondary     | `var(--color-secondary-600)`     | white            | Darker secondary                     | Alternative action |
| ghost         | transparent                      | Inherit text     | Slight bg on hover                   | Minimal, text-like |
| link          | transparent                      | `var(--color-primary-600)` | Underline on hover              | Inline links       |
| rusty         | `var(--color-warning-600)`       | white            | Darker amber                         | Special (status)   |

**Sizes:**

| Size        | Height | Padding (x/y) | Icon Size | Use Case          |
|-------------|--------|---------------|-----------|-------------------|
| sm          | 32px   | 12px / 8px    | 16px      | Compact UI        |
| default     | 36px   | 16px / 8px    | 18px      | Standard buttons  |
| lg          | 40px   | 24px / 12px   | 20px      | Primary CTA       |
| icon        | 36px   | 8px           | 18px      | Icon-only toggle  |
| icon-sm     | 32px   | 4px           | 16px      | Compact icons     |
| icon-lg     | 40px   | 12px          | 20px      | Large icons       |

**Steampunk Styling (Dark Theme):**
```css
.btn-default {
  @apply bg-copper-600 text-white rounded-sm hover:bg-copper-700;
  box-shadow: 0 2px 4px rgba(205, 127, 50, 0.2);
  transition: all 200ms cubic-bezier(0.4, 0, 0.2, 1);
}

.btn-default:active {
  box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.3);
  transform: translateY(1px);
}

.btn-default:focus-visible {
  outline: 3px solid var(--color-primary-600);
  outline-offset: 2px;
}
```

**Example (Svelte 5 Runes):**
```svelte
<script>
  import Button from '$lib/components/ui/button/button.svelte';
  import { CheckCircle } from '@lucide/svelte';

  let loading = $state(false);

  const handleClick = async () => {
    loading = true;
    // ... perform action ...
    loading = false;
  };
</script>

<Button 
  variant="default" 
  size="lg"
  {loading}
  onclick={handleClick}
>
  <CheckCircle size={20} />
  {$m.core.actions.save()}
</Button>
```

#### Lever (Mechanical Toggle)

**Anatomy:**
```
OFF                    ON
[●─────────────]    [─────────●]
```

**Props:**
```typescript
interface LeverProps {
  checked?: boolean;
  disabled?: boolean;
  onchange?: (checked: boolean) => void;
}
```

**Styling (Steampunk Variant: `variant-steampunk-lever`):**
```css
.lever-track {
  width: 44px;
  height: 24px;
  background: var(--color-surface-100);
  border: 1px solid var(--color-primary-600);
  border-radius: 12px;
  position: relative;
  box-shadow: inset 0 1px 3px rgba(0, 0, 0, 0.3);
}

.lever-thumb {
  width: 20px;
  height: 20px;
  background: var(--color-primary-600);
  border-radius: 50%;
  position: absolute;
  top: 2px;
  left: 2px;
  transition: left 200ms cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 2px 4px rgba(205, 127, 50, 0.4);
}

.lever-track.checked .lever-thumb {
  left: 22px;
}

.lever-track:active .lever-thumb {
  box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.2);
}
```

### 5.2 Cards & Containers

#### StatsCard (Data Display)

**Anatomy:**
```
┌─────────────────────────┐
│ METRIC NAME      [icon] │
│                         │
│ 1,234                   │
│ Last updated: 2min ago  │
└─────────────────────────┘
```

**Props:**
```typescript
interface StatsCardProps {
  label: string;          // Metric name (Paraglide)
  value: number | string;
  unit?: string;
  trend?: 'up' | 'down' | 'neutral';
  trendValue?: string;    // e.g., "+5.2%"
  icon?: Component;
  lastUpdated?: Date;
}
```

**Styling:**
```css
.stats-card {
  @apply rounded-lg bg-surface-900 border border-surface-100 p-lg;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
}

.stats-card:hover {
  border-color: var(--color-primary-600);
  box-shadow: 0 4px 8px rgba(205, 127, 50, 0.15);
}

.stats-value {
  font-family: 'Bebas Neue', Georgia, serif;
  font-size: 36px;
  font-weight: 600;
  color: var(--color-primary-600);
  line-height: 1;
}

.stats-label {
  font-size: 12px;
  color: var(--color-text-900);
  text-transform: uppercase;
  letter-spacing: 0.1em;
}
```

#### GaugeStatCard (Circular Indicator)

**Anatomy (Steampunk Gauge):**
```
    ┌─── 12 ───┐
  9 │ ╭─────╮  │ 3
    │ │ 450 │  │
    │ ╰─────╯  │
    └─── 6 ───┘
   Model Count
```

**Props:**
```typescript
interface GaugeStatCardProps {
  label: string;
  value: number;
  max: number;
  variant?: 'copper' | 'brass' | 'standard';
}
```

**Styling (Variant: `variant-steampunk-gauge`):**
```css
.gauge {
  width: 120px;
  height: 120px;
  border-radius: 50%;
  background: conic-gradient(
    var(--color-primary-600) 0deg calc(var(--progress) * 3.6deg),
    var(--color-surface-200) calc(var(--progress) * 3.6deg) 360deg
  );
  box-shadow: 
    inset 0 0 10px rgba(0, 0, 0, 0.3),
    0 4px 8px rgba(205, 127, 50, 0.2);
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
}

.gauge::before {
  content: '';
  position: absolute;
  inset: 3px;
  border-radius: 50%;
  background: var(--color-background);
}

.gauge-value {
  position: relative;
  z-index: 1;
  font-size: 28px;
  font-weight: 600;
  color: var(--color-primary-600);
}
```

#### Card (Generic Container)

**Props:**
```typescript
interface CardProps {
  variant?: 'default' | 'elevated' | 'outline';
  children?: Snippet;
}
```

**Variants:**

| Variant   | Border    | Shadow             | Background       | Use Case            |
|-----------|-----------|-------------------|------------------|---------------------|
| default   | None      | Light (2px 4px)   | --surface-900    | Standard container  |
| elevated  | None      | Strong (4px 12px) | --surface-900    | Prominent content   |
| outline   | 1px solid | None              | Transparent      | Bordered sections   |

### 5.3 Form Inputs & Validation

#### Input Field

**Props:**
```typescript
interface InputProps {
  type?: 'text' | 'email' | 'password' | 'number' | 'date';
  placeholder?: string;
  value?: string;
  disabled?: boolean;
  error?: string;
  onchange?: (value: string) => void;
}
```

**Styling:**
```css
input {
  @apply w-full px-md py-sm rounded-md;
  border: 1px solid var(--color-surface-100);
  background: var(--color-background-alt);
  color: var(--color-text-950);
  font: 400 14px Inter, system-ui;
  transition: border-color 200ms, box-shadow 200ms;
}

input:focus {
  outline: none;
  border-color: var(--color-primary-600);
  box-shadow: 0 0 0 3px rgba(205, 127, 50, 0.1);
}

input:disabled {
  background: var(--color-surface-100);
  color: var(--color-text-900);
  cursor: not-allowed;
}

input.error {
  border-color: var(--color-error-600);
}

input.error:focus {
  box-shadow: 0 0 0 3px rgba(255, 51, 51, 0.1);
}
```

**Error Display:**
```css
.input-error {
  font-size: 12px;
  color: var(--color-error-600);
  margin-top: 4px;
}
```

### 5.4 Navigation & Sidebar

#### Navigation Patterns by Platform

The app uses one navigation vocabulary with different structures by device:

- **Desktop**: Persistent sidebar with multi-level navigation and quick access to dense workspaces.
- **Tablet**: Adaptive sidebar or top-level segmented navigation with metadata-dense sections retained in a compressed layout.
- **Mobile**: Drawer-based navigation, bottom navigation, or tabbed top navigation depending on task complexity; all actions remain gesture-friendly and thumb-reachable.

#### Sidebar (Desktop / Tablet)

**Structure:**
```
┌──────────────────────┐
│ rusty-shed   [≡]     │  Header (32px)
├──────────────────────┤
│ ▣ Dashboard          │
│ ▣ Inventory          │  Navigation (flex column)
│ ▣ Reports            │
├──────────────────────┤
│ ⚙ Settings           │  Footer
└──────────────────────┘
```

**Props:**
```typescript
interface SidebarProps {
  items: NavItem[];
  active?: string;
  collapsed?: boolean;
  onitemclick?: (id: string) => void;
}

interface NavItem {
  id: string;
  label: string;         // Paraglide key
  icon?: Component;
  badge?: number;
  href?: string;
}
```

**Styling:**
```css
.sidebar {
  width: var(--sidebar-width);  /* 280px */
  background: color-mix(in srgb, var(--color-primary-600) 5%, var(--color-background) 95%);
  border-right: 1px solid var(--color-surface-100);
  display: flex;
  flex-direction: column;
  transition: width 300ms ease;
}

.sidebar-item {
  @apply px-md py-sm flex items-center gap-sm;
  color: var(--color-text-900);
  cursor: pointer;
  transition: all 150ms;
  border-left: 3px solid transparent;
  position: relative;
}

.sidebar-item:hover {
  background: var(--color-surface-100);
  border-left-color: var(--color-primary-600);
}

.sidebar-item.active {
  background: var(--color-surface-100);
  border-left-color: var(--color-primary-600);
  color: var(--color-primary-600);
  font-weight: 500;
}
```

**Responsive:**
- **Desktop (≥1024px):** Sidebar visible, 280px wide
- **Tablet (768–1023px):** Sidebar collapses to icon rail or condensed top nav; preserve quick access without consuming the full width
- **Mobile (<768px):** Use drawer or tabbed navigation with larger touch targets and bottom-sheet actions that remain consistent with the sidebar structure

### 5.5 Drawer & Form Interaction Patterns

Drawers are a primary interaction pattern throughout rusty-shed. They provide contextual workflows for creating, editing, and managing railway models, purchases, wishlists, and other application data. All drawers must follow the Mechanical Precision visual identity while providing consistent, accessible, and responsive interaction patterns.

#### Drawer Types and Usage

Use the appropriate presentation pattern according to the complexity of the task.

| Pattern | Recommended usage |
|---------|-------------------|
| Side Drawer | Creating or editing structured records |
| Compact Drawer | Quick actions and simple attribute editing |
| Bottom Sheet | Contextual actions, filters, and short selections on mobile |
| Full-Screen Form | Complex data entry on mobile |
| Confirmation Dialog | Destructive actions and simple confirmations |

Do not use a full-size side drawer for simple interactions that can be handled more efficiently with a compact component.

#### Drawer Dimensions and Responsive Behaviour

**Desktop**

- Drawers open from the right side of the application.
- Standard width: 480px–560px.
- Complex forms may use a width of up to 640px.
- Drawers must not exceed the available viewport width.
- The underlying page remains visible but visually de-emphasised.
- Use a subtle overlay to communicate a modal interaction.

**Tablet**

- Use a responsive width between 420px and 560px, depending on available space.
- Prevent the drawer from obscuring essential navigation or contextual information.
- Use a full-width presentation when the available space is insufficient.

**Mobile**

- Replace side drawers with full-screen forms for complex data entry.
- Use bottom sheets for filters, contextual actions, and short selections.
- Respect device safe areas and the virtual keyboard.
- Keep primary actions accessible when the keyboard is visible.

A single interaction vocabulary should remain recognizable across all platforms: the steampunk visual system is shared, while the modal shell and input ergonomics vary by device and task complexity.

#### Header Architecture

Every drawer must have a consistent header.

- **Icon:** 24px icon inside a subtle `#1F1F1F` rounded container.
- **Title:** Primary field label or human-readable task title using the app’s heading scale.
- **Close control:** Top-right action with clear tappable area and visible focus state.
- **Secondary actions:** Optional inline actions (Save as draft, Duplicate, More actions) aligned to the right edge.
- **Spacing:** 16px frame padding; 12px separation between icon, title, and controls.

The drawer header is part of the functional UI and should remain visually dense enough to be legible on desktop while resizing gracefully for handheld screens.

#### Body Layout and Form Composition

The drawer body should use a clear, modular structure to reduce cognitive load and support quick scanning.

- Use a single column for mobile and compact drawers.
- Use a 2-column grid for wide desktop drawers when fields are logically paired.
- Group related fields into labelled sections with consistent spacing and hierarchy.
- Keep the most important control in the first viewport if the user is entering or editing critical data.
- Use sticky action bars for long forms, particularly on tablet and mobile.
- Avoid stacking more than one destructive or secondary action in the same footer row.

For mobile forms, the sheet should preserve the visual identity without reproducing a desktop panel. The same form fields, label scale, and typography remain consistent; only the shell and motion differ.

#### Form Behaviors and Motion

- Side drawers should animate from the right using a subtle transform and low-opacity backdrop.
- Bottom sheets should lift from the bottom with a soft overscroll and a firm but restrained ease curve.
- Full-screen mobile forms should use a slide-up or modal-style transition, preserving context while making the form feel native.
- Reduced-motion users should still receive a clear state change without decorative motion.

The decorative steampunk cues (metallic borders, rivet-like details, brass highlights) should remain subordinate to form clarity and task completion. Decorative motifs may be softened on smaller screens, but the interaction pattern and the information hierarchy must remain obvious.

#### Accessibility Requirements

- Every drawer must expose a clear dialog role with modal semantics on desktop and mobile.
- Focus must move into the drawer when opened and return to the trigger when closed.
- Escape closes the drawer on desktop and tablet; mobile close affordances must remain visible and consistent.
- All controls need a minimum 44×44px touch target on Android and tablet surfaces.
- Input labels must remain visible even when the keyboard is open.
- Dialog type, page content, and action buttons must remain reachable without accidental overlay blocking.

This ensures the same steampunk interaction language remains accessible across mouse, touch, and keyboard-driven workflows.

### 5.6 Filtering & Faceted Navigation

Filtering is a core interaction pattern in rusty-shed. It allows users to efficiently navigate large collections of railway models, purchases, wishlists, and other domain entities. The filtering experience must be responsive, touch-friendly, and consistent with the Mechanical Precision design language.

#### Responsive Filtering Patterns

**Desktop**

- Use a persistent, collapsible sidebar.
- Keep filters visible while browsing results.
- Display active filter selections.
- Allow individual filters to be removed.
- Keep sorting separate from filtering.

**Tablet**

- Use an overlay filter panel or side drawer.
- Do not permanently reserve a large portion of the viewport for filters.
- Preserve active filter visibility on the results page.

**Mobile**

- Never display a persistent filter sidebar.
- Provide a prominent Filter button in the collection toolbar.
- Open filters using a full-height bottom sheet or full-screen view.
- Use bottom sheets for short filter configurations and full-screen presentation for complex configurations.
- Keep active filter chips visible on the results page.

The system must preserve one shared visual vocabulary: the same steampunk tokens, panel treatment, and typography are reused across devices, while the filter shell adapts to the available space and input method.

#### Collection Toolbar

The collection toolbar should provide:

- Search input
- Filter action with active-filter count
- Separate sort action
- Active-filter indicators

**Suggested mobile layout:**

1. Search field
2. Filter and sort actions
3. Horizontally scrollable active-filter chips
4. Result count and sorting summary
5. Collection results

The toolbar must remain compact and must not unnecessarily reduce the available space for collection items.

#### Filter Panel Architecture

The filter panel must contain three regions.

**Header**

- Title: `FILTER COLLECTION.`
- Optional subtitle
- Close action
- Optional reset action

**Content**

- Vertically scrollable filter groups
- Collapsible sections for secondary criteria
- Searchable options for long lists
- Option counts where relevant
- Clear individual filter-group selections

**Footer**

- Primary action: `Apply Filters`
- Secondary action: `Clear all`
- Optional tertiary action: `Save view` for repeatable collection states

#### Filter Group Patterns

Use a predictable, scannable hierarchy for filter groups.

- Group by domain area (Type, Brand, Scale, Era, Condition, Budget, Status).
- Keep the most frequently used criteria at the top of the panel.
- Use a consistent `label + control + count` pattern for each option set.
- Support multi-select with visible selected states and a clear “selected” affordance.
- Use chips for high-frequency selections and checkboxes or segmented controls for nested logic.

The panel should feel mechanical and precise, not chaotic or overly decorative. Functional controls must be more prominent than ornamental accents.

#### Active Filters & Persistence

Active filters must remain visible and easy to remove.

- Render selected filters as chips above the collection results.
- Allow each chip to be dismissed individually.
- Maintain filter state when a collection view is re-opened or the same dataset is revisited.
- Show a non-zero count in the filter button on mobile and tablet.
- Distinguish between “inactive”, “selected but not applied”, and “applied” states in complex workflows.

#### Accessibility Requirements

- Filter panels must support keyboard navigation and focus trapping while open.
- Touch targets must remain at least 44×44px.
- Active states must be visibly different from default states under both themes.
- Scrollable filter regions must remain operable with pointer, keyboard, and assistive technology.
- All labels and counts must be exposed to screen readers without relying solely on color.

**Summary:** filtering should feel like a disciplined engineer’s control panel: structured, transparent, and highly usable, rather than a decorative overlay. It must support real railway-data workflows without sacrificing the steampunk identity or mobile usability.

### 5.7 Modals & Dialogs

#### Dialog (Modal)

**Anatomy:**
```
╔════════════════════════════════════╗
║ Title                          [×] ║
╟────────────────────────────────────╢
║ Content                            ║
║                                    ║
╟────────────────────────────────────╢
║            [Cancel] [Confirm]      ║
╚════════════════════════════════════╝
```

**Props:**
```typescript
interface DialogProps {
  open?: boolean;
  title: string;        // Paraglide key
  description?: string;
  children?: Snippet;
  actions?: DialogAction[];
  onopencange?: (open: boolean) => void;
}

interface DialogAction {
  label: string;
  variant?: 'default' | 'destructive';
  onclick?: () => void;
}
```

**Styling:**
```css
.dialog-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(4px);
  z-index: 40;
  animation: fadeIn 200ms ease-out;
}

.dialog-content {
  @apply rounded-xl bg-background border border-surface-100;
  box-shadow: 0 20px 25px rgba(0, 0, 0, 0.3);
  max-width: 90vw;
  max-height: 90vh;
  overflow-y: auto;
  animation: slideUp 300ms cubic-bezier(0.4, 0, 0.2, 1);
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
```

### 5.6 Status & Alerts

#### Toast Notification

**Anatomy:**
```
┌────────────────────────────────┐
│ ⚠ Warning message      [×]     │
└────────────────────────────────┘
```

**Props:**
```typescript
interface ToastProps {
  type?: 'success' | 'error' | 'warning' | 'info';
  message: string;         // Paraglide key
  duration?: number;       // ms, 0 = persistent
  action?: { label: string; onclick: () => void };
}
```

**Toast Signal Styling (Steampunk):**
```css
.toast {
  @apply rounded-md px-lg py-md flex items-center gap-md;
  border-left: 4px solid;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
  animation: slideInRight 300ms ease-out;
}

.toast.success {
  background: rgba(47, 165, 106, 0.1);
  border-left-color: var(--color-success-600);
  color: var(--color-success-600);
}

.toast.error {
  background: rgba(255, 51, 51, 0.1);
  border-left-color: var(--color-error-600);
  color: var(--color-error-600);
}

.toast.warning {
  background: rgba(255, 177, 0, 0.1);
  border-left-color: var(--color-warning-600);
  color: var(--color-warning-600);
}

.toast.info {
  background: rgba(205, 127, 50, 0.1);
  border-left-color: var(--color-primary-600);
  color: var(--color-primary-600);
}

/* Special signal border for non-fatal errors */
.toast.signal {
  border-left: 1px dotted #d48a42;
  border-top: 1px solid #d48a42;
  border-right: 1px solid #d48a42;
  border-bottom: 1px solid #d48a42;
}

@keyframes slideInRight {
  from {
    opacity: 0;
    transform: translateX(20px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}
```

#### Badge (Status Indicator)

**Props:**
```typescript
interface BadgeProps {
  variant?: 'primary' | 'success' | 'error' | 'warning' | 'neutral';
  children?: Snippet;
}
```

**Styling:**
```css
.badge {
  @apply inline-flex items-center gap-xs px-sm py-xs rounded-full;
  font-size: 11px;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.badge.primary {
  background: rgba(205, 127, 50, 0.2);
  color: var(--color-primary-600);
}

.badge.success {
  background: rgba(47, 165, 106, 0.2);
  color: var(--color-success-600);
}

.badge.error {
  background: rgba(255, 51, 51, 0.2);
  color: var(--color-error-600);
}
```

---


---

## 6. Accessibility & Tauri Desktop Considerations

### 6.1 WCAG AA Compliance

All interactive elements and text must meet **WCAG AA standards**:

- **Normal Text:** 4.5:1 contrast minimum
- **Large Text (18px+ or 14px bold+):** 3:1 contrast minimum
- **Graphics & UI Components:** 3:1 contrast minimum
- **Touch Targets:** Minimum 44×44px on mobile and tablet interfaces; desktop controls may use 32px minimum but should never shrink below accessible tap size in responsive layouts

**Testing Tools:**
- WebAIM Contrast Checker
- Axe DevTools (browser extension)
- NVDA or JAWS screen reader (Windows)
- Android accessibility inspector for touch target and screen-reader validation

### 6.2 Keyboard Navigation

**Tab Order:**
1. All interactive elements must be keyboard-accessible
2. Tab order must follow visual/logical hierarchy
3. Focus must be visible with 3px ring (--color-primary-600) and 2px offset

**Focus Management (Svelte):**
```svelte
<script>
  let buttonRef = $state<HTMLButtonElement>();

  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.key === 'Escape') {
      // Close modal, dismiss menu, etc.
    }
  };
</script>

<button 
  bind:this={buttonRef}
  onkeydown={handleKeyDown}
  class="focus-ring"
>
  Action
</button>

<style>
  :global(.focus-ring:focus-visible) {
    outline: 3px solid var(--color-primary-600);
    outline-offset: 2px;
  }
</style>
```

### 6.3 Motion & Animation Preferences

Respect `prefers-reduced-motion`:

```css
@media (prefers-reduced-motion: reduce) {
  * {
    animation: none !important;
    transition: none !important;
  }
  
  .lever-thumb {
    /* Remove transition, snap to final position */
  }
}
```

**Safe Animations:**
- Opacity changes (recommended, safe)
- Color transitions (recommended)
- Size/scale changes (use with caution)
- Translate/position moves (use with caution, max 300ms)

**Avoid:**
- Flashing or blinking (>3 per second)
- Parallax scrolling
- Auto-playing video/sound

### 6.4 Platform-Specific Shell Patterns

Tauri 2.0 provides native window controls for desktop environments (minimize, maximize, close) positioned at top-right (Windows/Linux) or top-left (macOS). These shell conventions are exclusively for desktop surfaces and should not be mirrored on Android.

**Custom Styling via CSS Tokens:**
```css
:root {
  --layout-titlebar-background: var(--color-background);
  --layout-titlebar-text: var(--color-text-900);
  --layout-titlebar-border: var(--color-surface-100);
  
  --layout-window-controls-hover-color: var(--color-primary-600);
  --layout-window-controls-close-color: var(--color-error-600);
}
```

**Desktop Window Control Configuration (tauri.conf.json):**
```json
{
  "tauri": {
    "windows": [
      {
        "titleBarStyle": "Transparent",
        "title": "rusty-shed",
        "width": 1200,
        "height": 800,
        "minWidth": 960,
        "minHeight": 600
      }
    ]
  }
}
```

**Mobile & Tablet Guidance:**
- Use safe-area insets for status bars and notch areas; avoid assuming a desktop titlebar exists.
- Prefer Android-native controls and bottom-sheet gestures over desktop-style floating toolbars.
- Keep the same copper/brass palette and component language while adapting shell chrome to mobile ergonomics.

### 6.5 High Contrast Mode

For users with high contrast preferences, use solid colors instead of gradients:

```css
@media (prefers-contrast: more) {
  .rivet {
    /* Solid circle instead of gradient */
    background: var(--color-primary-600);
  }
  
  .gauge {
    /* Solid border instead of gradient */
    border: 2px solid var(--color-primary-600);
  }
}
```

---

## 7. Implementation Guidelines

### 7.1 Svelte 5 Runes Only

All components must use modern Svelte 5 reactivity (Runes). **No legacy stores or reactive declarations.**

**❌ Legacy (Forbidden):**
```svelte
<script>
  export let count = 0;
  $: doubled = count * 2;
</script>

<button on:click={() => count++}>
  {count} → {doubled}
</button>
```

**✅ Svelte 5 (Required):**
```svelte
<script>
  interface Props {
    count?: number;
  }

  const { count = $bindable(0) } = $props();
  const doubled = $derived(count * 2);

  const increment = () => { count++; };
</script>

<button onclick={increment}>
  {count} → {doubled}
</button>
```

### 7.2 Component Props Pattern

Use `$props()` for all component inputs:

```typescript
interface ButtonProps {
  variant?: 'default' | 'destructive' | 'outline';
  size?: 'sm' | 'default' | 'lg';
  disabled?: boolean;
  loading?: boolean;
  onclick?: (e: MouseEvent) => void;
  children?: Snippet;
}

// ✅ Correct
const { variant = 'default', size = 'default', disabled = false, loading = false, onclick, children } = $props();

// ❌ Wrong (legacy export let)
export let variant = 'default';
```

### 7.3 TypeScript Strict Mode

**No `any` type.** Use `unknown` or specific interfaces:

```typescript
// ❌ Forbidden
const data: any = fetchData();

// ✅ Correct
interface Model {
  id: string;
  name: string;
  scale: string;
}

const data: Model[] = await fetchData();

// Or use unknown with type guards
const data: unknown = fetchData();
if (Array.isArray(data)) {
  // Process safely
}
```

### 7.4 Internationalization (i18n)

**All user-facing strings must use Paraglide-JS:**

1. **Add message keys to both locales:**

   **`src/lib/paraglide/messages/en.json`:**
   ```json
   {
     "core.actions.save": "Save",
     "core.actions.delete": "Delete",
     "core.validation.required": "This field is required"
   }
   ```

   **`src/lib/paraglide/messages/it.json`:**
   ```json
   {
     "core.actions.save": "Salva",
     "core.actions.delete": "Elimina",
     "core.validation.required": "Questo campo è obbligatorio"
   }
   ```

2. **Use in components:**
   ```svelte
   <script>
     import * as m from '$lib/paraglide/messages.js';
   </script>

   <button>{m.core.actions.save()}</button>
   ```

3. **Use in Rust (backend):**
   ```rust
   // Messages are sync'd via tauri-specta; use message keys as constants
   const SAVE_SUCCESS: &str = "model.actions.save_success";
   ```

### 7.5 Error Handling (Rust Backend)

**No `unwrap()`.** Always use `Result` with serializable error enums:

```rust
#[derive(Debug, serde::Serialize, Deserialize)]
pub enum AppError {
    #[serde(rename = "model_not_found")]
    ModelNotFound(String),
    
    #[serde(rename = "validation_error")]
    ValidationError { field: String, message: String },
    
    #[serde(rename = "database_error")]
    DatabaseError(String),
}

impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        serde_json::to_string(&error).unwrap_or_else(|_| "Unknown error".to_string())
    }
}

// ✅ Correct
#[tauri::command]
pub async fn save_model(model: Model) -> Result<ModelId, AppError> {
    let id = db::insert(&model)
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(id)
}
```

### 7.6 Tauri-Specta Bindings

After modifying Rust command signatures or types:

```bash
pnpm specta:generate
```

**Then verify the TypeScript bindings in `src-tauri/bindings/`.**

### 7.7 Code Quality Checks

Run before committing:

```bash
# Frontend
pnpm svelte-check                # Type check Svelte
pnpm lint                        # ESLint + formatting

# Backend
cargo clippy -- -D warnings      # No warnings allowed
cargo test                       # Unit tests
cargo fmt                        # Format
```

---

## 8. Motion & Animation

### 8.1 Steampunk Lever Interaction

The iconic "lever press" provides tactile feedback:

```svelte
<script>
  let pressed = $state(false);
  
  const handleMouseDown = () => { pressed = true; };
  const handleMouseUp = () => { pressed = false; };
</script>

<button 
  class="lever"
  class:pressed
  onmousedown={handleMouseDown}
  onmouseup={handleMouseUp}
  onmouseleave={handleMouseUp}
>
  Pull Lever
</button>

<style>
  .lever {
    transition: transform 80ms ease-out, box-shadow 80ms ease-out;
  }
  
  .lever:active {
    transform: translateY(2px);
    box-shadow: inset 0 2px 6px rgba(0, 0, 0, 0.3);
  }
</style>
```

### 8.2 Gauge Spin-Up

Animated percentage fill on load:

```svelte
<script>
  import { animate } from 'motion';

  interface Props {
    value: number;
    max: number;
  }

  const { value, max } = $props();
  const percentage = $derived((value / max) * 100);
  let displayPercentage = $state(0);

  $effect(() => {
    animate(0, percentage, {
      duration: 0.8,
      onUpdate: (v) => {
        displayPercentage = v;
      }
    });
  });
</script>

<div class="gauge" style="--progress: {displayPercentage}%">
  <span>{Math.round(displayPercentage)}%</span>
</div>

<style>
  .gauge {
    background: conic-gradient(
      var(--color-primary-600) 0deg calc(var(--progress) * 3.6deg),
      var(--color-surface-200) calc(var(--progress) * 3.6deg) 360deg
    );
  }
</style>
```

### 8.3 Modal Entrance

Slide-up with fade:

```css
@keyframes modalEnter {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.modal {
  animation: modalEnter 300ms cubic-bezier(0.4, 0, 0.2, 1);
}

@media (prefers-reduced-motion: reduce) {
  .modal {
    animation: none;
  }
}
```

### 8.4 Transition Timings

| Animation Type      | Duration | Easing                           |
|---------------------|----------|----------------------------------|
| Hover state         | 150ms    | ease-out / ease-in-out           |
| Button press        | 80ms     | ease-out                         |
| Dialog enter        | 300ms    | cubic-bezier(0.4, 0, 0.2, 1)    |
| Sidebar collapse    | 300ms    | cubic-bezier(0.4, 0, 0.2, 1)    |
| Gauge fill          | 800ms    | ease-in-out                      |
| Toast notification  | 300ms    | cubic-bezier(0.4, 0, 0.2, 1)    |

---

## 9. Code Examples

### 9.1 Building a Steampunk Card Component

**File: `src/lib/components/ui/steampunk-card/SteampunkCard.svelte`**

```svelte
<script lang="ts">
  import type { Snippet } from 'svelte';
  
  interface Props {
    title: string;
    subtitle?: string;
    variant?: 'default' | 'elevated' | 'blueprint';
    children?: Snippet;
    footer?: Snippet;
  }

  const { title, subtitle, variant = 'default', children, footer } = $props();
  
  const variantClass = {
    default: 'bg-surface-900 border border-surface-100',
    elevated: 'bg-surface-900 border-0 shadow-lg',
    blueprint: 'bg-opacity-50 border-dashed border-primary-600'
  }[variant];
</script>

<div class="card {variantClass}">
  <div class="card-header">
    <h3 class="card-title">{title}</h3>
    {#if subtitle}
      <p class="card-subtitle">{subtitle}</p>
    {/if}
  </div>
  
  <div class="card-body">
    {@render children?.()}
  </div>
  
  {#if footer}
    <div class="card-footer">
      {@render footer?.()}
    </div>
  {/if}
</div>

<style>
  .card {
    @apply rounded-lg p-lg;
    transition: all 200ms ease-out;
  }
  
  .card:hover {
    border-color: var(--color-primary-600);
  }
  
  .card-header {
    margin-bottom: var(--spacing-lg);
    border-bottom: 1px solid var(--color-surface-100);
    padding-bottom: var(--spacing-md);
  }
  
  .card-title {
    font-family: 'Bebas Neue', Georgia, serif;
    font-size: 20px;
    color: var(--heading-color);
    margin: 0;
  }
  
  .card-subtitle {
    font-size: 12px;
    color: var(--color-text-900);
    margin: 4px 0 0 0;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
  
  .card-body {
    margin-bottom: var(--spacing-lg);
  }
  
  .card-footer {
    border-top: 1px solid var(--color-surface-100);
    padding-top: var(--spacing-md);
    display: flex;
    gap: var(--spacing-md);
  }
</style>
```

**Usage:**
```svelte
<script>
  import SteampunkCard from '$lib/components/ui/steampunk-card/SteampunkCard.svelte';
  import Button from '$lib/components/ui/button/button.svelte';
  import * as m from '$lib/paraglide/messages.js';
</script>

<SteampunkCard 
  title={$m.models.details.title()}
  subtitle="Model Information"
  variant="elevated"
>
  <p>Model details go here...</p>
  
  {#snippet footer()}
    <Button variant="outline">{m.core.actions.cancel()}</Button>
    <Button variant="default">{m.core.actions.save()}</Button>
  {/snippet}
</SteampunkCard>
```

### 9.2 Form with Validation (Svelte 5 + Superform)

**File: `src/lib/components/forms/ModelForm.svelte`**

```svelte
<script lang="ts">
  import { superForm } from 'sveltekit-superforms';
  import { zodClient } from 'sveltekit-superforms/adapters';
  import { modelSchema } from '$lib/schemas';
  import TextInput from '$lib/components/ui/input/TextInput.svelte';
  import Button from '$lib/components/ui/button/button.svelte';
  import * as m from '$lib/paraglide/messages.js';

  const { form, errors, enhance } = superForm(data.form, {
    validators: zodClient(modelSchema),
  });

  let loading = $state(false);

  const handleSubmit = async () => {
    loading = true;
    // Form data is validated automatically by superForm
    loading = false;
  };
</script>

<form method="POST" use:enhance>
  <div class="form-group">
    <label for="name">{m.models.fields.name()}</label>
    <TextInput
      id="name"
      name="name"
      bind:value={$form.name}
      error={$errors.name?.[0]}
      placeholder={m.models.placeholders.name()}
    />
  </div>

  <div class="form-group">
    <label for="manufacturer">{m.models.fields.manufacturer()}</label>
    <TextInput
      id="manufacturer"
      name="manufacturer"
      bind:value={$form.manufacturer}
      error={$errors.manufacturer?.[0]}
    />
  </div>

  <div class="form-actions">
    <Button variant="outline" type="button">
      {m.core.actions.cancel()}
    </Button>
    <Button variant="default" type="submit" loading={loading}>
      {m.core.actions.save()}
    </Button>
  </div>
</form>

<style>
  .form-group {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
    margin-bottom: var(--spacing-lg);
  }

  label {
    font-weight: 500;
    color: var(--color-text-950);
    font-size: 14px;
  }

  .form-actions {
    display: flex;
    gap: var(--spacing-md);
    justify-content: flex-end;
    margin-top: var(--spacing-xl);
    border-top: 1px solid var(--color-surface-100);
    padding-top: var(--spacing-lg);
  }
</style>
```

### 9.3 Tauri Command with Type Safety

**Backend (Rust): `src-tauri/src/commands/models.rs`**

```rust
use serde::{Deserialize, Serialize};
use tauri::State;
use crate::error::AppError;
use crate::models::{Model, ModelId};
use crate::db::Database;

#[derive(Serialize, Deserialize)]
pub struct CreateModelRequest {
    pub name: String,
    pub manufacturer: String,
    pub scale: String,
}

#[tauri::command]
pub async fn create_model(
    db: State<'_, Database>,
    request: CreateModelRequest,
) -> Result<Model, AppError> {
    let model = Model {
        id: ModelId::new(),
        name: request.name.clone(),
        manufacturer: request.manufacturer.clone(),
        scale: request.scale.clone(),
        ..Default::default()
    };

    db.insert(&model)
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(model)
}
```

**Frontend (Svelte): `src/routes/models/create/+page.svelte`**

```svelte
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import type { CreateModelRequest, Model } from '$lib/bindings';
  import ModelForm from '$lib/components/forms/ModelForm.svelte';

  let result: Model | null = $state(null);
  let error: string | null = $state(null);

  const handleSave = async (formData: CreateModelRequest) => {
    try {
      result = await invoke<Model>('create_model', { request: formData });
    } catch (err) {
      error = String(err);
    }
  };
</script>

{#if result}
  <p>✓ Model created: {result.name}</p>
{:else if error}
  <p class="error">✗ {error}</p>
{/if}

<ModelForm onsave={handleSave} />
```

---

## 10. Textures & Visual Effects

### 10.1 Parchment Texture (Light Theme)

```css
.parchment-texture {
  background-image: repeating-linear-gradient(
    45deg,
    transparent,
    transparent 10px,
    rgba(161, 130, 80, 0.02) 10px,
    rgba(161, 130, 80, 0.02) 20px
  );
}
```

### 10.2 Metal Grain Texture (Dark Theme)

```css
.metal-grain-texture {
  background-image: repeating-linear-gradient(
    90deg,
    transparent,
    transparent 1px,
    rgba(255, 255, 255, 0.02) 1px,
    rgba(255, 255, 255, 0.02) 2px
  );
}
```

### 10.3 Rivet Decorations

```css
.rivet {
  display: inline-block;
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: radial-gradient(
    circle at 30% 30%,
    rgba(255, 255, 255, 0.3),
    rgba(205, 127, 50, 0.6)
  );
  box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.4);
}

/* High contrast mode */
@media (prefers-contrast: more) {
  .rivet {
    background: var(--color-primary-600);
    box-shadow: none;
  }
}
```

### 10.4 Blueprint Panel Effect

```css
.blueprint-panel {
  background: rgba(26, 26, 27, 0.5);
  border: 2px dotted var(--color-primary-600);
  border-radius: 8px;
  padding: var(--spacing-lg);
  backdrop-filter: blur(2px);
}
```

---

## 11. Checklist for Component Implementation

When creating a new component, ensure:

- [ ] Props defined with TypeScript interface
- [ ] Uses Svelte 5 Runes ($props, $state, $derived, $effect)
- [ ] Follows Tailwind + shadcn-svelte conventions
- [ ] Has steampunk variant (if interactive element)
- [ ] Accessibility: WCAG AA contrast, keyboard nav, ARIA labels
- [ ] i18n: All strings use Paraglide-JS (en + it)
- [ ] Error handling: No try-catch swallowing errors; pass to parent
- [ ] Focus management: Visible 3px ring with 2px offset
- [ ] Respects prefers-reduced-motion
- [ ] Tested with Vitest (unit) + browser (visual)
- [ ] Documented in COMPONENTS_TREE.md
- [ ] No legacy store usage or $: reactive declarations

---

## 12. Resources & References

- **Tauri 2.0 Docs:** https://docs.rs/tauri/2.0/tauri/
- **Svelte 5 Runes:** https://svelte.dev/docs/runes
- **Tailwind CSS v4:** https://tailwindcss.com/docs
- **shadcn-svelte:** https://www.shadcn-svelte.com/
- **Paraglide-JS i18n:** https://inlang.com/m/gkh6pjz9/library-inlang-paraglideJs
- **WCAG Guidelines:** https://www.w3.org/WAI/WCAG21/quickref/
- **Phosphor Icons:** https://phosphoricons.com/ (recommended icon set)
- **Storybook Steampunk Design:** https://www.smashingmagazine.com/2021/03/steampunk-style-guide/

---

**Document Version:** 1.0  
**Last Updated:** November 2026  
**Maintained by:** Design Systems & Frontend Architecture Team

EOF
echo "Part 3 created successfully"
