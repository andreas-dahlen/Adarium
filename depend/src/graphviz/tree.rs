use crate::dependencies::DependencyGraph;

pub(super) fn create_tree(graph: &DependencyGraph) -> String {
    let mut output = String::from("digraph Tree {\n");

    // Folder relationships
    for folder in &graph.folders {
        let label = folder.file_name().unwrap_or_default().to_string_lossy();

        output.push_str(&format!(
            "    \"{}\" [label=\"{}\", shape=box];\n",
            folder.display(),
            label
        ));

        if let Some(parent) = folder.parent().filter(|p| !p.as_os_str().is_empty()) {
            output.push_str(&format!(
                "    \"{}\" -> \"{}\";\n",
                parent.display(),
                folder.display()
            ));
        }
    }

    // File relationships
    for node in &graph.nodes {
        let label = node.file_name().unwrap_or_default().to_string_lossy();

        output.push_str(&format!(
            "    \"{}\" [label=\"{}\"];\n",
            node.display(),
            label
        ));

        if let Some(parent) = node.parent().filter(|p| !p.as_os_str().is_empty()) {
            output.push_str(&format!(
                "    \"{}\" -> \"{}\";\n",
                parent.display(),
                node.display()
            ));
        }
    }

    // Dependency relationships
    for edge in &graph.edges {
        output.push_str(&format!(
            "    \"{}\" -> \"{}\" [color=blue, style=dashed, constraint=false];\n",
            edge.source.display(),
            edge.dependency.resolved_path.display()
        ));
    }

    output.push_str("}\n");

    output
}
