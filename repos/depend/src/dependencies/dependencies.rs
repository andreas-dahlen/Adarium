mod build_graph;
mod resolve_dependencies;
#[path = "source/source.rs"]
mod source;

pub use build_graph::{DependencyEdge, DependencyGraph};

pub use resolve_dependencies::ResolvedDependency;

pub use source::{Dependency, SourceLocation, SourcePosition};
#[cfg(test)]
pub use source::{DependencyKind, SourceExcerpt};

use std::path::{Path, PathBuf};

use crate::dependencies::{
    resolve_dependencies::resolve_file_dependencies,
    source::{
        build_project_source_data,
        process_file::{process_file, process_source},
    },
};

use build_graph::build_graph;
use resolve_dependencies::resolve_dependencies;

pub fn create_dependency_graph(root: &Path) -> Result<DependencyGraph, Box<dyn std::error::Error>> {
    let exclusions = [".git", "target"];

    let project_data = build_project_source_data(root, &exclusions)?;
    //     log::debug!("project source data: {project_data:#?}");

    let dependencies = resolve_dependencies(root, &project_data);

    Ok(build_graph(&project_data, &dependencies))

    // let dependencies = resolve_dependencies(root, &project_data);
    // log::debug!("Resolved dependencies: {dependencies:#?}");
}

pub struct FileDependencies {
    pub source: PathBuf,
    pub dependencies: Vec<ResolvedDependency>,
}

pub fn create_file_dependency_graph(
    root: &Path,
    relative_path: &Path,
) -> Result<FileDependencies, Box<dyn std::error::Error>> {
    let file_data = process_file(root, relative_path)?;

    let dependencies = resolve_file_dependencies(root, relative_path, &file_data.dependencies);

    Ok(FileDependencies {
        source: relative_path.to_path_buf(),
        dependencies,
    })
}

pub fn create_file_dependency_graph_from_source(
    root: &Path,
    relative_path: &Path,
    source: &str,
) -> Result<FileDependencies, Box<dyn std::error::Error>> {
    let file_data = process_source(source, relative_path);

    let dependencies = resolve_file_dependencies(root, relative_path, &file_data.dependencies);

    Ok(FileDependencies {
        source: relative_path.to_path_buf(),
        dependencies,
    })
}
