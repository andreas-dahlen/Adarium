use crate::{cli::print_violations::print_violation, violations::Violation};
mod print_violations;

pub fn handle_cli(violations: &[Violation]) -> Result<bool, Box<dyn std::error::Error>> {
    for violation in violations {
        print_violation(violation);
    }

    if !violations.is_empty() {
        println!("\nFound {} violations.", violations.len());
    };

    Ok(!violations.is_empty())
}
