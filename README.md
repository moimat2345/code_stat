<div align="center">

# code-stats

**Know your codebase at a glance — entirely offline.**

A fast CLI tool that recursively scans a directory, detects programming languages,
and displays a clean breakdown of file counts, line counts, and largest files. Like GitHub Linguist, but local and instant.

![Rust](https://img.shields.io/badge/Rust-stable-000000?logo=rust&logoColor=white)
![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Linux%20%7C%20Windows-5C6BC0)
![License](https://img.shields.io/badge/License-MIT-green)
![Dependencies](https://img.shields.io/badge/Dependencies-4-7E57C2)

<a href="README.fr.md"><img src="https://img.shields.io/badge/%F0%9F%87%AB%F0%9F%87%B7_Lire_en_fran%C3%A7ais-blue?style=for-the-badge" alt="Lire en francais"></a>

</div>

---

## What It Does

code-stats scans a source code repository and produces a **language breakdown by lines**, showing for each detected language the number of files, total lines, percentage, and the largest file.

```
$ code-stats ~/projects/my-app

  Language breakdown (by lines):

 Language    Lines     %      Files  Largest file
 C           12,340  62.1%     48   src/parser.c      (812)
 Python       3,210  16.2%     12   tools/sync.py     (540)
 Rust         2,100  10.6%      8   src/main.rs       (420)
 HTML           800   4.0%      3   web/index.html    (420)
 Shell          310   1.6%      5   scripts/deploy.sh (120)
 Other        1,100   5.5%     14   data/config.lock  (600)

  Largest file: src/parser.c — 812 lines
  Files: 90 analyzed, 3 skipped (2 binary)
  Total lines: 19,860
```

### Key features

- **22 languages** detected out of the box (C, C++, Rust, Python, JS, TS, Go, Java, and more)
- **Respects `.gitignore`** automatically — skips hidden files, `target/`, `node_modules/`, etc.
- **Binary-safe** — detects and skips binary files using Git's own null-byte heuristic
- **Handles non-UTF-8** — line counting works at the byte level, no encoding issues
- **Aligned output** — file paths and line counts are padded for clean readability

---

## Quick Start

```sh
cargo install --path .
code-stats .
```

To update, re-run the same command. To uninstall:

```sh
cargo uninstall code-stats
```

---

## Usage

```
code-stats [PATH]
```

| Argument | Default | Description |
|:---------|:--------|:------------|
| `PATH` | `.` | Directory to analyze |

```sh
# Analyze current directory
code-stats

# Analyze a specific project
code-stats ~/projects/my-app
```

---

## How It Works

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│   Scanner    │────▶│  Detection   │────▶│   Analyzer   │────▶│    Stats     │────▶│   Report     │
│  (ignore)    │     │  (extension  │     │  (binary     │     │  (aggregate  │     │  (comfy-     │
│  walk dirs   │     │   + filename)│     │   + lines)   │     │   + sort)    │     │   table)     │
└──────────────┘     └──────────────┘     └──────────────┘     └──────────────┘     └──────────────┘
```

| Stage | Module | What it does |
|:------|:-------|:-------------|
| **Scan** | `scanner.rs` | Walks the directory tree using the [`ignore`](https://crates.io/crates/ignore) crate (same engine as ripgrep) |
| **Detect** | `detection.rs` | Maps file extensions and special filenames (Makefile, Dockerfile) to languages |
| **Analyze** | `analyzer.rs` | Checks for binary files (null-byte in first 8 KB), counts lines at byte level |
| **Aggregate** | `stats.rs` | Groups by language, computes percentages, finds largest files |
| **Render** | `report.rs` | Formats an aligned table with comfy-table |

### Design principles

- **No `unwrap()` in production code** — all errors handled with `anyhow`
- **I/O separated from logic** — `analyzer.rs` does file I/O, `stats.rs` is pure computation
- **`lib.rs` exposes the full pipeline** — usable as a library, not just a CLI

---

## Supported Languages

| Language | Extensions |
|:---------|:-----------|
| C | `.c` |
| C Header | `.h` |
| C++ | `.cpp`, `.cc`, `.cxx`, `.c++`, `.hpp`, `.hxx`, `.hh` |
| Rust | `.rs` |
| Python | `.py`, `.pyi` |
| JavaScript | `.js`, `.mjs`, `.cjs` |
| TypeScript | `.ts`, `.mts`, `.cts` |
| Go | `.go` |
| Java | `.java` |
| Kotlin | `.kt`, `.kts` |
| Swift | `.swift` |
| PHP | `.php` |
| HTML | `.html`, `.htm` |
| CSS | `.css`, `.scss`, `.sass` |
| SQL | `.sql` |
| Shell | `.sh`, `.bash`, `.zsh`, `.fish` |
| JSON | `.json` |
| YAML | `.yaml`, `.yml` |
| TOML | `.toml` |
| Markdown | `.md`, `.markdown` |
| Makefile | `Makefile`, `makefile`, `GNUmakefile` |
| Dockerfile | `Dockerfile` |

Files with unrecognized extensions are grouped under **Other**.

---

## Tech Stack

| | Crate | Usage |
|:-|:------|:------|
| ![Clap](https://img.shields.io/badge/clap-4-5C6BC0?logoColor=white) | clap (derive) | CLI argument parsing + `--help` generation |
| ![Ignore](https://img.shields.io/badge/ignore-0.4-7E57C2?logoColor=white) | ignore | Directory traversal respecting `.gitignore` (by BurntSushi) |
| ![ComfyTable](https://img.shields.io/badge/comfy--table-7-9575CD?logoColor=white) | comfy-table | Terminal table rendering with alignment |
| ![Anyhow](https://img.shields.io/badge/anyhow-1-7986CB?logoColor=white) | anyhow | Error handling with context chaining |

**4 dependencies.** No async runtime, no serde, no proc-macro beyond clap derive.

---

## Project Structure

```
code-stats/
├── src/
│   ├── main.rs        # CLI entry point (clap)
│   ├── lib.rs         # Module declarations
│   ├── models.rs      # Shared types: Language, FileInfo, Report
│   ├── scanner.rs     # Directory traversal (ignore crate)
│   ├── detection.rs   # Language detection by extension + filename
│   ├── analyzer.rs    # Binary detection + line counting
│   ├── stats.rs       # Aggregation (pure, no I/O)
│   └── report.rs      # Terminal rendering (comfy-table)
├── Cargo.toml
├── LICENSE            # MIT
└── .github/
    └── workflows/
        └── ci.yml     # fmt + clippy + build
```

---

## License

MIT

---

<p align="center">
  <sub>Built by Mateon — Powered by Rust</sub>
</p>
