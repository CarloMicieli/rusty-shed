use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use syn::visit::Visit;
use syn::{Attribute, File, ItemFn, ItemMod, ItemUse, Meta, Path as SynPath, UseTree};

pub struct RustSource {
    pub path: PathBuf,
    pub syntax: File,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DependencyDebt {
    pub source: String,
    pub dependency: String,
}

pub fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn production_sources() -> Vec<RustSource> {
    let root = crate_root();
    let source_root = root.join("src");
    let mut paths = Vec::new();
    collect_rust_files(&source_root, &mut paths);
    paths.sort();

    paths
        .into_iter()
        .map(|path| {
            let source = fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
            let syntax = syn::parse_file(&source)
                .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.display()));
            let relative = path
                .strip_prefix(&root)
                .expect("source file must be below the crate root")
                .to_path_buf();

            RustSource {
                path: relative,
                syntax,
            }
        })
        .collect()
}

pub fn relative_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

pub fn command_functions(sources: &[RustSource]) -> Vec<(&Path, &ItemFn)> {
    let mut commands = Vec::new();

    for source in sources {
        let mut visitor = FunctionVisitor {
            functions: Vec::new(),
        };
        visitor.visit_file(&source.syntax);
        commands.extend(
            visitor
                .functions
                .into_iter()
                .filter(|function| has_attribute(&function.attrs, &["tauri", "command"]))
                .map(|function| (source.path.as_path(), function)),
        );
    }

    commands
}

pub fn has_attribute(attributes: &[Attribute], expected: &[&str]) -> bool {
    attributes
        .iter()
        .any(|attribute| path_is(attribute.path(), expected))
}

pub fn instrument_redacts_payload(attributes: &[Attribute]) -> bool {
    attributes
        .iter()
        .filter(|attribute| path_is(attribute.path(), &["tracing", "instrument"]))
        .any(|attribute| {
            let Meta::List(arguments) = &attribute.meta else {
                return false;
            };
            let arguments = arguments.tokens.to_string();
            arguments.contains("skip_all") || arguments.contains("skip (")
        })
}

pub fn instrument_declares_error_fields(attributes: &[Attribute]) -> bool {
    attributes
        .iter()
        .filter(|attribute| path_is(attribute.path(), &["tracing", "instrument"]))
        .any(|attribute| {
            let Meta::List(arguments) = &attribute.meta else {
                return false;
            };
            let arguments = arguments.tokens.to_string();
            arguments.contains("error . type") && arguments.contains("error . message")
        })
}

pub fn observed_outward_dependencies(sources: &[RustSource]) -> BTreeSet<DependencyDebt> {
    let mut dependencies = BTreeSet::new();

    for source in sources {
        let Some(source_layer) = source_layer(&source.path) else {
            continue;
        };

        if source_layer != "domain" && source_layer != "application" {
            continue;
        }

        let mut visitor = DependencyVisitor::default();
        visitor.visit_file(&source.syntax);

        for path in visitor.paths {
            let Some(dependency) = classify_dependency(source_layer, &path) else {
                continue;
            };

            dependencies.insert(DependencyDebt {
                source: relative_path(&source.path),
                dependency,
            });
        }
    }

    dependencies
}

fn classify_dependency(source_layer: &str, path: &[String]) -> Option<String> {
    if path.first().is_some_and(|part| part == "tauri")
        || (source_layer == "application" && path.first().is_some_and(|part| part == "sqlx"))
    {
        return Some(format!("external::{}", path.first()?));
    }

    if path.first().is_none_or(|part| part != "crate") {
        return None;
    }

    let forbidden_layer = path.iter().position(|part| {
        part == "application" || part == "infrastructure" || part == "interface"
    })?;

    let is_forbidden = match source_layer {
        "domain" => true,
        "application" => path[forbidden_layer] != "application",
        _ => false,
    };

    is_forbidden.then(|| path[..=forbidden_layer].join("::"))
}

fn source_layer(path: &Path) -> Option<&str> {
    path.components().find_map(|component| {
        let value = component.as_os_str().to_str()?;
        matches!(value, "domain" | "application").then_some(value)
    })
}

fn path_is(path: &SynPath, expected: &[&str]) -> bool {
    let actual: Vec<_> = path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect();
    actual == expected
}

fn is_cfg_test(attributes: &[Attribute]) -> bool {
    attributes.iter().any(|attribute| {
        let Meta::List(arguments) = &attribute.meta else {
            return false;
        };
        attribute.path().is_ident("cfg") && arguments.tokens.to_string() == "test"
    })
}

fn collect_rust_files(directory: &Path, files: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", directory.display()));

    for entry in entries {
        let path = entry
            .unwrap_or_else(|error| {
                panic!("failed to read entry in {}: {error}", directory.display())
            })
            .path();

        if path.is_dir() {
            collect_rust_files(&path, files);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }
}

struct FunctionVisitor<'ast> {
    functions: Vec<&'ast ItemFn>,
}

impl<'ast> Visit<'ast> for FunctionVisitor<'ast> {
    fn visit_item_fn(&mut self, function: &'ast ItemFn) {
        if !is_cfg_test(&function.attrs) {
            self.functions.push(function);
            syn::visit::visit_item_fn(self, function);
        }
    }

    fn visit_item_mod(&mut self, item: &'ast ItemMod) {
        if !is_cfg_test(&item.attrs) {
            syn::visit::visit_item_mod(self, item);
        }
    }
}

#[derive(Default)]
struct DependencyVisitor {
    paths: Vec<Vec<String>>,
}

impl<'ast> Visit<'ast> for DependencyVisitor {
    fn visit_item_use(&mut self, item: &'ast ItemUse) {
        if !is_cfg_test(&item.attrs) {
            flatten_use_tree(&item.tree, &mut Vec::new(), &mut self.paths);
        }
    }

    fn visit_item_mod(&mut self, item: &'ast ItemMod) {
        if !is_cfg_test(&item.attrs) {
            syn::visit::visit_item_mod(self, item);
        }
    }

    fn visit_item_fn(&mut self, item: &'ast ItemFn) {
        if !is_cfg_test(&item.attrs) {
            syn::visit::visit_item_fn(self, item);
        }
    }

    fn visit_path(&mut self, path: &'ast SynPath) {
        if let Some(first) = path.segments.first() {
            let first = first.ident.to_string();
            if first == "crate" || first == "tauri" || first == "sqlx" {
                self.paths.push(
                    path.segments
                        .iter()
                        .map(|segment| segment.ident.to_string())
                        .collect(),
                );
            }
        }

        syn::visit::visit_path(self, path);
    }
}

fn flatten_use_tree(tree: &UseTree, prefix: &mut Vec<String>, paths: &mut Vec<Vec<String>>) {
    match tree {
        UseTree::Path(path) => {
            prefix.push(path.ident.to_string());
            flatten_use_tree(&path.tree, prefix, paths);
            prefix.pop();
        }
        UseTree::Name(name) => {
            let mut path = prefix.clone();
            path.push(name.ident.to_string());
            paths.push(path);
        }
        UseTree::Rename(rename) => {
            let mut path = prefix.clone();
            path.push(rename.ident.to_string());
            paths.push(path);
        }
        UseTree::Glob(_) => {
            let mut path = prefix.clone();
            path.push("*".to_string());
            paths.push(path);
        }
        UseTree::Group(group) => {
            for item in &group.items {
                flatten_use_tree(item, prefix, paths);
            }
        }
    }
}
