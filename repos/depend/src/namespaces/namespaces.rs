use std::{collections::HashSet, path::PathBuf};

use crate::{
    config::NamespacePattern,
    dependencies::{DependencyEdge, DependencyGraph, FileDependencies},
    namespaces::{
        resolve_dependencies::resolve_dependencies,
        resolve_memberships::{NamespaceMemberships, resolve_memberships},
    },
};

pub use resolve_memberships::NamespaceMembership;

pub use resolve_dependencies::NamespaceDependencyEdge;

mod resolve_dependencies;
mod resolve_memberships;

pub struct ResolvedNamespaces {
    pub namespaces: NamespaceMemberships,
    pub namespace_edges: Vec<NamespaceDependencyEdge>,
}

pub(super) fn resolve_namespaces(
    folders: &[NamespacePattern],
    files: &[NamespacePattern],
    graph: &DependencyGraph,
) -> Result<ResolvedNamespaces, Box<dyn std::error::Error>> {
    let namespaces = resolve_memberships(folders, files, &graph.nodes, &graph.folders)?;

    let namespace_edges = resolve_dependencies(&graph.edges, &namespaces.files);

    Ok(ResolvedNamespaces {
        namespaces,
        namespace_edges,
    })
}

pub(super) fn resolve_namespace_file(
    folders: &[NamespacePattern],
    files: &[NamespacePattern],
    file_dependencies: &FileDependencies,
) -> Result<ResolvedNamespaces, Box<dyn std::error::Error>> {
    let mut file_paths = vec![file_dependencies.source.clone()];

    file_paths.extend(
        file_dependencies
            .dependencies
            .iter()
            .map(|dependency| dependency.resolved_path.clone()),
    );
    let mut folder_paths = HashSet::new();

    for path in &file_paths {
        let mut parent = path.parent();

        while let Some(folder) = parent {
            if folder.as_os_str().is_empty() {
                break;
            }
            folder_paths.insert(folder.to_path_buf());
            parent = folder.parent();
        }
    }
    let folder_paths: Vec<PathBuf> = folder_paths.into_iter().collect();

    let namespaces = resolve_memberships(folders, files, &file_paths, &folder_paths)?;

    let edges: Vec<DependencyEdge> = file_dependencies
        .dependencies
        .iter()
        .map(|dependency| DependencyEdge {
            source: file_dependencies.source.clone(),
            dependency: dependency.clone(),
        })
        .collect();

    let namespace_edges = resolve_dependencies(&edges, &namespaces.files);

    Ok(ResolvedNamespaces {
        namespaces,
        namespace_edges,
    })
}
