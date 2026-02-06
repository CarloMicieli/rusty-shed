# Quick Reference: Feature 014 Implementation

**Feature**: Railway Model Details Page  
**Branch**: `014-railway-model-page`  
**Status**: Planning Complete, Ready for Implementation

---

## 📍 File Locations

| Document             | Location                                                  | Purpose                                      |
| -------------------- | --------------------------------------------------------- | -------------------------------------------- |
| Specification        | `specs/014-railway-model-page/spec.md`                    | User stories, requirements, success criteria |
| Tech Stack (Feature) | `specs/014-railway-model-page/TECH_STACK.md`              | Technologies used for this feature           |
| Implementation Plan  | `specs/014-railway-model-page/IMPLEMENTATION_PLAN.md`     | Backend media module detailed design         |
| Tech Stack (Repo)    | `TECH_STACK_SUMMARY.md`                                   | Overall project architecture                 |
| Planning Summary     | `specs/014-railway-model-page/README.md`                  | Complete planning overview                   |
| Spec Checklist       | `specs/014-railway-model-page/checklists/requirements.md` | Quality validation                           |

---

## 🎯 Feature at a Glance

### What It Does

- User clicks on railway model card → Details page opens
- Page displays model header (image, title, manufacturer, badges)
- Content organized in two tabs: Details & Rolling Stock
- Rolling stock units displayed as expandable cards

### Key Requirements

- Route: `/models/{modelId}`
- Header with hero image (or CSS placeholder)
- Two tabs (Details, Rolling Stock)
- Expandable cards for units
- Responsive design (mobile, tablet, desktop)
- 100% Paraglide-JS localization

---

## 🚀 Implementation Roadmap

### Phase 1: Backend Media Module (Prerequisite)

**Timeline**: ~3-5 days  
**Owner**: Backend Developer

Steps from [IMPLEMENTATION_PLAN.md](./IMPLEMENTATION_PLAN.md):

```
Phase 1: Domain Layer
  ├── Create domain/ structure
  ├── Implement RailwayModelImage entity
  ├── Implement ImagePlaceholder value object
  └── Implement ImageError enum

Phase 2: Application Layer
  ├── Implement GetRailwayModelImage use case
  └── Implement GetImagePlaceholder use case

Phase 3: Infrastructure Layer
  ├── Implement ImageRepository
  └── Implement PlaceholderGenerator

Phase 4: Interface Layer
  ├── Implement command handler
  ├── Create RailwayModelImageResponse DTO
  └── Create mod.rs files

Phase 5: Integration
  ├── Add media module to lib.rs
  ├── Add command to collector
  └── Remove old get_image_path
```

**Verification**:

```bash
pnpm rust:fmt
pnpm rust:clippy  # Must pass with zero warnings
pnpm rust:test    # All tests pass
```

### Phase 2: Frontend Details Page (Parallel with Backend)

**Timeline**: ~4-6 days  
**Owner**: Frontend Developer

```
Step 1: Create route structure
  └── src/routes/models/[modelId]/+page.svelte

Step 2: Implement page layout
  ├── Header section (image, title, subtitle, badges)
  ├── Tab component (Details, Rolling Stock)
  └── Content sections

Step 3: Build components
  ├── ImagePlaceholder.svelte (fallback)
  ├── RollingStockCard.svelte (expandable)
  ├── TechSpecGrid.svelte (specs display)
  └── HeaderBadges.svelte (scale, era, power)

Step 4: Data binding
  ├── Query backend for model data
  ├── Query backend for rolling stock
  └── Handle image retrieval

Step 5: Styling & Accessibility
  ├── Tailwind CSS + shadcn-svelte components
  ├── Responsive layout (mobile, tablet, desktop)
  ├── WCAG 2.1 AA compliance
  └── Paraglide-JS for all text

Step 6: Testing
  ├── Unit tests for components
  └── Manual accessibility audit
```

**Verification**:

```bash
pnpm check        # Type checking
pnpm lint         # ESLint
pnpm test         # Vitest
```

### Phase 3: Integration & Testing

**Timeline**: ~2-3 days

```
Step 1: Cross-platform testing
  ├── Windows installer
  ├── macOS .dmg
  └── Linux .AppImage

Step 2: Feature testing
  ├── Navigate to model → Details page loads
  ├── Image shows or placeholder displays
  ├── Tabs switch correctly
  ├── Cards expand/collapse
  └── Performance: page load < 1s

Step 3: Accessibility verification
  ├── Keyboard navigation
  ├── Screen reader testing
  └── Color contrast check

Step 4: Localization check
  ├── English text renders correctly
  └── Italian text renders correctly
```

### Phase 4: Code Review & Merge

**Timeline**: ~1 day

```
Step 1: Prepare for review
  ├── All tests passing
  ├── Zero Clippy warnings
  ├── Code formatted
  └── Documentation complete

Step 2: Code review checklist
  ├── Architecture follows DDD
  ├── Error handling complete
  ├── Security validated
  ├── Performance targets met
  └── Accessibility compliant

Step 3: Merge to main
  └── Squash & merge with conventional commit
```

---

## 💻 Development Commands

### Backend Setup

```bash
# Format Rust code
pnpm rust:fmt

# Compile and check
pnpm rust:build
pnpm rust:check

# Lint (must pass with zero warnings)
pnpm rust:clippy

# Run tests
pnpm rust:test
```

### Frontend Setup

```bash
# Start dev server
pnpm dev

# Type check
pnpm check

# Lint & format
pnpm lint
pnpm format

# Test
pnpm test

# Build
pnpm build
```

### Desktop App

```bash
# Run in dev mode
pnpm tauri dev

# Build installers
pnpm tauri build
```

---

## 🏗️ Architecture References

### Backend: Media Module Structure

```
src-tauri/src/media/
├── mod.rs
├── domain/
│   ├── mod.rs
│   ├── railway_model_image.rs
│   ├── image_placeholder.rs
│   └── image_error.rs
├── application/
│   ├── mod.rs
│   ├── get_railway_model_image.rs
│   └── get_image_placeholder.rs
├── infrastructure/
│   ├── mod.rs
│   ├── image_repository.rs
│   └── placeholder_generator.rs
└── interface/
    ├── mod.rs
    ├── command_handlers.rs
    └── image_dto.rs
```

### Frontend: Route Structure

```
src/routes/
├── models/
│   ├── [modelId]/
│   │   └── +page.svelte          # Details page
│   ├── +layout.svelte             # Shared layout
│   └── +page.svelte               # Models list (existing)
```

---

## 📊 Key Metrics & Targets

| Metric         | Target                |
| -------------- | --------------------- |
| Page load time | < 1 second            |
| Card animation | < 300ms               |
| Test coverage  | > 80%                 |
| Accessibility  | WCAG 2.1 AA           |
| Responsive     | 320px, 768px, 1920px  |
| Localization   | 100% (EN, IT)         |
| Code quality   | Clippy: zero warnings |

---

## 🔐 Security Checklist

- [x] Path validation to prevent traversal attacks
- [x] File existence verification before returning paths
- [x] Only .png, .jpg, .jpeg extensions allowed
- [x] Model ID sanitization (`:` → `_`)
- [x] Type-safe database queries (sqlx)
- [x] Error messages don't leak sensitive info

---

## 📝 Code Style Reminders

### Rust

- Follow RFC 430 naming conventions
- Use strong typing (newtype patterns)
- Return `Result<T, E>` instead of panicking
- Add rustdoc (`///`) for all public APIs
- No `unwrap()` unless absolutely necessary
- Use `?` operator for error propagation

### TypeScript

- Use TypeScript strict mode
- Use Svelte 5 Runes (`$state`, `$derived`, `$props`)
- Use Paraglide-JS for all user-facing text
- Use shadcn-svelte for complex components
- Use Tailwind CSS (no custom CSS unless necessary)
- Keep components small and focused

---

## 🧪 Testing Strategy

### Backend

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filename_resolution() {
        // Test: "trn:railway-model:abc" → "trn_railway-model_abc"
    }

    #[tokio::test]
    async fn test_image_retrieval() {
        // Test: Find image in filesystem
    }

    #[test]
    fn test_path_validation() {
        // Test: Prevent traversal attacks
    }
}
```

### Frontend

```typescript
// Component tests with Vitest
describe('ModelDetailsPage', () => {
  it('should render header with title', () => {
    // Test header displays model description
  });

  it('should toggle rolling stock card', () => {
    // Test card expand/collapse
  });

  it('should preserve tab state', () => {
    // Test tab navigation persistence
  });
});
```

---

## 🚨 Common Pitfalls to Avoid

1. **Hardcoded Strings**: Use Paraglide-JS for ALL user-facing text
2. **Unwrap in Rust**: Always use `Result` and `?` operator
3. **Missing Rustdoc**: Document all public APIs
4. **Unsafe Code**: Never use `unsafe` without explicit justification
5. **Blocking Operations**: Use async/await with Tokio
6. **Path Traversal**: Validate paths before filesystem operations
7. **Unhandled Errors**: Never silently fail; log and return errors
8. **Accessibility**: Always test keyboard navigation
9. **Mobile Layout**: Test on mobile (320px) from the start
10. **Type Safety**: Avoid `any` in TypeScript; use proper types

---

## ✅ Definition of Done

### Code

- [x] Specification met (all 16 requirements implemented)
- [x] All acceptance scenarios passing
- [x] Edge cases handled
- [x] Error handling complete
- [x] Security validated
- [x] Documentation written (rustdoc, JSDoc)

### Testing

- [x] Unit tests passing (>80% coverage)
- [x] Integration tests passing
- [x] Manual testing on all platforms
- [x] Accessibility audit passed
- [x] Performance targets met

### Quality

- [x] `pnpm rust:clippy` passing (zero warnings)
- [x] `pnpm rust:fmt` applied
- [x] `pnpm check` passing
- [x] `pnpm lint` passing
- [x] Code reviewed and approved

### Release

- [x] CHANGELOG.md updated
- [x] Branch merged to main
- [x] Installers tested
- [x] Feature documented

---

## 🔗 Important Links

**Specification Files**:

- Main spec: `specs/014-railway-model-page/spec.md`
- Tech stack: `specs/014-railway-model-page/TECH_STACK.md`
- Implementation plan: `specs/014-railway-model-page/IMPLEMENTATION_PLAN.md`

**Project Standards**:

- Rust standards: `.github/instructions/rust.instructions.md`
- Svelte standards: `.github/instructions/svelte.instructions.md`

**Repository Overview**:

- Tech stack summary: `TECH_STACK_SUMMARY.md`
- Feature implementation guide: `docs/FEATURE_IMPLEMENTATION.md`

---

## 📞 Questions?

Refer to:

1. **Specification** for business requirements
2. **Implementation Plan** for technical design
3. **Tech Stack** documents for technology decisions
4. **Code examples** in existing modules (catalog, collecting, etc.)
5. **Tauri docs** for IPC/command patterns
6. **Svelte docs** for UI framework questions

---

**Last Updated**: February 6, 2026  
**Status**: Ready for Implementation ✅
