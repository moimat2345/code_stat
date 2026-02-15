use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::Path;

use crate::detection::detect_language;
use crate::models::{FileInfo, ScanResult, SkipReason, SkippedFile};
use crate::scanner;

const BINARY_CHECK_SIZE: usize = 8192;

pub fn analyze(root: &Path) -> ScanResult {
    let mut analyzed = Vec::new();
    let mut skipped = Vec::new();

    for entry in scanner::scan(root) {
        let entry = match entry {
            Ok(e) => e,
            Err(err) => {
                eprintln!("warning: {err}");
                continue;
            }
        };

        let abs_path = entry.path();

        let file_type = match entry.file_type() {
            Some(ft) => ft,
            None => continue,
        };
        if !file_type.is_file() {
            continue;
        }

        let language = match detect_language(abs_path) {
            Some(lang) => lang,
            None => continue,
        };

        let rel_path = abs_path
            .strip_prefix(root)
            .unwrap_or(abs_path)
            .to_path_buf();

        match analyze_file(abs_path) {
            Ok(Some((line_count, byte_size))) => {
                analyzed.push(FileInfo {
                    path: rel_path,
                    language,
                    line_count,
                    byte_size,
                });
            }
            Ok(None) => {
                skipped.push(SkippedFile {
                    path: rel_path,
                    reason: SkipReason::Binary,
                });
            }
            Err(err) => {
                skipped.push(SkippedFile {
                    path: rel_path,
                    reason: SkipReason::ReadError(err.to_string()),
                });
            }
        }
    }

    ScanResult { analyzed, skipped }
}

/// Analyzes a single file. Returns Ok(Some((lines, bytes))) for text files,
/// Ok(None) for binary files, or Err for read errors.
fn analyze_file(path: &Path) -> std::io::Result<Option<(usize, u64)>> {
    let metadata = fs::metadata(path)?;
    let byte_size = metadata.len();

    if byte_size == 0 {
        return Ok(Some((0, 0)));
    }

    if is_binary(path)? {
        return Ok(None);
    }

    let line_count = count_lines(path)?;
    Ok(Some((line_count, byte_size)))
}

fn is_binary(path: &Path) -> std::io::Result<bool> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; BINARY_CHECK_SIZE];
    let bytes_read = reader.read(&mut buffer)?;
    Ok(buffer[..bytes_read].contains(&0))
}

fn count_lines(path: &Path) -> std::io::Result<usize> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; 8192];
    let mut count = 0usize;
    let mut last_byte = 0u8;

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        for &byte in &buffer[..bytes_read] {
            if byte == b'\n' {
                count += 1;
            }
            last_byte = byte;
        }
    }

    // If the file doesn't end with a newline, count the last line
    if last_byte != b'\n' && last_byte != 0 {
        count += 1;
    }

    Ok(count)
}
