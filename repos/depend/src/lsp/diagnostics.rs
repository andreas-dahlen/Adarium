use tower_lsp_server::ls_types::{Diagnostic, DiagnosticSeverity, Position, Range};

use crate::{
    dependencies::{SourceLocation, SourcePosition},
    violations::{Violation, ViolationReason},
};

fn to_lsp_position(position: &SourcePosition) -> Position {
    Position {
        line: position.line - 1,
        character: position.column - 1,
    }
}

fn to_lsp_range(location: &SourceLocation) -> Range {
    Range {
        start: to_lsp_position(&location.start),
        end: to_lsp_position(&location.end),
    }
}

pub(super) fn violation_to_diagnostic(violation: &Violation) -> Diagnostic {
    let message = match violation.reason {
        ViolationReason::UndefinedBothNamespaces => {
            "Neither source nor dependency has a namespace defined."
        }
        ViolationReason::UndefinedSourceNamespace => "Source file has no namespace defined.",
        ViolationReason::UndefinedDependencyNamespace => "Dependency has no namespace defined.",
        ViolationReason::DisallowedImport => "Import violates namespace dependency rules.",
    };

    Diagnostic {
        range: to_lsp_range(&violation.metadata.location),
        severity: Some(DiagnosticSeverity::WARNING),
        source: Some("depend".to_string()),
        message: message.to_string(),
        ..Default::default()
    }
}
