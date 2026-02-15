use std::collections::HashMap;

use crate::models::{FileInfo, GlobalStats, LanguageStats, Report, ScanResult, SkipReason};

pub fn aggregate(scan: &ScanResult) -> Report {
    let global = compute_global_stats(scan);
    let languages = compute_language_stats(&scan.analyzed, global.total_lines);

    Report { global, languages }
}

fn compute_global_stats(scan: &ScanResult) -> GlobalStats {
    let analyzed_count = scan.analyzed.len();
    let skipped_count = scan.skipped.len();
    let total_files = analyzed_count + skipped_count;
    let total_lines: usize = scan.analyzed.iter().map(|f| f.line_count).sum();

    let largest_file = scan
        .analyzed
        .iter()
        .max_by_key(|f| f.line_count)
        .map(|f| (f.path.clone(), f.line_count));

    let binary_count = scan
        .skipped
        .iter()
        .filter(|s| matches!(s.reason, SkipReason::Binary))
        .count();

    GlobalStats {
        total_files,
        total_lines,
        analyzed_count,
        skipped_count,
        largest_file,
        binary_count,
    }
}

fn compute_language_stats(files: &[FileInfo], total_lines: usize) -> Vec<LanguageStats> {
    let mut by_language: HashMap<_, Vec<&FileInfo>> = HashMap::new();
    for file in files {
        by_language.entry(file.language).or_default().push(file);
    }

    let mut stats: Vec<LanguageStats> = by_language
        .into_iter()
        .map(|(language, files)| {
            let file_count = files.len();
            let line_count: usize = files.iter().map(|f| f.line_count).sum();
            let percentage = if total_lines > 0 {
                (line_count as f64 / total_lines as f64) * 100.0
            } else {
                0.0
            };
            let largest = files
                .iter()
                .max_by_key(|f| f.line_count)
                .expect("non-empty group");

            LanguageStats {
                language,
                file_count,
                line_count,
                percentage,
                largest_file: largest.path.clone(),
                largest_file_lines: largest.line_count,
            }
        })
        .collect();

    stats.sort_by(|a, b| b.line_count.cmp(&a.line_count));
    stats
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Language, SkippedFile};
    use std::path::PathBuf;

    fn make_file(path: &str, language: Language, lines: usize) -> FileInfo {
        FileInfo {
            path: PathBuf::from(path),
            language,
            line_count: lines,
            byte_size: lines as u64 * 40,
        }
    }

    #[test]
    fn test_aggregate_basic() {
        let scan = ScanResult {
            analyzed: vec![
                make_file("src/main.rs", Language::Rust, 100),
                make_file("src/lib.rs", Language::Rust, 50),
                make_file("README.md", Language::Markdown, 30),
            ],
            skipped: vec![],
        };

        let report = aggregate(&scan);

        assert_eq!(report.global.total_files, 3);
        assert_eq!(report.global.total_lines, 180);
        assert_eq!(report.global.analyzed_count, 3);
        assert_eq!(report.global.skipped_count, 0);

        // Languages should be sorted by line count descending
        assert_eq!(report.languages.len(), 2);
        assert_eq!(report.languages[0].language, Language::Rust);
        assert_eq!(report.languages[0].line_count, 150);
        assert_eq!(report.languages[0].file_count, 2);
        assert_eq!(report.languages[1].language, Language::Markdown);
        assert_eq!(report.languages[1].line_count, 30);
    }

    #[test]
    fn test_aggregate_percentages() {
        let scan = ScanResult {
            analyzed: vec![
                make_file("a.rs", Language::Rust, 75),
                make_file("b.py", Language::Python, 25),
            ],
            skipped: vec![],
        };

        let report = aggregate(&scan);

        assert!((report.languages[0].percentage - 75.0).abs() < 0.01);
        assert!((report.languages[1].percentage - 25.0).abs() < 0.01);
    }

    #[test]
    fn test_aggregate_with_skipped() {
        let scan = ScanResult {
            analyzed: vec![make_file("a.rs", Language::Rust, 10)],
            skipped: vec![
                SkippedFile {
                    path: PathBuf::from("image.png"),
                    reason: SkipReason::Binary,
                },
                SkippedFile {
                    path: PathBuf::from("locked.txt"),
                    reason: SkipReason::ReadError("permission denied".into()),
                },
            ],
        };

        let report = aggregate(&scan);

        assert_eq!(report.global.total_files, 3);
        assert_eq!(report.global.analyzed_count, 1);
        assert_eq!(report.global.skipped_count, 2);
    }

    #[test]
    fn test_aggregate_empty() {
        let scan = ScanResult {
            analyzed: vec![],
            skipped: vec![],
        };

        let report = aggregate(&scan);

        assert_eq!(report.global.total_files, 0);
        assert_eq!(report.global.total_lines, 0);
        assert!(report.languages.is_empty());
        assert!(report.global.largest_file.is_none());
    }

    #[test]
    fn test_largest_file_tracking() {
        let scan = ScanResult {
            analyzed: vec![
                make_file("small.rs", Language::Rust, 10),
                make_file("big.rs", Language::Rust, 500),
                make_file("medium.rs", Language::Rust, 100),
            ],
            skipped: vec![],
        };

        let report = aggregate(&scan);

        let (path, lines) = report
            .global
            .largest_file
            .as_ref()
            .expect("should have largest");
        assert_eq!(path, &PathBuf::from("big.rs"));
        assert_eq!(*lines, 500);

        assert_eq!(report.languages[0].largest_file, PathBuf::from("big.rs"));
        assert_eq!(report.languages[0].largest_file_lines, 500);
    }
}
