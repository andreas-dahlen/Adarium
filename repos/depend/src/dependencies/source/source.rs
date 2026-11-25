pub mod gather_paths;
pub mod process_file;

pub use process_file::{Dependency, FileData, SourceLocation, SourcePosition};
use std::{
    collections::HashMap,
    io,
    path::{Path, PathBuf},
};

#[cfg(test)]
pub use process_file::{DependencyKind, SourceExcerpt};

use gather_paths::gather_paths;
use process_file::process_file;

#[derive(Debug)]
pub struct ProjectSourceData {
    pub files: HashMap<PathBuf, FileData>,
    pub folders: Vec<PathBuf>,
}

pub(super) fn build_project_source_data(
    root: &Path,
    exclusions: &[&str],
) -> io::Result<ProjectSourceData> {
    let paths = gather_paths(root, exclusions)?;

    let mut project_data = ProjectSourceData {
        files: HashMap::new(),
        folders: paths.folders,
    };

    for path in paths.files {
        match process_file(root, &path) {
            Ok(data) => {
                project_data.files.insert(path, data);
            }
            Err(error) if error.kind() == io::ErrorKind::InvalidData => {}
            Err(error) => {
                log::warn!("Failed to process {}: {error}", path.display());
            }
        }
    }

    Ok(project_data)
}
