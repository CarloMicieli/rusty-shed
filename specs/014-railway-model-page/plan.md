# Implementation Plan: Feature 014 - Railway Model Details Page

**Feature ID**: 014  
**Title**: Railway Model Details Page  
**Created**: February 6, 2026  
**Status**: Planning Phase Complete

## Overview

This plan outlines the implementation strategy for Feature 014, which adds a detailed view page for railway models in the Rusty Shed application. The feature is divided into two major components: backend media module and frontend details page UI.

### Scope

- **Backend**: Media module for image handling (5 implementation phases)
- **Frontend**: Railway model details page with tabs (5 implementation phases)
- **Quality**: Integration testing, code quality, and review (3 phases)

### Timeline

- **Estimated Duration**: 2 weeks
- **Team Size**: 2 developers (1 backend, 1 frontend)
- **Dependencies**: None - uses existing tech stack

## Architecture & Stack

### Technology Stack

**Frontend**:

- Svelte 5 with Runes ($state, $derived, $props)
- SvelteKit file-based routing
- Tailwind CSS 4 + shadcn-svelte components
- Paraglide-JS for i18n (EN, IT)
- Vitest + Playwright for testing
- Zod for runtime validation

**Backend**:

- Rust 1.93.0+
- Tauri 2.9.5 (IPC framework)
- Tokio 1.49.0 (async runtime)
- SQLite with sqlx 0.8.6 (type-safe queries)
- specta 2.0.0-rc.22 (TypeScript bindings)
- thiserror 2.0.18 (error handling)

### Architectural Pattern: Domain-Driven Design (DDD)

The project follows a layered DDD architecture with 13 feature modules. Feature 014 adds:

1. **Media Module** (Backend)
   - Domain layer: Image entities and value objects
   - Application layer: Use cases for image/placeholder retrieval
   - Infrastructure layer: File I/O and placeholder generation
   - Interface layer: Tauri commands and DTOs

2. **Routes & Components** (Frontend)
   - Route: `/models/{modelId}` - Details page
   - Components: Header, Details tab, Rolling Stock tab
   - Data binding: Load model data and images
   - Responsive layout: 320px, 768px, 1920px breakpoints

### Data Model

#### Entities (No New Database Tables)

**RailwayModel** (existing)

- id: String (UUID)
- name: String
- manufacturer: String
- product_code: String
- description: String
- scale: String
- era: String
- power_method: String
- created_at: DateTime
- updated_at: DateTime

**RailwayModelImage** (domain entity, no DB persistence)

- id: RailwayModelId
- path: PathBuf (filesystem path)
- exists: bool

**RollingStock** (existing)

- model_id: String (FK to RailwayModel)
- type: String
- road_number: String
- depot: String
- series_code: String
- company: String
- country: String
- livery: String
- specifications: JSON

### File Storage

**Image Storage Location**:

```
{AppLocalDataDir}/models/
├── trn_railway-model_uuid1.png
├── trn_railway-model_uuid2.jpg
└── trn_railway-model_uuid3.jpeg
```

**Filename Convention**:

- Pattern: `{model_id_with_colons_as_underscores}.{ext}`
- Example: Model ID `trn:railway-model:abc123` → `trn_railway-model_abc123.png`
- Supported extensions: `.png`, `.jpg`, `.jpeg`

## Implementation Phases

### Phase 1: Backend Module Setup (T001)

**Objective**: Establish media module structure

**Deliverables**:

- Create `src-tauri/src/media/` module directory
- Create submodule directories: domain/, application/, infrastructure/, interface/
- Create mod.rs files for each layer
- Add media module to `src-tauri/src/lib.rs`

**Files**:

- `src-tauri/src/media/mod.rs`
- `src-tauri/src/media/domain/mod.rs`
- `src-tauri/src/media/application/mod.rs`
- `src-tauri/src/media/infrastructure/mod.rs`
- `src-tauri/src/media/interface/mod.rs`

### Phase 2: Domain Layer (T002-T005)

**Objective**: Define domain entities and value objects

**Deliverables**:

- RailwayModelImage entity
- ImagePlaceholder value object
- ImageError custom error enum
- Business logic for filename resolution

**Key Decisions**:

- Error handling: thiserror crate for ergonomic error types
- Filename resolution: Colon (:) replaced with underscore (\_)
- Path validation: Prevent directory traversal attacks

### Phase 3: Application Layer (T006-T008)

**Objective**: Implement use cases

**Deliverables**:

- GetRailwayModelImage use case
- GetImagePlaceholder use case
- Use case orchestration

**Key Decisions**:

- Async-first design (Tokio runtime)
- Clean separation of concerns

### Phase 4: Infrastructure Layer (T009-T011)

**Objective**: Implement data access and external services

**Deliverables**:

- ImageRepository for filesystem operations
- PlaceholderGenerator for HTML/CSS creation
- File I/O with Tokio
- Path validation/sanitization

**Key Decisions**:

- Async file operations with `tokio::fs`
- Pure HTML/CSS placeholder (no JavaScript)
- Responsive design using Tailwind classes

### Phase 5: Interface & Integration (T012-T016)

**Objective**: Expose media functionality via Tauri commands

**Deliverables**:

- Command handler: `get_railway_model_image`
- DTO: RailwayModelImageResponse
- TypeScript bindings auto-generation
- Integration into lib.rs

**Key Decisions**:

- specta auto-generates TypeScript types
- Bindings at: `src/lib/bindings.ts`
- Error mapping: ImageError → CommandError

### Phase 6: Frontend Routes & Layout (T017-T020)

**Objective**: Create page structure and routing

**Deliverables**:

- Route: `src/routes/models/[modelId]/+page.svelte`
- Layout wrapper
- Responsive container
- Basic header/tab structure

**Key Decisions**:

- SvelteKit file-based routing
- Responsive breakpoints: 320px, 768px, 1920px
- Tailwind classes for layout

### Phase 7: Frontend Components (T021-T024)

**Objective**: Build reusable UI components

**Deliverables**:

- Header component (image, title, badges)
- Details tab content
- Rolling Stock tab content
- Expandable card component for units

**Key Decisions**:

- shadcn-svelte for consistent design
- Component composition (Svelte 5 slots)
- Props with Svelte 5 runes

### Phase 8: Data Binding (T025-T028)

**Objective**: Connect UI to data

**Deliverables**:

- Fetch model data via Tauri command
- Fetch rolling stock data
- Fetch image via media command
- Handle loading/error states

**Key Decisions**:

- Zod for runtime validation
- Error boundaries for graceful degradation
- Loading indicators for UX

### Phase 9: Styling & Localization (T029-T032)

**Objective**: Apply visual design and language support

**Deliverables**:

- Tailwind CSS styling
- Paraglide-JS localization (EN, IT)
- Dark mode support
- Accessibility (WCAG 2.1 AA)

**Key Decisions**:

- No custom CSS (Tailwind only)
- 100% UI string localization via Paraglide
- Semantic HTML for screen readers
- Focus states, color contrast compliance

### Phase 10: Frontend Testing (T033-T036)

**Objective**: Ensure component correctness

**Deliverables**:

- Unit tests (Vitest) for components
- Component snapshot tests
- Integration tests (Playwright) for page flow
- Test coverage >80%

**Key Decisions**:

- Vitest for fast unit testing
- Playwright for E2E testing
- Mock Tauri commands in tests

### Phase 11: Integration & Verification (T037-T039)

**Objective**: Test end-to-end functionality

**Deliverables**:

- Verify backend/frontend communication
- Test image loading (happy path)
- Test placeholder fallback
- Performance profiling

**Key Decisions**:

- Use dev environment for testing
- Load real images for validation
- Monitor page load time

### Phase 12: Code Quality (T040-T043)

**Objective**: Ensure production-ready code

**Deliverables**:

- `cargo fmt` - Code formatting (Rust)
- `cargo clippy` - Linting (Rust)
- ESLint/Prettier - Formatting (TypeScript)
- Comprehensive documentation

**Key Decisions**:

- Zero warnings/clippy violations
- 100% rustdoc comments (public APIs)
- JSDoc for complex functions

### Phase 13: Code Review & Merge (T044-T047)

**Objective**: Quality gate and integration

**Deliverables**:

- Code review by tech lead
- Merge to main branch
- Update CHANGELOG.md
- Close feature branch

**Key Decisions**:

- Minimum 2 reviewers
- All CI checks must pass
- Feature complete before merge

## Dependencies & Integration Points

### Backend Dependencies

- None new (uses existing crates)
- Depends on: RailwayModel entity, existing error handling patterns

### Frontend Dependencies

- Catalog module (for model data)
- Media module (for image retrieval)
- i18n system (Paraglide-JS)

### Database Dependencies

- No new tables (uses existing RailwayModel and RollingStock)
- Filesystem for image storage

## Assumptions & Constraints

### Assumptions

1. Images stored in `{AppLocalDataDir}/models/` directory
2. Model IDs available from catalog module
3. Rolling stock data available via existing query
4. Tauri 2.9.5 supports specta for bindings
5. TypeScript bindings auto-generated to `src/lib/bindings.ts`

### Constraints

- No external image processing library (pure HTML/CSS)
- No database schema changes
- No new npm/cargo dependencies
- Frontend must be 100% localized (no hardcoded strings)
- WCAG 2.1 AA accessibility required

## Risk Factors

| Risk                     | Probability | Impact | Mitigation                                |
| ------------------------ | ----------- | ------ | ----------------------------------------- |
| Image file not found     | Medium      | Low    | Placeholder fallback always available     |
| Path traversal attack    | Low         | High   | Path validation + testing                 |
| Async timing issues      | Low         | Medium | Tokio test framework, integration tests   |
| Type safety (TypeScript) | Low         | Medium | specta auto-generation + testing          |
| Localization coverage    | Medium      | Medium | Paraglide build check + translation audit |

## Success Criteria

1. **Functional**: Page loads model details with image/placeholder
2. **Performance**: Page load <2s on desktop, <3s on mobile
3. **Accessibility**: WCAG 2.1 AA compliance (auto-checked)
4. **Localization**: 100% UI strings in EN/IT
5. **Code Quality**: 0 Clippy warnings, >80% test coverage
6. **Security**: Path validation prevents traversal attacks
7. **UX**: Responsive on 320px-1920px, accessible via keyboard

## Quality Gates

- [ ] All 47 tasks completed
- [ ] All functional requirements (FR-001 to FR-016) implemented
- [ ] All user stories (US-001 to US-003) passing acceptance criteria
- [ ] All success criteria (SC-001 to SC-006) verified
- [ ] Zero Clippy warnings (Rust)
- [ ] Zero ESLint errors (TypeScript)
- [ ] Test coverage >80%
- [ ] WCAG 2.1 AA compliance verified
- [ ] Paraglide localization complete (EN, IT)
- [ ] Performance targets met (<2s desktop, <3s mobile)
- [ ] Code review approved
- [ ] All tests passing in CI/CD

## Next Steps

1. **Sprint Planning**: Import 47 tasks into project management
2. **Team Assignment**: Assign backend and frontend developers
3. **Kickoff**: Daily standups, shared documentation review
4. **Execution**: Follow task breakdown phases sequentially
5. **Quality Assurance**: Run verification scripts before merge
6. **Merge**: Complete feature branch integration

---

**Plan Status**: ✅ Ready for Implementation  
**Approval Date**: February 6, 2026  
**Plan Owner**: Architecture Team
