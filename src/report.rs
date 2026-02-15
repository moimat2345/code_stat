use comfy_table::{Attribute, Cell, CellAlignment, ContentArrangement, Table};

use crate::models::Report;

pub fn render(report: &Report) {
    if report.languages.is_empty() {
        println!("No source files found.");
        return;
    }

    println!();
    println!("  {} (by lines):", bold("Language breakdown"));
    println!();

    let mut table = Table::new();
    table
        .set_content_arrangement(ContentArrangement::Dynamic)
        .load_preset(comfy_table::presets::NOTHING);

    table.set_header(vec![
        Cell::new("Language").add_attribute(Attribute::Bold),
        Cell::new("Lines").add_attribute(Attribute::Bold),
        Cell::new("%").add_attribute(Attribute::Bold),
        Cell::new("Files").add_attribute(Attribute::Bold),
        Cell::new("Largest file").add_attribute(Attribute::Bold),
    ]);

    for lang in &report.languages {
        let file_display = format!(
            "{} ({})",
            lang.largest_file.display(),
            format_number(lang.largest_file_lines),
        );

        table.add_row(vec![
            Cell::new(lang.language.display_name()),
            Cell::new(format_number(lang.line_count)).set_alignment(CellAlignment::Right),
            Cell::new(format!("{:.1}%", lang.percentage)).set_alignment(CellAlignment::Right),
            Cell::new(lang.file_count).set_alignment(CellAlignment::Right),
            Cell::new(file_display),
        ]);
    }

    println!("{table}");
    println!();

    // Global stats
    if let Some((path, lines)) = &report.global.largest_file {
        println!(
            "  {} {} — {} lines",
            bold("Largest file:"),
            path.display(),
            format_number(*lines),
        );
    }

    println!(
        "  {} {} analyzed, {} skipped ({} binary)",
        bold("Files:"),
        format_number(report.global.analyzed_count),
        format_number(report.global.skipped_count),
        format_number(report.global.binary_count),
    );
    println!(
        "  {} {}",
        bold("Total lines:"),
        format_number(report.global.total_lines),
    );
    println!();
}

fn bold(s: &str) -> String {
    format!("\x1b[1m{s}\x1b[0m")
}

fn format_number(n: usize) -> String {
    let s = n.to_string();
    let bytes = s.as_bytes();
    let mut result = String::with_capacity(s.len() + s.len() / 3);
    for (i, &b) in bytes.iter().enumerate() {
        if i > 0 && (bytes.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(b as char);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(0), "0");
        assert_eq!(format_number(42), "42");
        assert_eq!(format_number(999), "999");
        assert_eq!(format_number(1000), "1,000");
        assert_eq!(format_number(12340), "12,340");
        assert_eq!(format_number(1000000), "1,000,000");
    }
}
