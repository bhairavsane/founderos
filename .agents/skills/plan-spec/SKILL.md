---
name: plan-spec
description: Use before writing code for any non-trivial feature, change or migration. Turns a vague request into testable acceptance criteria, non-goals, a risk list and a task breakdown where each task is small and has its own verification command.
---

# Plan & Spec

## Goal
Remove ambiguity before code exists. A plan is good when a different engineer could
verify the result without asking you anything.

## Step 0: Orient (read-only, no edits)
- Map the repo: entrypoints, layers, naming conventions, existing utilities, test/lint/build commands.
- Search for prior art: `grep`/symbol search for anything similar to what you are about to build.
- Read `docs/agent-lessons.md`, `docs/adr/`, and `README` if present.
- Record findings in 5-10 lines. Do not paste file contents.

## Step 1: Write the spec
Use `resources/spec-template.md`. Required sections:
1. **Problem** - one paragraph, the user pain, not the solution.
2. **Non-goals** - explicit list of what you will NOT build. This is your anti-slop shield.
3. **Acceptance criteria** - Given/When/Then, each independently testable, numbered (AC1..ACn).
4. **Constraints** - perf budget, compatibility, security/privacy, deadlines, cost.
5. **Assumptions** - things you decided without asking; the human can veto.
6. **Risks** - top 3 with mitigation.

## Step 2: Break into tasks
Rules for every task:
- One behavior change per task; <= ~150 changed lines; touches <= 5 files.
- Names the AC it satisfies (`T3 -> AC2`).
- Has a **verification command** (test name, curl, script) that fails before and passes after.
- Ordered so the system is runnable/green after every task (vertical slices, not layers).
- Refactors needed to enable a feature are their own earlier task, behavior-preserving.

Template line:
`T2 -> AC1,AC3 | add expiry check in token service | verify: pnpm test token.expiry | files: ~3`

## Step 3: Approval gate
Keep the plan <= 40 lines. STOP and ask "Approve this plan to proceed?" when the change touches > 2 files,
alters a schema/API contract/migration, adds a dependency, or touches auth/billing/infra.
Below that threshold, state the plan in 3-5 lines and proceed.

## Anti-patterns
- Specs that restate the request without testable criteria.
- A task called "implement feature X" (too big to verify).
- Planning infrastructure, abstractions or config "while we're here".
- Writing a spec file for a 10-line change (put it in the plan artifact only).

## Output
Plan artifact (and `docs/specs/<slug>.md` only for large/risky work). Then hand off to
`design-architecture` if any contract, schema or boundary changes; otherwise to `build-minimal-code`.
