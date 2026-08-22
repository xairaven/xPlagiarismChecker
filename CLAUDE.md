# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

xPlagiarismChecker is a desktop application for detecting plagiarism in
student code submissions. It is early-stage: submissions can be imported and
stored in a project database file, but the actual plagiarism-comparison
algorithm has not been implemented yet (see `TODO.md`). See `VISION.md` for
the full target architecture and UI design.

**Frontend migration in progress.** The GUI is being rewritten from an
`egui` desktop app (the `gui` crate below) to a **Tauri + React/TypeScript**
app, per `VISION.md`'s "Frontend technology" section — plain `egui` was
judged too costly for the polished UI (especially the code-diff review
screen) this app needs. Until that migration lands: do not add new features
to the `gui` crate, it is being removed; this file's `gui` architecture
section below describes the soon-to-be-legacy code for reference only. Once
the Tauri/React scaffold exists, this file needs a real rewrite of the
workspace layout, commands, and frontend architecture sections to match.

## Workspace layout

Cargo workspace, currently two members (about to become `engine` +
`src-tauri`, see above):

- `engine/` — plain Rust library, no GUI dependencies. Owns file/archive
  ingestion, submission parsing, and the on-disk database format. Unaffected
  by the frontend migration.
- `gui/` — `eframe`/`egui` desktop application (binary name
  `xPlagiarismChecker`), depends on `engine`. **Being replaced** — see the
  migration note above.

## Commands

```
cargo build --workspace           # build everything
cargo run -p gui                  # run the desktop app
cargo check --workspace           # fast type-check
cargo clippy --workspace --all-targets   # lint (pedantic + nursery, see below)
cargo fmt --all                   # format (rustfmt.toml: edition 2024, max_width 90)
```

There are currently no automated tests in the repo. If adding some, run a
single crate's tests with `cargo test -p engine` / `cargo test -p gui`.

The `compress-tools` crate (used by `engine` for archive extraction) links
against system `libarchive`; it must be installed (with dev headers) for the
project to build.

## Lint policy (enforced via `[workspace.lints]` in root `Cargo.toml`)

`clippy::unwrap_used`, `clippy::expect_used`, `clippy::panic`, and
`clippy::indexing_slicing` are all set to `deny`, plus `pedantic`/`nursery` as
warnings, and `unsafe_code` is denied. Do not introduce `.unwrap()`,
`.expect()`, `panic!()`, or direct slice indexing in new code — use
`thiserror`-based `Result` propagation and `.get()`/pattern matching instead,
following the existing error-enum-per-module style (`engine::errors::LibError`,
`engine::io::FileError`, `gui::errors::ProjectError`, etc.).

## `engine` architecture

- `models::submission` — `Submission` (a student's turned-in work) holds
  `SubmissionMetadata` (student name + optional assignment title) and a list
  of `CodeFile` (relative path, text content, extension).
- `models::pattern::FileNamePattern` — describes how a submission's
  archive/folder name encodes student + assignment (`StudentTask`,
  `TaskStudent`, or `StudentOnly`), and parses a filename into
  `SubmissionMetadata` accordingly. This pattern is a per-database setting.
- `models::ignore::IgnoreList` — accepted file extensions / ignored directory
  names. Defaults are baked in at compile time from
  `engine/assets/accepted_extensions.txt` and `ignored_directories.txt` via
  `include_str!`.
- `io::FileLoader` — entry point for importing submissions from a list of
  paths. Each path is either a directory (walked with `walkdir`) or a
  supported archive (`zip`/`rar`/`7z`/`tar`, extracted uniformly through
  `compress-tools`). Files are filtered by the ignore list *before* extraction
  where possible, then binary-sniffed (null-byte check) as a safety net.
  Per-path failures are collected as `BadFile` rather than aborting the whole
  import.
- `models::database::Database` — the project save format. A `Database` is
  serialized as a **zip file** (extension `.xai`) containing `meta.json`
  (`DatabaseMetadata`), `settings.json` (`DatabaseSettings`, i.e. the ignore
  list + file name pattern used for this project), and a `submissions/`
  tree laid out as `submissions/<student>/<assignment>/<relative_path>` (the
  `<assignment>` segment is omitted for `FileNamePattern::StudentOnly`).
  `Database::load` reverses this by grouping zip entries back into
  submissions according to the stored `file_name_pattern`. Saves are
  crash-safe: written to a `.tmp` file first, then renamed over the target.

## `gui` architecture (legacy — being removed, kept here only as a reference)

Skip this unless you're specifically working on the migration itself. The
current `egui` app follows a "render, then apply" split (widgets never
mutate state directly; they push a `UiCommand` onto a channel, drained once
per frame by `UiCommandHandler`) with a `Context`/`GuiState` split, a
`Page` trait + `PageId` enum for navigation, a `SettingWidget<V>` abstraction
for settings rows, `rust-i18n`-based localization (`locales/en.json`/`ua.json`),
and `egui-aesthetix` for theming. None of this carries over to the Tauri/React
rewrite in any code form — only the *concepts* it enforced (settings are
per-app vs. per-database, errors surface as dismissible notices, i18n exists
for en/ua) need to be re-achieved in the new stack, per `VISION.md`.