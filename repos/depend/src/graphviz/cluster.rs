use crate::{dependencies::DependencyGraph, graphviz::folder_cluster};

pub(super) fn create_cluster(graph: &DependencyGraph) -> String {
    let mut output = String::from("digraph Dependencies {\n");

    let mut cluster_id = 0;

    for folder in &graph.folders {
        if folder
            .parent()
            .is_some_and(|parent| parent.as_os_str().is_empty())
        {
            folder_cluster::write_folder_cluster(&mut output, folder, graph, 4, &mut cluster_id);
        }
    }

    for node in &graph.nodes {
        if node
            .parent()
            .is_some_and(|parent| parent.as_os_str().is_empty())
        {
            output.push_str(&format!("    \"{}\";\n", node.display()));
        }
    }

    for edge in &graph.edges {
        output.push_str(&format!(
            "    \"{}\" -> \"{}\"[color=blue, style=dashed, rank=same];\n",
            edge.source.display(),
            edge.dependency.resolved_path.display()
        ));
    }

    output.push_str("}\n");

    output
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::dependencies::{
        Dependency, DependencyEdge, DependencyKind, ResolvedDependency, SourceExcerpt,
        SourceLocation, SourcePosition,
    };

    #[test]
    fn exports_nodes_and_edges() {
        let graph = DependencyGraph {
            nodes: vec![PathBuf::from("src/main.ts"), PathBuf::from("src/utils.ts")],
            folders: vec![],
            edges: vec![DependencyEdge {
                source: PathBuf::from("src/main.ts"),
                dependency: ResolvedDependency {
                    dependency: Dependency {
                        source: "./utils".to_owned(),
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
                    resolved_path: PathBuf::from("src/utils.ts"),
                },
            }],
        };

        let output = create_cluster(&graph);

        assert_eq!(
            output,
            concat!(
                "digraph Dependencies {\n",
                "    \"src/main.ts\";\n",
                "    \"src/utils.ts\";\n",
                "    \"src/main.ts\" -> \"src/utils.ts\";\n",
                "}\n",
            )
        );
    }
}
