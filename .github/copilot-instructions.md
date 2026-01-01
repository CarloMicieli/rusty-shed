# GitHub Copilot Instructions: Tauri 2 + Svelte 5 Clean Architecture

## 1. Core Tech Stack

* **Frontend:** Svelte 5 (Runes), Tailwind 4, Skeleton 4.x.
* **Backend:** Rust, Tauri 2.
* **Localization:** Paraglide-JS (`import * as m from '$paraglide/messages'`).
* **Error Handling:** `thiserror` (Rust) mapped to localized strings (TS).

---

## 2. Architectural Boundaries

### Feature-Based Folders

Organize by feature in `src/lib/features/[feature_name]/`:

* `ui/`: Svelte components.
* `service.ts`: The **IPC Bridge**. Contains a class using Runes to manage state.
* `types.ts`: TypeScript interfaces for the feature.

### Rust Layering (Clean Architecture)

* **Commands:** Thin wrappers in `src/commands.rs` or `src/features/[feature]/mod.rs`.
* **Use Cases:** Business logic orchestration.
* **Domain:** Entities and Repository traits.
* **Infrastructure:** Database/API implementations.

---

## 3. Frontend Implementation Rules (Svelte 5)

* **Runes Only:** Use `$state`, `$derived`, `$props`, and `$effect`. **No** `export let` or Svelte 4 stores.
* **Events:** Use standard attributes (e.g., `onclick={...}`) instead of `on:click`.
* **Snippets:** Use `{#snippet name()}` instead of `<slot />`.
* **No Raw Invoke:** Never call `invoke()` inside a `.svelte` component.

### The Service Pattern Template

Every IPC interaction must follow this structure in `service.ts`:

```typescript
import { invoke } from "@tauri-apps/api/core";
import * as m from '$paraglide/messages';

export class FeatureState {
    data = $state<T | null>(null);
    isLoading = $state(false);
    error = $state<string | null>(null);

    async execute() {
        this.isLoading = true;
        this.error = null;
        try {
            this.data = await invoke("plugin:feature|command");
        } catch (e: any) {
            // Map Rust 'thiserror' tag to Paraglide message
            this.error = this.mapError(e.type);
        } finally {
            this.isLoading = false;
        }
    }

    private mapError(type: string): string {
        const errorMap: Record<string, string> = {
            'AuthError': m.error_unauthorized(),
            'DatabaseError': m.error_db_fail()
        };
        return errorMap[type] ?? m.error_generic();
    }
}

```

---

## 4. Backend Implementation Rules (Rust)

* **Error Enums:** Use `thiserror` and tag for TypeScript compatibility.
```rust
#[derive(serde::Serialize, thiserror::Error, Debug)]
#[serde(tag = "type", content = "details")]
pub enum AppError {
    #[error("Unauthorized")]
    AuthError,
}

```


* **Thin Commands:** Commands should only extract state and call a Use Case.
* **Safety:** Prefer `Result<T, AppError>` for all command returns.

---

## 5. Quality & Workflow

### Mandatory Verification

Before declaring a task finished, ensure:

1. **Frontend:** `pnpm check` and `pnpm lint` pass.
2. **Backend:** `cargo fmt` and `cargo clippy -- -D warnings` pass.
3. **Localization:** No hardcoded strings in UI; all text comes from `* as m`.

### Commit Guidelines

Follow Conventional Commits: `feat:`, `fix:`, `docs:`, `refactor:`, `perf:`, `test:`, `build:`, `ci:`, `chore:`, `revert:`.
