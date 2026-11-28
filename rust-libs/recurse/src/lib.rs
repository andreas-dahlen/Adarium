use rayon::prelude::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{fs, io};

const EXCLUDED_FOLDERS: &[&str] = &[
    ".git",
    ".vitest",
    ".vite",
    "node_modules",
    "target",
    "dist",
    "build",
    "coverage",
    ".next",
    ".nuxt",
];

#[derive(Default)]
pub struct Discovery {
    pub files: HashMap<String, Vec<PathBuf>>,
    pub folders: HashMap<String, Vec<PathBuf>>,
}

impl Discovery {
    fn merge(&mut self, other: Discovery) {
        for (name, paths) in other.files {
            self.files.entry(name).or_default().extend(paths);
        }

        for (name, paths) in other.folders {
            self.folders.entry(name).or_default().extend(paths);
        }
    }

    fn merge_parallel(discoveries: impl ParallelIterator<Item = Discovery>) -> Discovery {
        discoveries.reduce(Self::default, |mut left, right| {
            left.merge(right);
            left
        })
    }
}

fn load_node(current: &Path, root: &Path, file_names: &[&str], folder_names: &[&str]) -> Discovery {
    let mut discovery = Discovery::default();

    let Some(name) = current.file_name() else {
        return discovery;
    };

    if current.is_file() {
        if let Some(name) = name.to_str()
            && file_names.contains(&name)
            && let Ok(relative) = current.strip_prefix(root)
        {
            discovery
                .files
                .entry(name.to_string())
                .or_default()
                .push(relative.to_path_buf());
        }

        return discovery;
    }

    if !current.is_dir() {
        return discovery;
    }

    if EXCLUDED_FOLDERS.iter().any(|excluded| name == *excluded) {
        return discovery;
    }

    if let Some(name) = name.to_str()
        && folder_names.contains(&name)
        && let Ok(relative) = current.strip_prefix(root)
    {
        discovery
            .folders
            .entry(name.to_string())
            .or_default()
            .push(relative.to_path_buf());
    }

    let Ok(entries) = fs::read_dir(current) else {
        return discovery;
    };

    let paths: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();

    let children = paths
        .into_par_iter()
        .map(|path| load_node(&path, root, file_names, folder_names));

    discovery.merge(Discovery::merge_parallel(children));

    discovery
}

pub fn discover(root: &Path, file_names: &[&str], folder_names: &[&str]) -> io::Result<Discovery> {
    let entries = fs::read_dir(root)?;

    let paths: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();

    let discoveries = paths
        .into_par_iter()
        .map(|path| load_node(&path, root, file_names, folder_names));

    Ok(Discovery::merge_parallel(discoveries))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn discovers_files_and_folders() {
        let temp = tempdir().unwrap();
        let root = temp.path();

        fs::create_dir_all(root.join("src/components")).unwrap();
        fs::create_dir_all(root.join("tests")).unwrap();

        fs::write(root.join("Cargo.toml"), "").unwrap();
        fs::write(root.join("src/main.rs"), "").unwrap();
        fs::write(root.join("src/components/Button.vue"), "").unwrap();
        fs::write(root.join("tests/example.rs"), "").unwrap();

        let discovery = discover(
            root,
            &["Cargo.toml", "main.rs", "Button.vue"],
            &["src", "components", "tests"],
        )
        .unwrap();

        assert_eq!(
            discovery.files["Cargo.toml"],
            vec![PathBuf::from("Cargo.toml")]
        );

        assert_eq!(
            discovery.files["main.rs"],
            vec![PathBuf::from("src/main.rs")]
        );

        assert_eq!(
            discovery.files["Button.vue"],
            vec![PathBuf::from("src/components/Button.vue")]
        );

        assert_eq!(discovery.folders["src"], vec![PathBuf::from("src")]);

        assert_eq!(
            discovery.folders["components"],
            vec![PathBuf::from("src/components")]
        );

        assert_eq!(discovery.folders["tests"], vec![PathBuf::from("tests")]);
    }

    #[test]
    fn ignores_excluded_folders() {
        let temp = tempdir().unwrap();
        let root = temp.path();

        fs::create_dir_all(root.join("node_modules/package")).unwrap();
        fs::create_dir_all(root.join("src")).unwrap();

        fs::write(root.join("node_modules/package/package.json"), "").unwrap();
        fs::write(root.join("src/main.rs"), "").unwrap();

        let discovery = discover(
            root,
            &["package.json", "main.rs"],
            &["node_modules", "package", "src"],
        )
        .unwrap();

        assert!(!discovery.files.contains_key("package.json"));
        assert!(!discovery.folders.contains_key("node_modules"));
        assert!(!discovery.folders.contains_key("package"));

        assert_eq!(
            discovery.files["main.rs"],
            vec![PathBuf::from("src/main.rs")]
        );
    }
    #[test]
    fn collects_multiple_matches() {
        let temp = tempdir().unwrap();
        let root = temp.path();

        fs::create_dir_all(root.join("src/components")).unwrap();
        fs::create_dir_all(root.join("tests")).unwrap();

        fs::write(root.join("src/main.rs"), "").unwrap();
        fs::write(root.join("src/components/mod.rs"), "").unwrap();
        fs::write(root.join("tests/main.rs"), "").unwrap();

        let discovery = discover(root, &["main.rs", "mod.rs"], &["components"]).unwrap();

        assert_eq!(discovery.files["main.rs"].len(), 2);

        assert_eq!(
            discovery.files["mod.rs"],
            vec![PathBuf::from("src/components/mod.rs")]
        );

        assert_eq!(
            discovery.folders["components"],
            vec![PathBuf::from("src/components")]
        );
    }
    #[test]
    fn returns_error_when_root_does_not_exist() {
        let temp = tempdir().unwrap();
        let root = temp.path().join("missing");

        let result = discover(&root, &["Cargo.toml"], &["src"]);

        assert!(result.is_err());
    }
}
