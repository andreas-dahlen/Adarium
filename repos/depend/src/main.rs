#[path = "cli/cli.rs"]
mod cli;
#[path = "config/config.rs"]
mod config;
#[path = "dependencies/dependencies.rs"]
mod dependencies;
#[path = "graphviz/graphviz.rs"]
mod graphviz;
#[path = "lsp/lsp.rs"]
mod lsp;
#[path = "namespaces/namespaces.rs"]
mod namespaces;
#[path = "violations/violations.rs"]
mod violations;

use std::path::Path;
use std::process::ExitCode;

use crate::cli::handle_cli;
use crate::config::load_config;
use crate::dependencies::create_dependency_graph;
use crate::graphviz::build_graphviz;
use crate::namespaces::resolve_namespaces;
use crate::violations::find_violations;

#[tokio::main]
async fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = args.first().map(String::as_str).unwrap_or("analyze-rules");

    let verbose = args.iter().any(|arg| arg == "--verbose");

    let log_level = if verbose { "debug" } else { "error" };

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    let root = match resolve_root(&args) {
        Ok(root) => root,
        Err(error) => {
            eprintln!("Failed to resolve project directory: {error}");
            return ExitCode::from(2);
        }
    };

    let result: Result<ExitCode, Box<dyn std::error::Error>> = async {
        match command {
            "build-graph" => {
                let _graph = create_dependency_graph(&root)?;
                // Export or otherwise output the graph
                Ok(std::process::ExitCode::SUCCESS)
            }
            "build-graphviz" => {
                let graph = create_dependency_graph(&root)?;
                build_graphviz(&root, &graph)?;
                Ok(std::process::ExitCode::SUCCESS)
            }
            "analyze-rules" => {
                let graph = create_dependency_graph(&root)?;

                let config = load_config(&root)?;
                let namespaces = resolve_namespaces(&config.folders, &config.files, &graph)?;
                let violations = find_violations(&config.rules, &namespaces.namespace_edges);
                let has_violations = handle_cli(&violations)?;

                if has_violations {
                    Ok(std::process::ExitCode::FAILURE)
                } else {
                    Ok(std::process::ExitCode::SUCCESS)
                }
            }
            "e2e" => {
                let graph = create_dependency_graph(&root)?;
                build_graphviz(&root, &graph)?;

                let config = load_config(&root)?;
                let namespaces = resolve_namespaces(&config.folders, &config.files, &graph)?;
                let violations = find_violations(&config.rules, &namespaces.namespace_edges);
                let has_violations = handle_cli(&violations)?;

                if has_violations {
                    Ok(std::process::ExitCode::FAILURE)
                } else {
                    Ok(std::process::ExitCode::SUCCESS)
                }
            }
            "lsp" => {
                lsp::lsp_start(&root).await;

                Ok(ExitCode::SUCCESS)
            }
            _ => Err(format!("Unknown command: {command}").into()),
        }
    }
    .await;

    match result {
        Ok(exit_code) => exit_code,
        Err(error) => {
            eprintln!("Application failed: {error}");
            ExitCode::from(2)
        }
    }
}

fn resolve_root(args: &[String]) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let root = match args.iter().position(|arg| arg == "--root") {
        Some(index) => args
            .get(index + 1)
            .filter(|value| !value.starts_with("--"))
            .ok_or("--root requires a directory argument")?,
        None => {
            return Ok(Path::new(".").canonicalize()?);
        }
    };

    Ok(Path::new(root).canonicalize()?)
}
