use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct ProjectPaths {
    pub files: Vec<PathBuf>,
    pub folders: Vec<PathBuf>,
}

//add traversal stop integration and exclusions... add file ending analysis to sort relevancy...
pub(super) fn gather_paths(root: &Path, exclusions: &[&str]) -> io::Result<ProjectPaths> {
    let entries = fs::read_dir(root)?;

    let mut paths = ProjectPaths {
        files: Vec::new(),
        folders: Vec::new(),
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                log::warn!(
                    "Failed to read directory entry in {}: {error}",
                    root.display()
                );
                continue;
            }
        };
        load_node(&entry.path(), root, &mut paths, exclusions);
    }

    fn load_node(current: &Path, root: &Path, paths: &mut ProjectPaths, exclusions: &[&str]) {
        if current.file_name().is_some_and(|name| {
            exclusions
                .iter()
                .any(|pattern| name.to_str() == Some(pattern))
        }) {
            return;
        }

        if current.is_file() {
            if let Ok(relative) = current.strip_prefix(root) {
                paths.files.push(relative.to_path_buf());
            }
            return;
        }

        if !current.is_dir() {
            return;
        }

        if let Ok(relative) = current.strip_prefix(root) {
            paths.folders.push(relative.to_path_buf());
        }

        let entries = match fs::read_dir(current) {
            Ok(entries) => entries,
            Err(error) => {
                log::warn!(
                    "Failed to read directory entry in {}: {error}",
                    current.display()
                );
                return;
            }
        };

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(error) => {
                    log::warn!(
                        "Failed to read directory entry in {}: {error}",
                        current.display()
                    );
                    continue;
                }
            };

            load_node(&entry.path(), root, paths, exclusions)
        }
    }

    Ok(paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gather_sorted(root: &Path) -> io::Result<ProjectPaths> {
        let mut paths = gather_paths(root, &[])?;

        paths.files.sort();
        paths.folders.sort();

        Ok(paths)
    }

    #[test]
    fn gathers_files_and_folders() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();

        fs::create_dir_all(root.join("src/components")).unwrap();
        fs::create_dir_all(root.join("utils")).unwrap();

        fs::write(root.join("src/main.rs"), "").unwrap();
        fs::write(root.join("src/components/Button.tsx"), "").unwrap();
        fs::write(root.join("utils/date.ts"), "").unwrap();

        let paths = gather_sorted(root).unwrap();

        assert_eq!(
            paths.files,
            vec![
                PathBuf::from("src/components/Button.tsx"),
                PathBuf::from("src/main.rs"),
                PathBuf::from("utils/date.ts"),
            ]
        );

        assert_eq!(
            paths.folders,
            vec![
                PathBuf::from("src"),
                PathBuf::from("src/components"),
                PathBuf::from("utils"),
            ]
        );
    }

    #[test]
    fn gathers_empty_directory() {
        let temp = tempfile::tempdir().unwrap();

        let paths = gather_sorted(temp.path()).unwrap();

        assert!(paths.files.is_empty());
        assert!(paths.folders.is_empty());
    }

    #[test]
    fn excludes_matching_files_and_folders() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();

        fs::create_dir_all(root.join("src/target")).unwrap();
        fs::write(root.join("src/main.rs"), "").unwrap();
        fs::write(root.join("src/target/generated.rs"), "").unwrap();
        fs::write(root.join("src/target.txt"), "").unwrap();

        let exclusions = vec!["target"];
        let paths = gather_paths(root, &exclusions).unwrap();

        assert_eq!(
            paths.files,
            vec![
                PathBuf::from("src/target.txt"),
                PathBuf::from("src/main.rs"),
            ]
        );
        assert_eq!(paths.folders, vec![PathBuf::from("src")]);
    }
}
