use crate::support::{
    command_functions, has_attribute, instrument_declares_error_fields, instrument_redacts_payload,
    production_sources, relative_path,
};

// ADR 0001, ADR 0002, and ADR 0004 confine Tauri transport concerns to the
// interface/bootstrap edge.
#[test]
fn command_surface_is_present() {
    assert!(
        !command_functions(&production_sources()).is_empty(),
        "no Tauri commands were discovered; update the architecture source traversal before disabling the suite"
    );
}

#[test]
fn tauri_commands_are_declared_at_the_interface_edge() {
    let sources = production_sources();
    let violations: Vec<_> = command_functions(&sources)
        .into_iter()
        .filter_map(|(path, function)| {
            let relative = relative_path(path);
            let is_interface = path.components().any(|component| {
                component.as_os_str().to_string_lossy() == "interface"
            });
            let is_command_module = path
                .file_name()
                .is_some_and(|name| name == "commands.rs");
            let is_crate_bootstrap = relative == "src/lib.rs";

            (!(is_interface || is_command_module || is_crate_bootstrap)).then(|| {
                format!(
                    "{}: tauri command `{}` is outside the interface/bootstrap edge (ADR 0002, ADR 0004)",
                    relative,
                    function.sig.ident
                )
            })
        })
        .collect();

    assert!(
        violations.is_empty(),
        "Tauri command placement violations:\n{}",
        violations.join("\n")
    );
}

// ADR 0002 and ADR 0010 require all IPC commands to participate in Specta
// binding generation.
#[test]
fn tauri_commands_participate_in_specta_type_generation() {
    let sources = production_sources();
    let violations: Vec<_> = command_functions(&sources)
        .into_iter()
        .filter(|(_, function)| !has_attribute(&function.attrs, &["specta", "specta"]))
        .map(|(path, function)| {
            format!(
                "{}: tauri command `{}` is missing #[specta::specta] (ADR 0002, ADR 0010)",
                relative_path(path),
                function.sig.ident
            )
        })
        .collect();

    assert!(
        violations.is_empty(),
        "Specta command violations:\n{}",
        violations.join("\n")
    );
}

// ADR 0012 requires redacted, structured tracing at the command boundary.
#[test]
fn tauri_commands_redact_payloads_and_record_trace_context() {
    let sources = production_sources();
    let violations: Vec<_> = command_functions(&sources)
        .into_iter()
        .filter_map(|(path, function)| {
            if !has_attribute(&function.attrs, &["tracing", "instrument"]) {
                return Some(format!(
                    "{}: tauri command `{}` is missing #[tracing::instrument] (ADR 0012)",
                    relative_path(path),
                    function.sig.ident
                ));
            }

            (!instrument_redacts_payload(&function.attrs)).then(|| {
                format!(
                    "{}: tauri command `{}` must use tracing skip_all or skip(...) to redact transport payloads (ADR 0012)",
                    relative_path(path),
                    function.sig.ident
                )
            })
        })
        .collect();

    let missing_error_fields: Vec<_> = command_functions(&sources)
        .into_iter()
        .filter(|(_, function)| !instrument_declares_error_fields(&function.attrs))
        .map(|(path, function)| {
            format!(
                "{}: tauri command `{}` must declare tracing error.type and error.message fields (ADR 0012)",
                relative_path(path),
                function.sig.ident
            )
        })
        .collect();

    assert!(
        violations.is_empty() && missing_error_fields.is_empty(),
        "Tauri tracing/redaction violations:\n{}",
        violations
            .into_iter()
            .chain(missing_error_fields)
            .collect::<Vec<_>>()
            .join("\n")
    );
}
