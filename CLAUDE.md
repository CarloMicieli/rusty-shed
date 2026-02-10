# Claude Code Instructions for Rusty Shed

This file contains project-specific instructions and conventions for working on the Rusty Shed model railway management application.

## Project Overview

Rusty Shed is a Tauri 2.x desktop application for managing model railway collections, built with:

- **Backend**: Rust with clean architecture and domain-driven design
- **Frontend**: SvelteKit with Svelte 5 (runes), Tailwind CSS 4, Skeleton UI 4.x
- **Database**: SQLite with sqlx migrations
- **Type Safety**: specta/tauri-specta for TypeScript bindings
- **Internationalization**: Paraglide i18n

## Code Quality Workflow

**ALWAYS run these verifications after making code changes:**

1. **Lint**: Run `pnpm lint` to check for ESLint issues
2. **Type Check**: Run `pnpm check` to verify TypeScript/Svelte types
3. **Fix Issues**: Address all errors and warnings before considering work complete

This is non-negotiable and catches issues early.

## File Organization

```
src/
├── lib/
│   ├── components/          # Shared UI components
│   │   └── ui/              # shadcn-svelte components
│   ├── features/            # Feature modules
│   │   └── feature-name/
│   │       ├── components/  # Feature-specific components
│   │       ├── FeatureController.svelte.ts
│   │       └── FeatureState.svelte.ts
│   └── paraglide/           # i18n messages
└── routes/                  # SvelteKit routes

src-tauri/
├── src/
│   ├── domain_name/
│   │   ├── domain/          # Aggregates, value objects, repositories
│   │   ├── application/     # Use cases
│   │   ├── infrastructure/  # Repository implementations
│   │   └── interface/       # Tauri commands
│   └── core/                # Shared domain primitives
└── migrations/              # SQLx migrations
```

## Development Workflow

### Before Committing

1. Run `pnpm format` - to format UI code
2. Run `pnpm lint` - must pass
3. Run `pnpm check` - must pass
4. Run `cargo fmt` - to format rust code
5. Run `cargo clippy` - address warnings
6. Run `cargo test` - all tests must pass
7. Test the UI manually if you made UI changes

### Making Changes

1. **Read before modifying**: Always read files before editing them
2. **Understand context**: Check related files and patterns
3. **Match existing style**: Follow the conventions already in the codebase
4. **Test thoroughly**: Verify changes work as expected
5. **Run verifications**: Lint and type check before considering work complete

## Common Pitfalls

❌ **Don't**:

- Hardcode English strings in components (use Paraglide)
- Create new migrations without rebuilding (`cargo build`)
- Skip lint/check after code changes
- Use `any` types in TypeScript
- Add unused imports

✅ **Do**:

- Use Paraglide for all user-facing text
- Rebuild after creating SQLx migrations
- Run `pnpm lint` and `pnpm check` after changes
- Use proper TypeScript types from bindings
- Remove unused imports immediately

## Component Library

- **UI Components**: shadcn-svelte (Card, Button, Input, Badge, etc.)
- **Icons**: lucide-svelte
- **Toasts**: Custom toaster service (`$lib/toaster`)
- **Theme**: Skeleton UI 4.x with custom surface/primary colors
