use crate::violations::{Violation, describe_violation};

pub(super) fn print_violation(violation: &Violation) {
    let description = describe_violation(violation);

    let location = &violation.metadata.location;
    let excerpt = &violation.metadata.excerpt;

    let line = location.start.line;
    let column = location.start.column;
    let underline_length = location.end.column.saturating_sub(column).max(1);

    println!("  ⚠ depends({}): {}", description.code, description.message);

    println!(
        "     ╭─[{}:{}:{}]",
        violation.source_file.display(),
        line,
        column
    );

    if let Some(before) = &excerpt.before {
        println!("{:>4} │ {}", line - 1, before);
    }

    println!("{:>4} │ {}", line, excerpt.line);
    println!(
        "     · {}{}",
        " ".repeat(column.saturating_sub(1) as usize),
        "─".repeat(underline_length as usize)
    );

    if let Some(after) = &excerpt.after {
        println!("{:>4} │ {}", line + 1, after);
    }

    println!("     ╰────");
}
