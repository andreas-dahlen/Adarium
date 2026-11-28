use std::{collections::HashMap, path::PathBuf};

use globset::{Glob, GlobSet, GlobSetBuilder};

use crate::{config::NamespacePattern, dependencies::DependencyGraph};

#[derive(Debug)]
pub struct NamespaceMemberships {
    pub files: HashMap<PathBuf, NamespaceMembership>,
    pub folders: HashMap<PathBuf, String>,
}

#[derive(Debug, Clone)]
pub struct NamespaceMembership {
    file: Option<String>,
    folder: Option<String>,
}

impl Default for NamespaceMembership {
    fn default() -> Self {
        Self {
            file: None,
            folder: None,
        }
    }
}

impl NamespaceMembership {
    pub fn is_none(&self) -> bool {
        self.file.is_none() && self.folder.is_none()
    }
    pub fn as_option(&self) -> Option<Self> {
        if self.is_none() {
            None
        } else {
            Some(self.clone())
        }
    }

    pub fn file(&self) -> Option<&str> {
        self.file.as_deref()
    }

    pub fn folder(&self) -> Option<&str> {
        self.folder.as_deref()
    }

    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.file
            .iter()
            .chain(self.folder.iter())
            .map(String::as_str)
    }
}

pub(super) fn resolve_memberships(
    folders: &[NamespacePattern],
    files: &[NamespacePattern],
    file_paths: &[PathBuf],
    folder_paths: &[PathBuf],
) -> Result<NamespaceMemberships, globset::Error> {
    let mut prelim_folders = Vec::new();
    let mut prelim_files = Vec::new();

    for declaration in folders {
        let glob_set = build_glob_set(&declaration.patterns)?;

        prelim_folders.push((declaration.namespace.clone(), glob_set));
    }
    for declaration in files {
        let glob_set = build_glob_set(&declaration.patterns)?;

        prelim_files.push((declaration.namespace.clone(), glob_set));
    }

    let resolved_namespaces = NamespaceMemberships {
        folders: resolve_folder_priority(folder_paths, &prelim_folders),
        files: resolve_file_priority(file_paths, &prelim_files, &prelim_folders),
    };
    Ok(resolved_namespaces)
}

fn resolve_file_priority(
    file_paths: &[PathBuf],
    prelim_files: &[(String, GlobSet)],
    prelim_folders: &[(String, GlobSet)],
) -> HashMap<PathBuf, NamespaceMembership> {
    let mut resolved = HashMap::new();

    for path in file_paths {
        let mut file_ownership = NamespaceMembership {
            file: None,
            folder: None,
        };
        for (namespace, glob_set) in prelim_files {
            if glob_set.is_match(path) {
                file_ownership.file = Some(namespace.clone());
            }
        }
        for (namespace, glob_set) in prelim_folders {
            if glob_set.is_match(path) {
                file_ownership.folder = Some(namespace.clone());
            }
        }
        if file_ownership.file.is_none() && file_ownership.folder.is_none() {
            continue;
        }
        resolved.insert(path.clone(), file_ownership);
    }

    resolved
}
fn resolve_folder_priority(
    folder_paths: &[PathBuf],
    prelim_folders: &[(String, GlobSet)],
) -> HashMap<PathBuf, String> {
    let mut resolved = HashMap::new();

    for (namespace, glob_set) in prelim_folders {
        for path in folder_paths {
            if glob_set.is_match(path) {
                resolved.insert(path.clone(), namespace.clone());
            }
        }
    }
    resolved
}

fn build_glob_set(patterns: &[String]) -> Result<GlobSet, globset::Error> {
    let mut builder = GlobSetBuilder::new();

    for pattern in patterns {
        builder.add(Glob::new(pattern)?);
    }

    builder.build()
}
