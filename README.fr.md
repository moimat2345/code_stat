<div align="center">

# code-stats

**Comprenez votre codebase en un coup d'oeil — entierement hors ligne.**

Un outil CLI rapide qui parcourt recursivement un repertoire, detecte les langages de programmation
et affiche un resume clair du nombre de fichiers, de lignes et des plus gros fichiers. Comme GitHub Linguist, mais local et instantane.

![Rust](https://img.shields.io/badge/Rust-stable-000000?logo=rust&logoColor=white)
![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Linux%20%7C%20Windows-5C6BC0)
![License](https://img.shields.io/badge/License-MIT-green)
![Dependencies](https://img.shields.io/badge/Dependencies-4-7E57C2)

<a href="README.md"><img src="https://img.shields.io/badge/%F0%9F%87%AC%F0%9F%87%A7_Read_in_English-blue?style=for-the-badge" alt="Read in English"></a>

</div>

---

## Ce que ca fait

code-stats analyse un depot de code source et produit une **repartition par langage en nombre de lignes**, affichant pour chaque langage detecte le nombre de fichiers, le total de lignes, le pourcentage et le plus gros fichier.

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

### Fonctionnalites cles

- **22 langages** detectes nativement (C, C++, Rust, Python, JS, TS, Go, Java, et plus)
- **Respecte `.gitignore`** automatiquement — ignore les fichiers caches, `target/`, `node_modules/`, etc.
- **Compatible binaire** — detecte et ignore les fichiers binaires avec l'heuristique null-byte de Git
- **Gere le non-UTF-8** — le comptage de lignes fonctionne au niveau des octets, aucun probleme d'encodage
- **Sortie alignee** — les chemins de fichiers et compteurs de lignes sont paddes pour une lisibilite optimale

---

## Demarrage rapide

```sh
cargo install --path .
code-stats .
```

Pour mettre a jour, relancez la meme commande. Pour desinstaller :

```sh
cargo uninstall code-stats
```

---

## Utilisation

```
code-stats [CHEMIN]
```

| Argument | Defaut | Description |
|:---------|:-------|:------------|
| `CHEMIN` | `.` | Repertoire a analyser |

```sh
# Analyser le repertoire courant
code-stats

# Analyser un projet specifique
code-stats ~/projects/my-app
```

---

## Comment ca marche

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│   Scanner    │────▶│  Detection   │────▶│   Analyzer   │────▶│    Stats     │────▶│   Report     │
│  (ignore)    │     │  (extension  │     │  (binary     │     │  (aggregate  │     │  (comfy-     │
│  walk dirs   │     │   + filename)│     │   + lines)   │     │   + sort)    │     │   table)     │
└──────────────┘     └──────────────┘     └──────────────┘     └──────────────┘     └──────────────┘
```

| Etape | Module | Ce qu'il fait |
|:------|:-------|:--------------|
| **Scan** | `scanner.rs` | Parcourt l'arborescence avec le crate [`ignore`](https://crates.io/crates/ignore) (meme moteur que ripgrep) |
| **Detection** | `detection.rs` | Associe les extensions et noms speciaux (Makefile, Dockerfile) aux langages |
| **Analyse** | `analyzer.rs` | Detecte les fichiers binaires (null-byte dans les 8 premiers Ko), compte les lignes au niveau octet |
| **Agregation** | `stats.rs` | Regroupe par langage, calcule les pourcentages, identifie les plus gros fichiers |
| **Rendu** | `report.rs` | Formate un tableau aligne avec comfy-table |

### Principes de conception

- **Aucun `unwrap()` en production** — toutes les erreurs gerees avec `anyhow`
- **I/O separee de la logique** — `analyzer.rs` gere les I/O, `stats.rs` est du calcul pur
- **`lib.rs` expose le pipeline complet** — utilisable comme bibliotheque, pas seulement en CLI

---

## Langages supportes

| Langage | Extensions |
|:--------|:-----------|
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

Les fichiers avec des extensions non reconnues sont regroupes sous **Other**.

---

## Stack technique

| | Crate | Utilisation |
|:-|:------|:------------|
| ![Clap](https://img.shields.io/badge/clap-4-5C6BC0?logoColor=white) | clap (derive) | Parsing des arguments CLI + generation du `--help` |
| ![Ignore](https://img.shields.io/badge/ignore-0.4-7E57C2?logoColor=white) | ignore | Parcours de repertoire respectant `.gitignore` (par BurntSushi) |
| ![ComfyTable](https://img.shields.io/badge/comfy--table-7-9575CD?logoColor=white) | comfy-table | Rendu de tableaux en terminal avec alignement |
| ![Anyhow](https://img.shields.io/badge/anyhow-1-7986CB?logoColor=white) | anyhow | Gestion d'erreurs avec chainage de contexte |

**4 dependances.** Pas de runtime async, pas de serde, pas de proc-macro au-dela de clap derive.

---

## Structure du projet

```
code-stats/
├── src/
│   ├── main.rs        # Point d'entree CLI (clap)
│   ├── lib.rs         # Declarations de modules
│   ├── models.rs      # Types partages : Language, FileInfo, Report
│   ├── scanner.rs     # Parcours de repertoire (crate ignore)
│   ├── detection.rs   # Detection de langage par extension + nom de fichier
│   ├── analyzer.rs    # Detection binaire + comptage de lignes
│   ├── stats.rs       # Agregation (pur, sans I/O)
│   └── report.rs      # Rendu terminal (comfy-table)
├── Cargo.toml
├── LICENSE            # MIT
└── .github/
    └── workflows/
        └── ci.yml     # fmt + clippy + build
```

---

## Licence

MIT

---

<p align="center">
  <sub>Construit par Mateon — Propulse par Rust</sub>
</p>
