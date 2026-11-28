use std::{collections::HashMap, path::PathBuf};

use crate::{
    dependencies::{Dependency, DependencyEdge},
    namespaces::resolve_memberships::NamespaceMembership,
};

pub struct NamespaceDependencyEdge {
    pub source: PathBuf,
    pub source_namespaces: NamespaceMembership,
    pub dependency: PathBuf,
    pub dependency_namespaces: NamespaceMembership,
    pub dependency_metadata: Dependency,
}

pub(super) fn resolve_dependencies(
    edges: &[DependencyEdge],
    namespace_files: &HashMap<PathBuf, NamespaceMembership>,
) -> Vec<NamespaceDependencyEdge> {
    let mut dependency_edges = Vec::new();

    for edge in edges {
        let source_namespace = namespace_files
            .get(&edge.source)
            .cloned()
            .unwrap_or_default();

        let dependency_namespace = namespace_files
            .get(&edge.dependency.resolved_path)
            .cloned()
            .unwrap_or_default();

        dependency_edges.push(NamespaceDependencyEdge {
            source: edge.source.clone(),
            source_namespaces: source_namespace,
            dependency: edge.dependency.resolved_path.clone(),
            dependency_namespaces: dependency_namespace,
            dependency_metadata: edge.dependency.dependency.clone(),
        });
    }

    dependency_edges
}
