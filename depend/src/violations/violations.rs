use crate::{
    config::NamespaceRules, namespaces::NamespaceDependencyEdge,
    violations::resolve_violations::resolve_violations,
};

mod violation_description;

pub use violation_description::{ViolationDescription, describe_violation};

pub use resolve_violations::Violation;

pub use resolve_violations::ViolationReason;

mod resolve_violations;

pub(super) fn find_violations(
    rules: &[NamespaceRules],
    dependency_edges: &[NamespaceDependencyEdge],
) -> Vec<Violation> {
    let violations = resolve_violations(rules, dependency_edges);

    log::debug!("Violations: {:?}", violations);

    violations
}
