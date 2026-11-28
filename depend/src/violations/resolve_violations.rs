use std::path::PathBuf;

use crate::{
    config::NamespaceRules,
    dependencies::Dependency,
    namespaces::{NamespaceDependencyEdge, NamespaceMembership},
};

#[derive(Debug)]
pub struct Violation {
    pub source_file: PathBuf,
    pub source: Option<NamespaceMembership>,
    pub dependency_file: PathBuf,
    pub dependency: Option<NamespaceMembership>,
    pub reason: ViolationReason,
    pub metadata: Dependency,
}

#[derive(Debug)]
pub enum ViolationReason {
    UndefinedBothNamespaces,
    UndefinedSourceNamespace,
    UndefinedDependencyNamespace,
    DisallowedImport,
}

pub(super) fn resolve_violations(
    rules: &[NamespaceRules],
    dependency_edges: &[NamespaceDependencyEdge],
) -> Vec<Violation> {
    let mut violations = Vec::new();

    for edge in dependency_edges {
        let reason = if edge.source_namespaces.is_none() && edge.dependency_namespaces.is_none() {
            Some(ViolationReason::UndefinedBothNamespaces)
        } else if edge.source_namespaces.is_none() {
            Some(ViolationReason::UndefinedSourceNamespace)
        } else if edge.dependency_namespaces.is_none() {
            Some(ViolationReason::UndefinedDependencyNamespace)
        } else {
            let is_allowed = edge.source_namespaces.iter().any(|source_namespace| {
                let found_rules = rules.iter().filter(|rule| {
                    rule.namespaces
                        .iter()
                        .any(|namespace| namespace == source_namespace)
                });

                edge.dependency_namespaces
                    .iter()
                    .any(|dependency_namespace| {
                        found_rules.clone().any(|rule| {
                            rule.allow_imports
                                .iter()
                                .any(|allowed| allowed == dependency_namespace)
                        })
                    })
            });

            if is_allowed {
                None
            } else {
                Some(ViolationReason::DisallowedImport)
            }
        };

        if let Some(reason) = reason {
            violations.push(Violation {
                source_file: edge.source.clone(),
                source: edge.source_namespaces.as_option(),
                dependency_file: edge.dependency.clone(),
                dependency: edge.dependency_namespaces.as_option(),
                reason,
                metadata: edge.dependency_metadata.clone(),
            });
        }
    }

    violations
}
