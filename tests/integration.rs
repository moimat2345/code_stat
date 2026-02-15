use std::path::Path;

use code_stats::analyzer;
use code_stats::models::Language;
use code_stats::stats;

fn fixture_path() -> &'static Path {
    Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/sample_repo"
    ))
}

#[test]
fn test_full_pipeline() {
    let scan = analyzer::analyze(fixture_path());
    let report = stats::aggregate(&scan);

    // We have 4 text files + 1 empty JS file = 5 analyzed, 0 skipped
    assert_eq!(report.global.analyzed_count, 5);
    assert_eq!(report.global.skipped_count, 0);

    // Check that we detected the right languages
    let languages: Vec<Language> = report.languages.iter().map(|l| l.language).collect();
    assert!(languages.contains(&Language::Rust));
    assert!(languages.contains(&Language::Python));
    assert!(languages.contains(&Language::Markdown));
    assert!(languages.contains(&Language::Makefile));
    assert!(languages.contains(&Language::JavaScript));
}

#[test]
fn test_line_counts() {
    let scan = analyzer::analyze(fixture_path());
    let report = stats::aggregate(&scan);

    let rust = report
        .languages
        .iter()
        .find(|l| l.language == Language::Rust)
        .expect("should find Rust");
    assert_eq!(rust.line_count, 10);
    assert_eq!(rust.file_count, 1);

    let python = report
        .languages
        .iter()
        .find(|l| l.language == Language::Python)
        .expect("should find Python");
    assert_eq!(python.line_count, 15);
    assert_eq!(python.file_count, 1);

    let js = report
        .languages
        .iter()
        .find(|l| l.language == Language::JavaScript)
        .expect("should find JavaScript");
    assert_eq!(js.line_count, 0);
    assert_eq!(js.file_count, 1);
}

#[test]
fn test_largest_file() {
    let scan = analyzer::analyze(fixture_path());
    let report = stats::aggregate(&scan);

    let (_, largest_lines) = report
        .global
        .largest_file
        .as_ref()
        .expect("should have largest");
    assert_eq!(*largest_lines, 15); // script.py has the most lines
}
