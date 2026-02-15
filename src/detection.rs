use std::path::Path;

use crate::models::Language;

pub fn detect_language(path: &Path) -> Option<Language> {
    if let Some(lang) = detect_by_filename(path) {
        return Some(lang);
    }
    detect_by_extension(path)
}

fn detect_by_filename(path: &Path) -> Option<Language> {
    let filename = path.file_name()?.to_str()?;
    match filename {
        "Makefile" | "makefile" | "GNUmakefile" => Some(Language::Makefile),
        "Dockerfile" => Some(Language::Dockerfile),
        _ => None,
    }
}

fn detect_by_extension(path: &Path) -> Option<Language> {
    let ext = path.extension()?.to_str()?;
    match ext {
        "c" => Some(Language::C),
        "h" => Some(Language::CHeader),
        "cpp" | "cc" | "cxx" | "c++" => Some(Language::Cpp),
        "hpp" | "hxx" | "hh" => Some(Language::Cpp),
        "rs" => Some(Language::Rust),
        "py" | "pyi" => Some(Language::Python),
        "js" | "mjs" | "cjs" => Some(Language::JavaScript),
        "ts" | "mts" | "cts" => Some(Language::TypeScript),
        "html" | "htm" => Some(Language::Html),
        "css" | "scss" | "sass" => Some(Language::Css),
        "json" => Some(Language::Json),
        "yaml" | "yml" => Some(Language::Yaml),
        "toml" => Some(Language::Toml),
        "md" | "markdown" => Some(Language::Markdown),
        "sh" | "bash" | "zsh" | "fish" => Some(Language::Shell),
        "go" => Some(Language::Go),
        "java" => Some(Language::Java),
        "kt" | "kts" => Some(Language::Kotlin),
        "swift" => Some(Language::Swift),
        "php" => Some(Language::Php),
        "sql" => Some(Language::Sql),
        _ => Some(Language::Other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_rust() {
        assert_eq!(detect_language(Path::new("main.rs")), Some(Language::Rust));
    }

    #[test]
    fn test_detect_c_header() {
        assert_eq!(
            detect_language(Path::new("stdio.h")),
            Some(Language::CHeader)
        );
    }

    #[test]
    fn test_detect_cpp_variants() {
        for ext in &["cpp", "cc", "cxx", "c++", "hpp", "hxx"] {
            let path = format!("file.{ext}");
            assert_eq!(detect_language(Path::new(&path)), Some(Language::Cpp));
        }
    }

    #[test]
    fn test_detect_makefile() {
        assert_eq!(
            detect_language(Path::new("Makefile")),
            Some(Language::Makefile)
        );
        assert_eq!(
            detect_language(Path::new("makefile")),
            Some(Language::Makefile)
        );
        assert_eq!(
            detect_language(Path::new("GNUmakefile")),
            Some(Language::Makefile)
        );
    }

    #[test]
    fn test_detect_dockerfile() {
        assert_eq!(
            detect_language(Path::new("Dockerfile")),
            Some(Language::Dockerfile)
        );
    }

    #[test]
    fn test_detect_unknown_extension() {
        assert_eq!(
            detect_language(Path::new("data.xyz")),
            Some(Language::Other)
        );
    }

    #[test]
    fn test_no_extension_no_special_name() {
        assert_eq!(detect_language(Path::new("README")), None);
    }

    #[test]
    fn test_detect_typescript() {
        assert_eq!(
            detect_language(Path::new("app.ts")),
            Some(Language::TypeScript)
        );
        assert_eq!(
            detect_language(Path::new("app.mts")),
            Some(Language::TypeScript)
        );
    }

    #[test]
    fn test_detect_shell() {
        for ext in &["sh", "bash", "zsh", "fish"] {
            let path = format!("script.{ext}");
            assert_eq!(detect_language(Path::new(&path)), Some(Language::Shell));
        }
    }
}
