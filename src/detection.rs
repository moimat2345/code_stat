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
