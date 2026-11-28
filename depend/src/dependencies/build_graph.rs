use std::{collections::HashMap, path::PathBuf};

use crate::dependencies::{resolve_dependencies::ResolvedDependency, source::ProjectSourceData};

#[derive(Debug)]
pub struct DependencyGraph {
    pub nodes: Vec<PathBuf>,
    pub edges: Vec<DependencyEdge>,
    pub folders: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct DependencyEdge {
    pub source: PathBuf,
    pub dependency: ResolvedDependency,
}

pub(super) fn build_graph(
    project_data: &ProjectSourceData,
    deps: &HashMap<PathBuf, Vec<ResolvedDependency>>,
) -> DependencyGraph {
    let nodes = project_data.files.keys().cloned().collect();

    let edges = deps
        .iter()
        .flat_map(|(source, dependencies)| {
            dependencies.iter().map(|dependency| DependencyEdge {
                source: source.clone(),
                dependency: dependency.clone(),
            })
        })
        .collect();

    DependencyGraph {
        nodes,
        edges,
        folders: project_data.folders.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dependencies::{
        resolve_dependencies::ResolvedDependency,
        source::{
            Dependency, FileData,
            process_file::{DependencyKind, SourceExcerpt, SourceLocation, SourcePosition},
        },
    };

    fn create_dependency(source: &str, resolved_path: &str) -> ResolvedDependency {
        ResolvedDependency {
            dependency: Dependency {
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
            },
            resolved_path: PathBuf::from(resolved_path),
        }
    }

    #[test]
    fn builds_graph_with_all_project_data_files_and_dependencies() {
        let mut files = HashMap::new();

        files.insert(PathBuf::from("src/main.ts"), FileData::default());

        files.insert(PathBuf::from("src/utils.ts"), FileData::default());

        files.insert(PathBuf::from("src/config.ts"), FileData::default());

        let project_data = ProjectSourceData {
            files,
            folders: vec![],
        };

        let source = PathBuf::from("src/main.ts");

        let mut deps = HashMap::new();
        deps.insert(
            source.clone(),
            vec![create_dependency("./utils", "src/utils.ts")],
        );

        let graph = build_graph(&project_data, &deps);

        // All project_data files should become nodes.
        assert_eq!(graph.nodes.len(), 3);

        for path in ["src/main.ts", "src/utils.ts", "src/config.ts"] {
            assert!(graph.nodes.contains(&PathBuf::from(path)));
        }

        // The resolved dependency should become an edge.
        assert_eq!(graph.edges.len(), 1);

        let edge = &graph.edges[0];

        assert_eq!(edge.source, source);
        assert_eq!(edge.dependency.dependency.source, "./utils");
    }

    #[test]
    fn builds_graph_without_dependencies() {
        let mut files = HashMap::new();

        files.insert(PathBuf::from("src/main.ts"), FileData::default());

        let project_data = ProjectSourceData {
            files,
            folders: vec![],
        };

        let graph = build_graph(&project_data, &HashMap::new());

        assert_eq!(graph.nodes.len(), 1);
        assert!(graph.edges.is_empty());
    }
}
