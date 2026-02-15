# code-stats

A fast, local CLI tool that analyzes source code repositories and displays language statistics. Think of it as a lightweight [GitHub Linguist](https://github.com/github-linguist/linguist) that runs entirely offline.

## Example output

```
$ code-stats .

  Language breakdown (by lines):

 Language    Lines     %      Files  Largest file
 C           12,340  62.1%     48   src/parser.c       (  812)
 Python       3,210  16.2%     12   tools/sync.py      (  540)
 Rust         2,100  10.6%      8   src/main.rs        (  420)
 HTML           800   4.0%      3   web/index.html     (  420)
 Shell          310   1.6%      5   scripts/deploy.sh  (  120)
 Other        1,100   5.5%     14   data/config.lock   (  600)

  Largest file: src/parser.c — 812 lines
  Files: 90 analyzed, 3 skipped (2 binary)
  Total lines: 19,860
```

## Installation

```sh
cargo install --path .
```

To update, re-run the same command. To uninstall:

```sh
cargo uninstall code-stats
```

## Usage

```
code-stats [PATH]
```

- `PATH` defaults to the current directory (`.`)
- Respects `.gitignore` rules automatically
- Skips hidden files and common build directories (`.git`, `target`, `node_modules`)

```sh
# Analyze current directory
code-stats

# Analyze a specific project
code-stats ~/projects/my-app
```

## Supported languages

| Language | Extensions |
|----------|-----------|
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

## How it works

1. Walks the directory tree using the [`ignore`](https://crates.io/crates/ignore) crate (same engine as ripgrep), respecting `.gitignore`
2. Detects languages by file extension and special filenames
3. Identifies binary files using a null-byte heuristic on the first 8 KB (same method as Git)
4. Counts lines at the byte level — handles non-UTF-8 files and CRLF line endings correctly
5. Aggregates statistics and renders a formatted table

## License

MIT
