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
