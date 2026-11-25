use crate::{
    namespaces::NamespaceMembership,
    violations::{Violation, ViolationReason},
};

pub struct ViolationDescription {
    pub code: &'static str,
    pub message: String,
}

pub fn describe_violation(violation: &Violation) -> ViolationDescription {
    let (code, message) = match &violation.reason {
        ViolationReason::UndefinedBothNamespaces => (
            "undefined-both-namespaces",
            "Neither the source nor the dependency belongs to a namespace.".to_string(),
        ),
        ViolationReason::UndefinedSourceNamespace => {
            let dependency = violation
                .dependency
                .as_ref()
                .map(format_memberships)
                .unwrap_or_else(|| "unassigned dependency".to_string());

            (
                "undefined-source-namespace",
                format!("Source has no namespace; dependency belongs to {dependency}."),
            )
        }
        ViolationReason::UndefinedDependencyNamespace => {
            let source = violation
                .source
                .as_ref()
                .map(format_memberships)
                .unwrap_or_else(|| "unassigned source".to_string());

            (
                "undefined-dependency-namespace",
                format!("Dependency has no namespace; source belongs to {source}."),
            )
        }
        ViolationReason::DisallowedImport => {
            let source = violation
                .source
                .as_ref()
                .map(format_memberships)
                .unwrap_or_else(|| "unassigned source".to_string());

            let dependency = violation
                .dependency
                .as_ref()
                .map(format_memberships)
                .unwrap_or_else(|| "unassigned dependency".to_string());

            (
                "disallowed-import",
                format!("Source {source} cannot import dependency {dependency}."),
            )
        }
    };

    ViolationDescription { code, message }
}

fn format_memberships(membership: &NamespaceMembership) -> String {
    let mut namespaces = Vec::new();

    if let Some(namespace) = membership.file() {
        namespaces.push(format!("[file: \"{namespace}\"]"));
    }

    if let Some(namespace) = membership.folder() {
        namespaces.push(format!("[folder: \"{namespace}\"]"));
    }

    namespaces.join(" or ")
}
