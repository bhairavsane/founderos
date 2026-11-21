---
name: engineering-core
description: Main router for all software work. Use at the start of any coding, feature, bug fix, refactor, deploy or maintenance task. Enforces minimal code, small files (max 250 lines), deterministic verification and the plan-design-build-test-deploy-maintain lifecycle by pointing to the right sub-skill.
---

# Engineering Core (main skill)

## Goal
Produce the smallest correct, verified, maintainable change. Every line must be
traceable to a requirement. Every claim must be backed by tool output.

## The 10 laws
1. **Smallest diff.** Solve exactly the stated acceptance criteria, nothing adjacent.
2. **Read before write.** Search for existing code, conventions and utilities. Reuse.
3. **No speculation + Rule of Three.** No "future-proofing", unused params, single-use wrappers,
   interfaces with one implementation, or config nobody asked for. Abstract only on the 3rd real reuse.
4. **No drive-by edits.** No unrelated renames, reformatting or refactors.
5. **Verify, don't assume.** Check real signatures/versions; run the code.
6. **Evidence over claims.** Done = script exit code 0 + pasted summary.
7. **Never game the checks.** Fix causes, not gates.
8. **Small files.** <= 250 lines/file (aim 200), <= 40 lines/function, one responsibility each.
9. **Comments say why**, never what. Delete narration comments.
10. **Learn.** Human correction -> one line in `docs/agent-lessons.md`.
11. **Context hygiene.** One feature per thread. End each phase with a <= 10 line state summary
    (decisions, files touched, open items). Start a fresh thread when switching features or when the thread gets long.

## Lifecycle router
Load the skill for the current phase. Do not skip a phase; small tasks make phases tiny, not absent.

| Phase | Skill | Exit gate (must be true to move on) |
|---|---|---|
| 1 Plan | `plan-spec` | Acceptance criteria are testable; tasks <= ~150 changed lines each |
| 2 Design | `design-architecture` | Contract/schema defined; failure modes listed; boring tech chosen |
| 3 Build | `build-minimal-code` + `modular-file-size` | Diff traces to criteria; no file > 250 lines |
| 4 Test | `test-strategy` + `verify-deterministic` | `verify.sh` exit 0 (not 1, not 3); new behavior has a test that failed first |
| 4b Review | `self-review` + `security-baseline` | Slop scan clean; security checklist done |
| 5 Deploy | `deploy-release` | Human approved; rollback written; smoke test passes |
| 6 Maintain | `maintain-evolve` | Deps/dead code/runbooks/lessons updated |
| Any bug | `debug-root-cause` | Regression test exists and passes |

## Task sizing
- **Trivial** (<= 1 file, < 30 lines, no behavior risk): skip written spec; still run verify.
- **Normal**: short plan (artifact, <= 40 lines) -> build -> verify -> self-review.
- **Large / risky** (new module, schema/migration, auth, payments, infra): full spec in
  `docs/specs/<slug>.md`, ADR for key decisions, human approval of plan before building.

## Approval gate (stop and wait for the human)
Any of: change touches > 2 files, alters a schema/API contract/migration, adds a dependency,
touches auth/billing/permissions/infra, or any deploy/destructive command. End the plan with
"Approve this plan to proceed?" Below that threshold, state the plan in 3-5 lines and proceed.

## Hard limits (enforced by scripts, not by goodwill)
- File length: `check_file_size.sh` fails > 250 lines on every file you touch (old untouched debt is ignored).
- Bypass markers: `check_bypass.sh` fails on added `any`, `@ts-ignore`, `eslint-disable`, `.skip`/`.only`, etc.
- Diff budget: `diff_budget.sh` warns > 300 added lines per task; split the task.
- New dependency: needs one-line justification (why not stdlib / existing dep), and license + audit check.
- Secrets: `secrets_scan.sh` must be clean.

## Stop-and-ask triggers
Ask the human (one question, with your recommended default) when: requirements conflict,
a destructive/irreversible action is needed, a new paid service or dependency is
required, auth/billing/data-retention behavior is unspecified, or 3 fix attempts on one
gate have failed.

## Final report format (always)
```
Changed: <files, one line each, why>
Verified: <paste verify.sh summary>
Not verified: <anything skipped and why>
Risks / follow-ups: <max 3 bullets>
```
Never write "should work". Write what was run and what it returned.
