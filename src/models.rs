use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Language {
    C,
    CHeader,
    Cpp,
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Html,
    Css,
    Json,
    Yaml,
    Toml,
    Markdown,
    Shell,
    Go,
    Java,
    Kotlin,
    Swift,
    Php,
    Sql,
    Dockerfile,
    Makefile,
    Other,
}

impl Language {
    pub fn display_name(self) -> &'static str {
        match self {
            Self::C => "C",
            Self::CHeader => "C Header",
            Self::Cpp => "C++",
            Self::Rust => "Rust",
            Self::Python => "Python",
            Self::JavaScript => "JavaScript",
            Self::TypeScript => "TypeScript",
            Self::Html => "HTML",
            Self::Css => "CSS",
            Self::Json => "JSON",
            Self::Yaml => "YAML",
            Self::Toml => "TOML",
            Self::Markdown => "Markdown",
            Self::Shell => "Shell",
            Self::Go => "Go",
            Self::Java => "Java",
            Self::Kotlin => "Kotlin",
            Self::Swift => "Swift",
            Self::Php => "PHP",
            Self::Sql => "SQL",
            Self::Dockerfile => "Dockerfile",
            Self::Makefile => "Makefile",
            Self::Other => "Other",
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub language: Language,
    pub line_count: usize,
    pub byte_size: u64,
}

#[derive(Debug, Clone)]
pub enum SkipReason {
    Binary,
    ReadError(String),
}

#[derive(Debug, Clone)]
pub struct SkippedFile {
    pub path: PathBuf,
    pub reason: SkipReason,
}

#[derive(Debug)]
pub struct ScanResult {
    pub analyzed: Vec<FileInfo>,
    pub skipped: Vec<SkippedFile>,
}

#[derive(Debug, Clone)]
pub struct LanguageStats {
    pub language: Language,
    pub file_count: usize,
    pub line_count: usize,
    pub percentage: f64,
    pub largest_file: PathBuf,
    pub largest_file_lines: usize,
}

#[derive(Debug, Clone)]
pub struct GlobalStats {
    pub total_files: usize,
    pub total_lines: usize,
    pub analyzed_count: usize,
    pub skipped_count: usize,
    pub binary_count: usize,
    pub largest_file: Option<(PathBuf, usize)>,
}

#[derive(Debug)]
pub struct Report {
    pub global: GlobalStats,
    pub languages: Vec<LanguageStats>,
}
