use std::path::Path;

use crate::graphviz::DependencyGraph;

pub(super) fn write_folder_cluster(
    output: &mut String,
    folder: &Path,
    graph: &DependencyGraph,
    indent: usize,
    cluster_id: &mut usize,
) {
    let id = *cluster_id;
    *cluster_id += 1;

    let padding = " ".repeat(indent);

    output.push_str(&format!("{padding}subgraph cluster_{id} {{\n"));

    output.push_str(&format!(
        "{padding}    label = \"{}\";\n",
        folder.file_name().unwrap_or_default().to_string_lossy()
    ));

    for node in &graph.nodes {
        if node.parent() == Some(folder) {
            let label = node.file_name().unwrap_or_default().to_string_lossy();

            output.push_str(&format!(
                "{padding}    \"{}\" [label=\"{}\"];\n",
                node.display(),
                label
            ));
        }
    }

    for child_folder in &graph.folders {
        if child_folder.parent() == Some(folder) {
            write_folder_cluster(output, child_folder, graph, indent + 4, cluster_id);
        }
    }
    output.push_str(&format!("{padding}}}\n"));
}
