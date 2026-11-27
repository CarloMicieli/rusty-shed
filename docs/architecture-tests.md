# Rust Backend Architecture Tests

The architecture suite turns important decisions in the accepted Architecture
Decision Records (ADRs) into executable safeguards. These tests do not replace
unit, application, or database integration tests; they prevent changes from
silently moving framework concerns across the Rust backend's layer boundaries.

The suite is a standalone Cargo integration test:

```bash
pnpm rust:architecture
```

It is also run as a named step in `.github/workflows/ci.yml` for pull requests
and pushes to `main`.

## Enforced rules

| Rule                                                                                         | ADRs             |
| -------------------------------------------------------------------------------------------- | ---------------- |
| Domain source files may not depend on the Tauri runtime                                      | 0004, 0011       |
| Domain/application internal-layer debt cannot grow; application Tauri/SQLx edges cannot grow | 0004, 0011       |
| Tauri commands remain in interface/command modules or the crate bootstrap                    | 0001, 0002, 0004 |
| Every Tauri command participates in Specta type generation                                   | 0002, 0010       |
| Every Tauri command has structured tracing and redacts transport/state payloads              | 0012             |

The dependency rules cover the current feature layout (`domain`,
`application`, `infrastructure`, and `interface`) rather than enforcing a
particular filename inside a layer. Selectors fail when they match no files, so
renaming the architecture cannot silently make a test pass.

## Known-dependency ratchet

Some bounded contexts predate the strictest wording of ADR 0004. They already
use Tauri, SQLx, interface types, or concrete infrastructure from application
and domain modules. Those exact source/dependency classes are listed in
`KNOWN_OUTWARD_DEPENDENCIES` in
`src-tauri/tests/architecture/known_debt.rs`.

The ratchet is intentionally two-sided:

- introducing an unlisted outward dependency fails the suite;
- removing a dependency also fails until its stale ratchet entry is deleted.

Do not add an entry merely to make a new architecture test pass. New code should
define a port in the inner layer and implement or compose it at the outer edge.
An existing list entry may only be removed after its dependency is actually
eliminated.

## Tooling

`archunit` provides Cargo-native, file/module dependency analysis and
actionable source-to-target violation reports. `cargo-modules` is useful for
visualizing graphs but does not provide the same assertion DSL; `cargo-deny`
continues to enforce crate licenses and sources rather than intra-crate layer
boundaries.

`syn` parses command attributes so the suite can reliably distinguish
`#[tauri::command]`, `#[specta::specta]`, and redacted tracing instrumentation
without depending on source formatting.

## Adding a rule

1. Identify the accepted ADR and quote the concrete architectural decision in
   the test comment.
2. Prefer an ArchUnitRust file/layer rule when the decision concerns source
   dependencies. Use the shared parser only when the decision concerns Rust
   syntax or attributes.
3. Give the test a descriptive name and include the violated file/dependency
   plus ADR identifier in its failure message.
4. Keep tests independent of file ordering and incidental formatting.
5. Run `pnpm rust:architecture`, the negative probe described below, and the
   normal Rust quality checks.
6. Update this document when the enforced rule set or its limitations change.

## Limitations

- The dependency extractor reads Rust syntax and sees all `cfg` branches. It
  does not expand procedural macros or inspect generated/build-script source.
- Some domain types currently derive `sqlx::Type`; removing persistence
  coupling from all value objects is a separate migration. The suite documents
  that debt and prevents Tauri from entering domain modules, but it does not
  claim all domain types are persistence-neutral.
- Unit of Work behavior, domain-event consumption, foreign keys, embedded
  migrations, translation search, and N+1 avoidance are runtime properties.
  They remain covered by behavior/database tests rather than brittle string
  checks.
- ADR 0006's semantic distinction between allowed catalog identifiers/read
  models and prohibited aggregate enrichment still requires review.
- The command tests verify Specta and tracing attributes. They cannot prove
  runtime trace IDs, error categorization, or that every Specta command is
  registered. CI's generated-binding freshness check covers the generated
  bridge.

When investigating a regression, the tests should be made to fail with the
representative violation rather than relying only on a passing baseline. A
temporary `tauri` import in a domain source file is a suitable probe; revert
that import after confirming the architecture test reports it.
