# xPlagiarismChecker — Project Vision & Architecture

## Context

The project currently has storage/import plumbing but no plagiarism-checking
feature and almost no database UI (`TODO.md` item 3, "Plagiarism checking
functionality," is still open). This document lays out the target
architecture and UI for the finished tool — a desktop app where a tutor
manages one "database" per subject, feeds it lab submissions, and runs
plagiarism comparisons over them — before any code is written.

## Mental model

- **App** = the running program. Has app-wide settings (language, theme,
  log level) that apply no matter which database you're working in.
- **Database** = one `.xai` file (zip container, same idea as `.docx`/`.jar`)
  representing one subject/course. Holds its own settings (how submissions
  are named/grouped, which files count as "code" vs "noise") and the actual
  submissions.
- **Submission** = one student's one piece of work (e.g. "Petrov — Lab 1"):
  a student name, an optional assignment name, and the code files inside it.
- **Comparison** = a one-off action you run against an open database: pick
  who to compare against whom (an ordinary option you set each time, not an
  architectural concern) and which **detection strategy** computes
  similarity (the actual pluggable piece — diff-based, parser-based, etc.),
  get back a scored list of pairs — each backed by a file-by-file breakdown,
  not just one number — then manually review any pair down to the block
  level.

Everything above the "Database" line already exists in some form; everything
from "Submission" down is what needs building.

## Module map

**Engine (`engine` crate — no UI, pure logic):**

| Module | Responsibility |
|---|---|
| Storage | The `.xai` database format: metadata, per-database settings, submissions. Save/load. *(exists)* |
| Import | Turn a folder or archive (zip/rar/7z/tar) into a `Submission`, applying ignore rules. *(exists)* |
| Ignore Rules | The whitelist of accepted file extensions and blacklist of ignored directory names, as individually add/remove/enable-toggle-able items. *(needs rework — currently a flat baked-in list, needs to become per-item and per-database)* |
| Naming Pattern | How a folder/archive name decodes into student + assignment. *(exists: student_task, task_student, student-only)* |
| **Comparison (new)** | Given a database's submissions, run options (who gets compared with whom) and a chosen **detection strategy** (the pluggable algorithm that scores similarity), produce a ranked list of pair scores — each with a file-by-file block-match breakdown — plus an on-demand line-level diff for one specific block. |

**Frontend (Tauri + React/TypeScript — presentation only, talks to Engine):**

The original plan was an `egui` desktop GUI (the old `gui` crate). That's
been replaced: the interface is now a **Tauri** app — a Rust backend
(`src-tauri`, replacing `gui`, still just a thin layer calling into
`engine`) paired with a **React + TypeScript** frontend built with **Vite**.
Reasoning: hand-building a polished, "modern SaaS-looking" UI (and
especially a good code-diff review screen) in an immediate-mode Rust GUI
toolkit was judged too costly compared to leaning on the web
ecosystem's existing components — see "Frontend technology" below.

| Area | Responsibility |
|---|---|
| App Settings | Language, theme, log level. *(exists today in the old egui GUI; to be rebuilt)* |
| About | Version/links. *(same)* |
| **Database workspace (new/rebuilt)** | Create/open/save a database; edit its naming pattern and ignore rules; import submissions; browse what's loaded; run comparisons; review results. |

The key architectural point: **ignore rules and naming pattern move from
being edited as global app settings to being edited as part of a database**,
matching how you described it ("app settings vs db settings"). The global
"open the ignore list in a text editor" flow goes away; editing happens
in-app, per database. This point is independent of the frontend technology
swap above and still holds.

## Frontend technology

Decided after comparing egui, Slint, Iced, Qt, and a web frontend — see the
conversation history for the full comparison. Landed on:

- **Tauri v2** as the shell. `engine` stays a plain Rust library, untouched;
  `src-tauri` (a small Rust crate, replacing `gui`) exposes `engine`
  functionality as commands the frontend calls via `invoke(...)`.
- **React + TypeScript**, built with **Vite** — chosen over Svelte/Vue
  mainly because it's the best-supported target for AI-assisted learning
  and has the deepest component ecosystem.
- **Tailwind CSS** for styling and **shadcn/ui** for components (buttons,
  dialogs, tables, forms, tabs) — copy-owned component source rather than a
  black-box library, and it gets a "modern SaaS" look with minimal manual
  design work.
- **Monaco Editor** (`@monaco-editor/react`) specifically for the Review
  ("work zone") screen — its built-in diff mode is the reason a web
  frontend was worth the learning investment; this was the one screen every
  Rust-native GUI option would have required building by hand.

**Dependency policy: add packages only when the screen/feature that needs
them is actually being built, not upfront.** The initial scaffold is kept to
just what a bare Tauri + React + TypeScript + Vite template needs. Concretely:

| Package | Added when |
|---|---|
| `react`, `react-dom`, `@tauri-apps/api`, `typescript`, `vite` | Initial scaffold |
| `tailwindcss` + config | First real screen (Settings) |
| `shadcn/ui` components | One at a time, per component actually used (its CLI adds one file per component — inherently incremental) |
| `@tauri-apps/plugin-dialog` | Database create/open, submission import (native file/folder pickers) |
| `@monaco-editor/react` | Review ("work zone") screen |
| `similar` (Rust, in `engine`) | Comparison module (detection strategy phase) |

This mirrors the existing Cargo dependency style in this repo already —
every crate in `engine`/`gui`'s `Cargo.toml` is there for a specific,
currently-used reason, nothing speculative.

## Comparison module design

Running a comparison involves two separate things, and only one of them is
the "modular strategy" architecture — the other is just an ordinary option
you set each time, no different from picking a theme.

### Run options (plain, per-run — not a plugin system)

Which pairs of submissions are even candidates for comparison this time.
Always excludes a student against their own other submissions. Picked fresh
every run via a simple dropdown, nothing stored on the database:

- *Full cross-check*: every submission against every other.
- *One assignment*: only submissions for a chosen lab (e.g. "Lab 1") against
  each other — so Lab 1 never gets compared against Lab 2.
- *One student*: one chosen student's submissions against everyone else's
  (spot-check a specific person).

### Detection Strategy (the actual pluggable architecture)

This is the part that has to be a real strategy pattern: a common interface
that takes two submissions and returns a **list of block matches** — not a
single opaque number:

- Each **block match** identifies one block (in practice, a file) from
  submission A and one from submission B, plus that specific pair's own
  similarity percentage — e.g. `A/src/main.rs` vs `B/main.rs`: 92%.
- A shared aggregation step — *not* the strategy itself, so every
  strategy's output stays comparable — rolls the block list up into one
  **overall similarity percentage** for the submission pair, plus a short
  **conclusion**: a plain-language verdict (e.g. "likely plagiarism" /
  "worth a manual look" / "no significant overlap") derived from that
  percentage. This is what turns a pile of per-file numbers into the one
  headline result a tutor scans in the Results list.

The block list a strategy returns is **complete and unfiltered** — the
overall percentage and conclusion are computed from every block, so they
stay an honest measure of the whole pair regardless of how the tutor is
currently viewing the breakdown.

Interchangeable implementations behind this interface:

- *Diff-based (v1)*: pair up files between the two submissions (by
  filename, or best-match if names don't line up) and run a line-based
  diff on each pair, producing that pair's block-match percentage. Simple,
  language-agnostic, fast to ship.
- *Parser-based (future)*: parse each file (per-language) into tokens or a
  syntax tree and compare structurally, file by file — catches renamed
  variables/reordered code that a text diff misses.
- *Other future strategies* (e.g. a token/n-gram engine) plug in the same
  way, still returning the same block-match shape.

Because every strategy returns the same block-match shape, adding a new
detection strategy later means writing a new implementation and adding it
to the picker — it never requires touching run options, the aggregation
step, the results list, or the review UI. Run options and detection
strategy are both exposed as simple pickers in the UI (the same
dropdown/radio style already used for theme/language/log-level in Settings)
— the difference between them is purely internal: run options are an enum
of plain values, detection strategy is an extension point.

### Similarity threshold (noise filter, not part of detection)

Real submissions always share *some* incidental overlap (boilerplate,
imports, the assignment's own starter code), so showing every block match
down to 0% would bury the real hits in noise. A **similarity threshold**
(e.g. defaulting to 15%) hides block matches below it from the block list —
purely a display filter applied on top of the already-computed, unfiltered
block list, so it's adjustable live with no recomputation, and it never
changes the overall percentage/conclusion for the pair (those are always
computed from every block). Blocks that pass the threshold are shown sorted
descending by their own percentage, worst offenders first.

## UI layout

**Navigator (left sidebar, exists today):** Database / Settings / About /
Exit. Unchanged.

**Settings page (exists, shrinks):** now purely app-level —
Language, Log Level, Theme. Ignore-rule editing moves out of here entirely.

**About page:** unchanged.

**Database page (rebuilt — this is where almost all new work lands):**
Behaves differently depending on whether a database is open.

- *Nothing open* — two actions: **Create New Database** (name, optional
  description, naming pattern, where to save the `.xai` file) and **Open
  Database** (file picker for an existing `.xai`).
- *Database open* — a workspace with these sections:
  1. **Overview** — name/description, quick stats (submission count,
     student count).
  2. **Database Settings** — naming pattern; two ignore-rule lists
     (accepted extensions, ignored directories), each shown as a checklist
     you can tick on/off, add a new entry to, or delete an entry from —
     no external text file involved.
  3. **Submissions** — an "add" action (pick archives and/or folders,
     imported via the existing Import module), a browsable list/tree of
     what's loaded grouped by student → assignment, with the ability to
     remove a submission, and any import failures shown as a dismissible
     notice (bad filenames, unsupported/corrupt files) rather than silently
     dropped.
  4. **New Comparison** — pick run options (+ the specific assignment/student
     if that option needs one) and a detection strategy, then run it.
  5. **Results** — the scored list from the last run, sorted by overall
     similarity, each row identifying the two students/submissions, the
     overall percentage, and the conclusion; click a row to open it.
  6. **Review ("work zone")** — opens one selected pair top-down: overall
     percentage + conclusion first, then its block matches (file name vs.
     file name, each with its own percentage) filtered by the similarity
     threshold and sorted descending — worst first — with the threshold
     itself adjustable right there for a quick "show me more/less" without
     re-running anything; picking one block opens the actual
     side-by-side/highlighted line diff for that specific file pair, so the
     tutor can see *why* it scored the way it did and make the final call
     manually.
  7. **Save** — persists the database (settings + submissions) back to its
     `.xai` file. Comparison results are *not* saved — they're a
     recomputed-on-demand view, not part of the file format, so re-running
     "New Comparison" after reopening a database is expected.

## Settings inventory (final state)

- **App-level** (`Settings` page): Language, Log Level, Theme, default
  Similarity Threshold (also live-adjustable in the Review view itself).
- **Per-database** (`Database` page → Database Settings): Naming pattern
  (student/task separator style), Accepted Extensions (checklist),
  Ignored Directories (checklist).
- **Per-run, not saved anywhere**: Run Options (comparison scope),
  Detection Strategy (diff-based v1; parser-based and others to follow via
  the same pluggable interface).

## Suggested build order

0. Scaffold the Tauri + React + TypeScript project (replacing the old
   egui `gui` crate) with the minimal starter dependency set only.
1. Ignore rules rework (per-item, per-database) + Database Settings section
   — foundational, no algorithm needed yet.
2. Database create/open/save + Submissions section — makes the app usable
   end-to-end for building up a database, even with no checking yet.
3. Comparison module in the engine: run options, plus the diff-based
   detection strategy as the first implementation of the pluggable
   interface.
4. New Comparison / Results / Review sections in the frontend.

This lets the project be useful (organizing labs into databases) well before
the checking algorithm itself is finished, and keeps each step independently
testable.
