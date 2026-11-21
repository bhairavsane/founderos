---
name: modular-file-size
description: Use whenever creating files, planning file layout, or touching a file near 200 lines. Keeps every source file at 200-250 lines maximum by splitting on responsibility, and explains how to safely split an existing oversized file without changing behavior.
---

# Modular Files (max 250 lines)

## Limits
| Unit | Target | Hard max |
|---|---|---|
| File | <= 200 lines | 250 lines |
| Function | <= 25 lines | 40 lines |
| Parameters | <= 3 | 4 (then use an object) |
| Nesting depth | <= 2 | 3 |
| Exports per file | 1 main + closely related types | ~5 |

Enforced by `check_file_size.sh` as a **ratchet**: every file you add or modify must be <= 250 lines;
old untouched files are ignored so legacy debt never blocks work. `check_file_size.sh --all` audits the
whole repo (use it in maintenance). Generated files, lockfiles, migrations excluded. Tests count too.

## Split by responsibility, never by line count
Good seams (one reason to change per file):
- `*.types.ts` / `*.schema.ts` / constants -> own file
- `*.logic.ts` or `*.service.ts` pure functions; `*.repo.ts` / `*.api.ts` for I/O; view files for presentation only
- transport (route/controller/handler) -> service (rules) -> repository (I/O)
- one React component per file; hooks in `use*.ts`; presentational vs container
- pure `format*`/`parse*`/`validate*` functions grouped by noun, not by "utils"
- one test file per unit; shared fixtures in a small `fixtures/` file

Bad seams: `part1.ts`/`part2.ts`, `helpers.ts`/`utils.ts` junk drawers, one-function-per-file
for trivial 3-liners, circular imports to "make it fit".

## Naming and layout
- File name = main export (`token-service.ts` exports `TokenService`/`issueToken`).
- Feature folders: `features/billing/{billing.routes.ts, billing.service.ts, billing.repo.ts, billing.schema.ts, billing.service.test.ts}`.
- Index/barrel files only at public boundaries; they contain re-exports and nothing else.
- Dependencies point inward: routes -> service -> repo. No upward or sideways imports across features without a public interface.

## Generating new code in chunks
1. Plan the file list first (names + one-line responsibility + expected size).
2. Create the types/schema file. Verify.
3. Create the next file, <= 200 lines. Verify (`verify.sh --fast`).
4. If a file passes ~180 lines while still incomplete, STOP and extract a seam *now*, before continuing.
5. Wire files together last, in a thin composition file.

## Splitting an existing oversized file (behavior-preserving)
1. Ensure tests cover current behavior. If not, write characterization tests first (assert today's outputs).
2. Run `verify.sh`; it must be green BEFORE you start.
3. Extract one cohesive group at a time: move code verbatim, update imports, change nothing else.
4. Run `verify.sh --fast` after each extraction. Commit each extraction separately.
5. Do not mix refactoring with feature work in one commit/PR.
6. Re-run `check_file_size.sh`; stop when every resulting file is under the limit.

If a file is touched and already > 250 lines, extracting what you touch is part of the task
(behavior-preserving, separate commit) unless the human defers it.

## Exceptions (rare, must be justified in the PR)
Data tables, generated code, large SQL/schema snapshots. Add the path to `.agents/size-ignore`
(one glob per line) with a comment saying why. Never raise the global limit to make a file pass.
