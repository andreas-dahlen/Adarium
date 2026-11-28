use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use oxc_resolver::{ResolveOptions, Resolver};

use crate::dependencies::source::{Dependency, ProjectSourceData};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedDependency {
    pub dependency: Dependency,
    pub resolved_path: PathBuf,
}

fn create_resolver() -> Resolver {
    let options = ResolveOptions {
        extensions: vec![
            ".ts".into(),
            ".tsx".into(),
            ".js".into(),
            ".jsx".into(),
            ".mts".into(),
            ".cts".into(),
            ".mjs".into(),
            ".cjs".into(),
        ],
        ..ResolveOptions::default()
    };

    Resolver::new(options)
}

pub(super) fn resolve_file_dependencies(
    root: &Path,
    source_path: &Path,
    dependencies: &[Dependency],
) -> Vec<ResolvedDependency> {
    let resolver = create_resolver();
    resolve_dependencies_for_file(root, source_path, dependencies, &resolver)
}

pub(super) fn resolve_dependencies(
    root: &Path,
    project_data: &ProjectSourceData,
) -> HashMap<PathBuf, Vec<ResolvedDependency>> {
    let resolver = create_resolver();
    let mut dependencies: HashMap<PathBuf, Vec<ResolvedDependency>> = HashMap::new();

    for (path, data) in &project_data.files {
        let resolved = resolve_dependencies_for_file(root, path, &data.dependencies, &resolver);

        if !resolved.is_empty() {
            dependencies.insert(path.clone(), resolved);
        }
    }

    dependencies
}

fn resolve_dependencies_for_file(
    root: &Path,
    source_path: &Path,
    dependencies: &[Dependency],
    resolver: &Resolver,
) -> Vec<ResolvedDependency> {
    let mut resolved_dependencies = Vec::new();

    let Some(parent) = source_path.parent() else {
        return resolved_dependencies;
    };

    let base_directory = root.join(parent);

    for dependency in dependencies {
        if !dependency.source.starts_with('.') {
            continue;
        }

        match resolver.resolve(&base_directory, &dependency.source) {
            Ok(resolution) => {
                let absolute_path = resolution.path();

                let Ok(relative_path) = absolute_path.strip_prefix(root) else {
                    continue;
                };

                let resolved_path = relative_path.to_path_buf();

                if resolved_path == source_path || !absolute_path.is_file() {
                    continue;
                }

                resolved_dependencies.push(ResolvedDependency {
                    dependency: dependency.clone(),
                    resolved_path,
                });
            }
            Err(error) => {
                log::warn!(
                    "Could not resolve import '{}' from '{}': {}",
                    dependency.source,
                    source_path.display(),
                    error
                );
            }
        }
    }

    resolved_dependencies
}

#[cfg(test)]
mod tests {
    use crate::dependencies::source::{
        FileData,
        process_file::{DependencyKind, SourceExcerpt, SourceLocation, SourcePosition},
    };

    use super::*;

    fn create_dependency(source: &str) -> Dependency {
        Dependency {
            source: source.to_owned(),
            kind: DependencyKind::Import,
            location: SourceLocation {
                start: SourcePosition {
                    offset: 0,
                    line: 1,
                    column: 1,
                },
                end: SourcePosition {
                    offset: 0,
                    line: 1,
                    column: 1,
                },
            },
            excerpt: SourceExcerpt {
                before: None,
                line: String::new(),
                after: None,
            },
        }
    }

    fn create_project_source_data(root: &Path) -> ProjectSourceData {
        let src = root.join("src");

        std::fs::create_dir_all(src.join("utils")).unwrap();
        std::fs::create_dir_all(src.join("components")).unwrap();

        std::fs::write(src.join("main.ts"), "").unwrap();
        std::fs::write(src.join("utils/date.ts"), "").unwrap();
        std::fs::write(src.join("components/Button.tsx"), "").unwrap();

        let mut files = HashMap::new();

        files.insert(
            PathBuf::from("src/main.ts"),
            FileData {
                dependencies: vec![
                    create_dependency("./utils/date"),
                    create_dependency("./components/Button"),
                    create_dependency("vue"),
                ],
            },
        );

        files.insert(PathBuf::from("src/utils/date.ts"), FileData::default());

        files.insert(
            PathBuf::from("src/components/Button.tsx"),
            FileData::default(),
        );

        ProjectSourceData {
            files,
            folders: vec![],
        }
    }

    #[test]
    fn resolves_relative_imports() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        let project_data = create_project_source_data(root);

        let dependencies = resolve_dependencies(root, &project_data);

        let resolved = dependencies.get(Path::new("src/main.ts")).unwrap();

        assert_eq!(resolved.len(), 2);

        assert_eq!(resolved[0].dependency.source, "./utils/date");
        assert_eq!(
            resolved[0].resolved_path,
            PathBuf::from("src/utils/date.ts")
        );

        assert_eq!(resolved[1].dependency.source, "./components/Button");
        assert_eq!(
            resolved[1].resolved_path,
            PathBuf::from("src/components/Button.tsx")
        );
    }

    #[test]
    fn resolves_single_file_dependencies() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        let project_data = create_project_source_data(root);

        let file_data = project_data.files.get(Path::new("src/main.ts")).unwrap();

        let resolved =
            resolve_file_dependencies(root, Path::new("src/main.ts"), &file_data.dependencies);

        assert_eq!(resolved.len(), 2);

        assert_eq!(resolved[0].dependency.source, "./utils/date");
        assert_eq!(
            resolved[0].resolved_path,
            PathBuf::from("src/utils/date.ts")
        );

        assert_eq!(resolved[1].dependency.source, "./components/Button");
        assert_eq!(
            resolved[1].resolved_path,
            PathBuf::from("src/components/Button.tsx")
        );
    }

    #[test]
    fn ignores_external_imports() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        let mut project_data = create_project_source_data(root);

        project_data.files.insert(
            PathBuf::from("src/main.ts"),
            FileData {
                dependencies: vec![create_dependency("vue")],
            },
        );

        let dependencies = resolve_dependencies(root, &project_data);

        assert!(dependencies.is_empty());
    }

    #[test]
    fn ignores_unresolved_imports() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        let mut project_data = create_project_source_data(root);

        project_data.files.insert(
            PathBuf::from("src/main.ts"),
            FileData {
                dependencies: vec![create_dependency("./missing")],
            },
        );

        let dependencies = resolve_dependencies(root, &project_data);

        assert!(dependencies.is_empty());
    }
}
