use std::{path::Path, process::Command};

mod cluster;
mod folder_cluster;
mod tree;
use cluster::create_cluster;
use tree::create_tree;

use crate::dependencies::DependencyGraph;

pub fn build_graphviz(
    root: &Path,
    graph: &DependencyGraph,
) -> Result<(), Box<dyn std::error::Error>> {
    let cluster_dot = create_cluster(graph);
    let tree_dot = create_tree(graph);

    write_dot(root, "cluster", &cluster_dot)?;
    write_dot(root, "tree", &tree_dot)?;

    Ok(())
}

fn write_dot(root: &Path, name: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dot = root.join(format!("{name}.dot"));
    let svg = root.join(format!("{name}.svg"));

    std::fs::write(&dot, content)?;

    let status = Command::new("dot")
        .arg("-Tsvg")
        .arg(&dot)
        .arg("-o")
        .arg(&svg)
        .status()?;

    if !status.success() {
        return Err(format!("Graphviz failed to render {name}").into());
    }

    Ok(())
}
