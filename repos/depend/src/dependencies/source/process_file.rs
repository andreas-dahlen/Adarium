use oxc_allocator::Allocator;
use oxc_ast::ast::Statement;
use oxc_parser::Parser;
use oxc_span::SourceType;
use std::{io, path::Path};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FileData {
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependency {
    pub source: String,
    pub kind: DependencyKind,
    pub location: SourceLocation,
    pub excerpt: SourceExcerpt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceExcerpt {
    pub before: Option<String>,
    pub line: String,
    pub after: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    pub start: SourcePosition,
    pub end: SourcePosition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourcePosition {
    pub offset: u32,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencyKind {
    Import,
    ReExport,
    ExportAll,
}

pub fn process_file(root: &Path, relative_path: &Path) -> io::Result<FileData> {
    let absolute_path = root.join(relative_path);
    let source = read_file(&absolute_path)?;

    Ok(process_source(&source, relative_path))
}

pub fn process_source(source: &str, relative_path: &Path) -> FileData {
    let allocator = Allocator::default();
    let Ok(source_type) = SourceType::from_path(relative_path) else {
        return FileData::default();
    };

    let parser_return = Parser::new(&allocator, source, source_type).parse();

    let mut dependencies = Vec::new();

    for statement in &parser_return.program.body {
        match statement {
            Statement::ImportDeclaration(import) => {
                if let Some(dependency_source) = import.source.value.as_str() {
                    let location = get_source_location(source, import.source.span);
                    let excerpt = get_source_excerpt(source, location.start.line);
                    dependencies.push(Dependency {
                        source: dependency_source.to_owned(),
                        kind: DependencyKind::Import,
                        location,
                        excerpt,
                    });
                }
            }
            Statement::ExportFromDeclaration(export) => {
                if let Some(dependency_source) = export.source.value.as_str() {
                    let location = get_source_location(source, export.source.span);
                    let excerpt = get_source_excerpt(source, location.start.line);
                    dependencies.push(Dependency {
                        source: dependency_source.to_owned(),
                        kind: DependencyKind::ReExport,
                        location,
                        excerpt,
                    });
                }
            }
            Statement::ExportAllDeclaration(export) => {
                if let Some(dependency_source) = export.source.value.as_str() {
                    let location = get_source_location(source, export.source.span);
                    let excerpt = get_source_excerpt(source, location.start.line);
                    dependencies.push(Dependency {
                        source: dependency_source.to_owned(),
                        kind: DependencyKind::ExportAll,
                        location,
                        excerpt,
                    });
                }
            }
            _ => {}
        }
    }

    FileData { dependencies }
}

fn read_file(path: &Path) -> io::Result<String> {
    std::fs::read_to_string(path)
}

fn get_source_location(source: &str, span: oxc_span::Span) -> SourceLocation {
    SourceLocation {
        start: get_source_position(source, span.start),
        end: get_source_position(source, span.end),
    }
}

fn get_source_excerpt(source: &str, line: u32) -> SourceExcerpt {
    let lines: Vec<&str> = source.lines().collect();
    let index = (line - 1) as usize;

    SourceExcerpt {
        before: index
            .checked_sub(1)
            .and_then(|i| lines.get(i).map(|line| line.to_string())),
        line: lines.get(index).unwrap_or(&"").to_string(),
        after: lines.get(index + 1).map(|line| line.to_string()),
    }
}

fn get_source_position(source: &str, offset: u32) -> SourcePosition {
    let offset = offset as usize;
    let prefix = &source[..offset];

    let line = prefix.matches('\n').count() as u32 + 1;

    let column = prefix
        .rsplit('\n')
        .next()
        .unwrap_or_default()
        .encode_utf16()
        .count() as u32
        + 1;

    SourcePosition {
        offset: offset as u32,
        line,
        column,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn process_source(source: &str, filename: &str) -> io::Result<FileData> {
        let temp = tempfile::tempdir()?;
        let root = temp.path();

        std::fs::write(root.join(filename), source)?;

        process_file(root, Path::new(filename))
    }

    fn dependency_summary(file_data: &FileData) -> Vec<(&str, DependencyKind)> {
        file_data
            .dependencies
            .iter()
            .map(|dependency| (dependency.source.as_str(), dependency.kind))
            .collect()
    }

    #[test]
    fn extracts_static_imports() {
        let file_data = process_source(
            r#"
                import { Button } from "../../components/Button";
                import { formatDate } from "../../utils/date";
            "#,
            "example.ts",
        )
        .unwrap();

        assert_eq!(
            dependency_summary(&file_data),
            vec![
                ("../../components/Button", DependencyKind::Import),
                ("../../utils/date", DependencyKind::Import),
            ]
        );
    }

    #[test]
    fn extracts_named_re_exports() {
        let file_data =
            process_source(r#"export { Button } from "./Button";"#, "example.ts").unwrap();

        assert_eq!(
            dependency_summary(&file_data),
            vec![("./Button", DependencyKind::ReExport)]
        );
    }

    #[test]
    fn extracts_wildcard_re_exports() {
        let file_data = process_source(r#"export * from "./utils";"#, "example.ts").unwrap();

        assert_eq!(
            dependency_summary(&file_data),
            vec![("./utils", DependencyKind::ExportAll)]
        );
    }

    #[test]
    fn ignores_unsupported_file_types() {
        let file_data = process_source("# Hello", "README.md").unwrap();

        assert_eq!(file_data, FileData::default());
    }

    #[test]
    fn handles_files_without_dependencies() {
        let file_data = process_source(
            r#"
                const greeting = "Hello";
                export const answer = 42;
            "#,
            "example.ts",
        )
        .unwrap();

        assert_eq!(file_data, FileData::default());
    }

    #[test]
    fn extracts_dependency_source_location() {
        let source = "const label = \"😀\";\nimport { Button } from \"./Button\";";

        let file_data = process_source(source, "example.ts").unwrap();

        let dependency = &file_data.dependencies[0];

        let start = source.find("\"./Button\"").unwrap();
        let end = start + "\"./Button\"".len();

        let line_start = source.rfind('\n').unwrap() + 1;

        let start_column = source[line_start..start].encode_utf16().count() as u32 + 1;

        let end_column = source[line_start..end].encode_utf16().count() as u32 + 1;

        assert_eq!(
            dependency.location,
            SourceLocation {
                start: SourcePosition {
                    offset: start as u32,
                    line: 2,
                    column: start_column,
                },
                end: SourcePosition {
                    offset: end as u32,
                    line: 2,
                    column: end_column,
                },
            }
        );
    }

    #[test]
    fn returns_error_when_file_does_not_exist() {
        let temp = tempfile::tempdir().unwrap();

        let result = process_file(temp.path(), Path::new("missing.ts"));

        assert!(result.is_err());
    }
}
