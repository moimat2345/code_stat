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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn write_temp_file(content: &[u8]) -> NamedTempFile {
        let mut tmp = NamedTempFile::new().expect("failed to create temp file");
        tmp.write_all(content).expect("failed to write temp file");
        tmp.flush().expect("failed to flush temp file");
        tmp
    }

    #[test]
    fn test_count_lines_trailing_newline() {
        let tmp = write_temp_file(b"line1\nline2\nline3\n");
        assert_eq!(count_lines(tmp.path()).unwrap(), 3);
    }

    #[test]
    fn test_count_lines_no_trailing_newline() {
        let tmp = write_temp_file(b"line1\nline2\nline3");
        assert_eq!(count_lines(tmp.path()).unwrap(), 3);
    }

    #[test]
    fn test_count_lines_single_line_no_newline() {
        let tmp = write_temp_file(b"hello");
        assert_eq!(count_lines(tmp.path()).unwrap(), 1);
    }

    #[test]
    fn test_count_lines_crlf() {
        let tmp = write_temp_file(b"line1\r\nline2\r\n");
        assert_eq!(count_lines(tmp.path()).unwrap(), 2);
    }

    #[test]
    fn test_is_binary_text() {
        let tmp = write_temp_file(b"just some text\nwith newlines\n");
        assert!(!is_binary(tmp.path()).unwrap());
    }

    #[test]
    fn test_is_binary_with_null() {
        let tmp = write_temp_file(b"some\x00binary\x00data");
        assert!(is_binary(tmp.path()).unwrap());
    }

    #[test]
    fn test_empty_file_not_binary() {
        let tmp = write_temp_file(b"");
        // Empty file - analyze_file returns Ok(Some((0, 0))) without calling is_binary
        let result = analyze_file(tmp.path()).unwrap();
        assert_eq!(result, Some((0, 0)));
    }
}
