use archunit::{assert_passes, project_files};

// ADR 0004 and ADR 0011 require the domain to remain independent of the Tauri
// runtime and its IPC framework.
#[test]
fn domain_does_not_depend_on_tauri() {
    let rule = project_files()
        .in_path("src/**/domain/**")
        .should_not()
        .depend_on_external_modules()
        .matching("tauri");

    assert_passes!(rule);
}
