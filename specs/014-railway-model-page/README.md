# ✅ Planning Phase Complete: Railway Model Details Page (Feature 014)

**Status**: Ready for Implementation  
**Branch**: `014-railway-model-page`  
**Created**: February 6, 2026

---

## Overview

Complete planning documentation for **Feature 014: Railway Model Details Page** has been created and committed. This feature enables users to click on a railway model card in their collection and view a detailed page with comprehensive model information, organized into tabs with expandable rolling stock cards.

The feature is split into two parallel work streams:
1. **Backend**: Media module for image management (prerequisite)
2. **Frontend**: Railway model details page UI (main feature)

---

## 📋 Delivered Documentation

### 1. Feature Specification
**File**: [specs/014-railway-model-page/spec.md](specs/014-railway-model-page/spec.md)

- ✅ 3 P1 user stories (all equally critical)
- ✅ 16 functional requirements (FR-001 through FR-016)
- ✅ Key entity definitions (RailwayModel, RollingStock, TechnicalSpecification)
- ✅ 5 edge cases identified and addressed
- ✅ 6 measurable success criteria
- ✅ Assumptions and scope boundaries clearly defined

**User Stories**:
1. **View Railway Model Details** - Core page functionality
2. **Navigate Model Information via Tabs** - Tab-based organization
3. **Explore Individual Rolling Stock Units** - Expandable unit cards

---

### 2. Technology Stack Reference
**File**: [specs/014-railway-model-page/TECH_STACK.md](specs/014-railway-model-page/TECH_STACK.md)

Complete tech stack for the feature:

**Frontend Stack**:
- Svelte 5 (with Runes: `$state`, `$derived`, `$props`)
- TypeScript (strict mode)
- Vite (build) + SvelteKit (routing)
- Tailwind CSS 4 + shadcn-svelte 1.1.1 (components)
- Paraglide-JS (i18n: EN, IT)
- Vitest + Playwright (testing)

**Backend Stack**:
- Rust 1.93.0+ with Tauri 2.9.5
- Tokio async runtime
- SQLite + sqlx ORM
- specta (TypeScript binding generation)
- thiserror (error handling)
- Image storage: Models directory with filesystem-based image retrieval

**IPC Communication**:
- Tauri command pattern with automatic TypeScript bindings
- Type-safe from Rust → TypeScript

---

### 3. Media Module Implementation Plan
**File**: [specs/014-railway-model-page/IMPLEMENTATION_PLAN.md](specs/014-railway-model-page/IMPLEMENTATION_PLAN.md)

**Comprehensive 5-phase implementation roadmap** for the backend media module:

#### Architecture (DDD Pattern)
```
media/
├── domain/
│   ├── railway_model_image.rs      # Image entity
│   ├── image_placeholder.rs        # Placeholder value object
│   └── image_error.rs              # Error type
├── application/
│   ├── get_railway_model_image.rs  # Use case
│   └── get_image_placeholder.rs    # Use case
├── infrastructure/
│   ├── image_repository.rs         # File system operations
│   └── placeholder_generator.rs    # HTML/CSS generation
└── interface/
    ├── command_handlers.rs         # Tauri command
    └── image_dto.rs                # Response DTO
```

#### Implementation Phases
1. **Phase 1**: Domain layer (entities, value objects, errors)
2. **Phase 2**: Application layer (use cases)
3. **Phase 3**: Infrastructure layer (file I/O, placeholder generation)
4. **Phase 4**: Interface layer (Tauri command)
5. **Phase 5**: Integration (add to lib.rs, remove old get_image_path)

#### Key Features
- ✅ Type-safe image retrieval
- ✅ Path traversal attack prevention
- ✅ Fallback placeholder (HTML/CSS) generation
- ✅ Support for .png, .jpg, .jpeg extensions
- ✅ Filename transformation (`:` → `_`)
- ✅ Comprehensive error handling
- ✅ Full rustdoc documentation

---

### 4. Repository-Wide Tech Stack
**File**: [TECH_STACK_SUMMARY.md](TECH_STACK_SUMMARY.md)

Complete overview of the entire Rusty Shed repository:
- Full technology matrix for frontend and backend
- DDD architecture explanation
- Module structure
- Data persistence strategy
- Security considerations
- Development workflow
- Testing approach
- Performance optimizations
- Key stats and metrics

---

### 5. Specification Quality Checklist
**File**: [specs/014-railway-model-page/checklists/requirements.md](specs/014-railway-model-page/checklists/requirements.md)

✅ All checklist items passing:
- [x] No implementation details (user/business focused)
- [x] All mandatory sections completed
- [x] No unresolved clarifications
- [x] Requirements are testable and unambiguous
- [x] Success criteria are measurable and technology-agnostic
- [x] Edge cases identified
- [x] Feature is bounded and ready for implementation

---

## 🎯 Feature Highlights

### Header Section
- **Primary Title**: Model description (e.g., "DB BR 218 Diesel Locomotive")
- **Subtitle**: Manufacturer | Product Code
- **Hero Image**: Actual photo or CSS/HTML placeholder
- **Quick Badges**: Scale, Era, Power Method (for at-a-glance identification)

### Tab Organization
1. **Details Tab**: Full detailed description of the model
2. **Rolling Stock Tab**: List of owned units with expandable cards

### Rolling Stock Cards
- **Collapsed Header**: `{Type} — {Road Number}` (e.g., "Locomotive — 218 217-8")
- **Expanded Body**: Type, Road Number, Depot, Series Code, Railway Company, Country, Livery, Technical Specs

### Accessibility & Responsiveness
- WCAG 2.1 AA compliance required
- Mobile (320px), Tablet (768px), Desktop (1920px) all supported
- Keyboard navigation for all interactive elements
- 100% Paraglide-JS localization (no hardcoded text)

---

## 📊 Specification Metrics

| Metric | Value |
|--------|-------|
| **User Stories** | 3 (all P1) |
| **Functional Requirements** | 16 |
| **Edge Cases** | 5 |
| **Success Criteria** | 6 (measurable) |
| **Key Entities** | 3 (RailwayModel, RollingStock, TechnicalSpec) |
| **Frontend Routes** | 1 (`/models/[modelId]`) |
| **Tauri Commands** | 2 (get_railway_model_image + existing data queries) |
| **Documentation Files** | 5 |

---

## 🚀 Next Steps (Implementation Phase)

### Backend Work (Feature Squad)
1. **Implement Media Module**
   - Follow IMPLEMENTATION_PLAN.md (5 phases)
   - Create DDD-structured module
   - Implement image retrieval + placeholder generation
   - Integrate into lib.rs
   - Run `pnpm rust:fmt`, `pnpm rust:clippy`, `pnpm rust:test`

### Frontend Work (Feature Squad)
2. **Implement Details Page**
   - Create route: `/src/routes/models/[modelId]/+page.svelte`
   - Implement header with hero image + badges
   - Create tab navigation (Details & Rolling Stock)
   - Build expandable rolling stock cards
   - Implement responsive design (mobile, tablet, desktop)
   - Use Paraglide-JS for all text
   - Run `pnpm check`, `pnpm lint`, `pnpm test`

### Testing & Verification
3. **Quality Assurance**
   - Unit tests for all components
   - Manual testing on Windows, macOS, Linux
   - Accessibility audit (WCAG 2.1 AA)
   - Performance testing (page load < 1s, animations < 300ms)
   - Localization verification (EN & IT)

### Deployment
4. **Release**
   - Merge to main after review
   - Verify TypeScript bindings generated
   - Build and test installers
   - Update CHANGELOG.md

---

## 📚 Specification Files Structure

```
specs/014-railway-model-page/
├── spec.md                           # Main specification (user-focused)
├── TECH_STACK.md                     # Feature-specific tech stack
├── IMPLEMENTATION_PLAN.md            # Detailed backend media module plan
├── checklists/
│   └── requirements.md               # Specification quality checklist
└── contracts/                        # (for Phase 1 planning: data model + API)
```

---

## 📋 Functional Requirements Summary

### Display & Navigation (FR-001 to FR-006)
- [x] Route: `/models/{modelId}`
- [x] Header displays model description, manufacturer, product code
- [x] Hero image (actual or CSS placeholder)
- [x] Quick badges (scale, era, power method)
- [x] Two tabs: Details & Rolling Stock

### Content Organization (FR-007 to FR-010)
- [x] Details tab shows full description
- [x] Rolling Stock tab displays units as cards
- [x] Card header format: `{type} — {road_number}`
- [x] Expanded card shows all unit details

### UX Features (FR-011 to FR-016)
- [x] Responsive grid/table for technical specs
- [x] Independent card expand/collapse
- [x] Tab state preservation
- [x] Graceful handling of missing fields
- [x] Empty state message when no rolling stock
- [x] 100% Paraglide-JS localization

---

## ✨ Quality Assurance Targets

| Aspect | Target | Status |
|--------|--------|--------|
| **Performance** | < 1 second page load | Specified |
| **Animation** | < 300ms card expand/collapse | Specified |
| **Accessibility** | WCAG 2.1 AA | Specified |
| **Responsiveness** | 320px, 768px, 1920px viewports | Specified |
| **Localization** | 100% Paraglide-JS | Specified |
| **Type Safety** | Full TypeScript + Rust coverage | Specified |
| **Code Quality** | Clippy: zero warnings | Specified |
| **Testing** | Unit + integration tests | Specified |

---

## 🔗 Related Resources

### In-Repo Documentation
- [Rust Standards](/.github/instructions/rust.instructions.md)
- [Svelte Standards](/.github/instructions/svelte.instructions.md)
- [Global Copilot Instructions](/.github/copilot-instructions.md)
- [Feature Implementation Guide](/docs/FEATURE_IMPLEMENTATION.md)

### External References
- [Tauri Documentation](https://tauri.app/)
- [Svelte 5 Docs](https://svelte.dev/docs/svelte/what-is-svelte)
- [Tailwind CSS](https://tailwindcss.com/)
- [shadcn-svelte](https://www.shadcn-svelte.com/)
- [Paraglide-JS](https://inlang.com/)

---

## ✅ Checklist: Ready for Handoff

- [x] Feature specification written (non-technical, business-focused)
- [x] User stories prioritized (3 P1 stories)
- [x] Functional requirements defined (16 total)
- [x] Success criteria measurable and technology-agnostic
- [x] Edge cases identified and addressed
- [x] Technology stack documented (tech-specific)
- [x] Backend implementation plan detailed (5 phases)
- [x] Architecture pattern documented (DDD)
- [x] Code structure specified with examples
- [x] Error handling approach defined
- [x] Security considerations addressed (path validation)
- [x] Testing strategy outlined
- [x] Specification quality checklist completed
- [x] All documents committed to branch

---

## 📝 Final Notes

### For Feature Squad (Implementation)
- Start with **backend media module** (prerequisite for image retrieval)
- Follow **IMPLEMENTATION_PLAN.md** exactly (5 phases)
- Frontend implementation can proceed in parallel once backend API is exposed
- Use **TECH_STACK.md** as reference for technology-specific questions

### For Reviewers
- Specification is **non-technical** and focuses on user value
- Technology decisions are **documented in TECH_STACK.md**
- Implementation approach is **detailed in IMPLEMENTATION_PLAN.md**
- All functional requirements are **testable and measurable**

### For Project Manager
- Feature is **well-scoped** with 3 independent P1 stories
- Estimated effort: ~2 weeks (backend module + frontend page)
- **No database schema changes** required
- **No new dependencies** required (existing tech stack sufficient)
- Ready for **Sprint Planning**

---

## 🎓 Learning Resources

For team members unfamiliar with project patterns:
1. Read `TECH_STACK_SUMMARY.md` for architecture overview
2. Review existing modules (e.g., `catalog/`, `collecting/`) for DDD pattern examples
3. Check `IMPLEMENTATION_PLAN.md` for detailed design decisions
4. Run `pnpm dev` to understand SvelteKit routing + Tauri commands

---

**Status**: ✅ **PLANNING PHASE COMPLETE**  
**Next Phase**: Implementation  
**Estimated Start**: After sprint planning  
**Branch**: `014-railway-model-page`

---

*All documentation generated and committed: February 6, 2026*
