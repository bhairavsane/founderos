---
name: verify-deterministic
description: Use before claiming any work is done, fixed, passing or ready. Runs one deterministic gate script (changed-file size limit, bypass-marker scan, secrets, lint, typecheck, tests, build, audit, diff budget) and requires its exit code and summary as evidence. Also defines pre-commit/CI setup so the gate cannot be bypassed.
---

# Deterministic Verification

## Principle
Opinions are not evidence. Trust, highest first: 1 compiler/types, 2 tests, 3 linters, 4 gate exit codes,
5 running the real thing (curl, CLI, browser), 6 your own reasoning (never sufficient alone).

## The gate
```
bash .agents/skills/verify-deterministic/scripts/verify.sh          # full
bash .agents/skills/verify-deterministic/scripts/verify.sh --fast   # inner loop: no build/audit
```
Gates in order: file-size -> no-bypass -> secrets -> lint -> typecheck -> test -> build -> audit -> diff-budget.
Auto-detects Node (npm/pnpm/yarn), Python, Go.

| Exit | Meaning | Allowed to say "done"? |
|---|---|---|
| 0 PASSED | every gate passed and all REQUIRE'd gates ran | yes |
| 1 FAILED | a gate failed | no, fix the cause |
| 3 UNVERIFIED | a required gate (default: `test`) did not run | no, add the missing gate or ask the human |

`REQUIRE="lint,typecheck,test"` raises the bar. Only the human may lower it.

## What the scripts enforce
- `check_file_size.sh`: every file you add/modify <= 250 lines (ratchet; `--all` audits the whole repo).
- `check_bypass.sh`: fails on ADDED `any`, `@ts-ignore`, `@ts-nocheck`, `eslint-disable`, `# type: ignore`,
  `# noqa`, `.skip`/`.only`, `xit`, `pytest.mark.skip`; warns on `console.log`, `debugger`, `TODO`.
- `secrets_scan.sh`: gitleaks if installed, else regex scan; tracked `.env` files fail.
- `diff_budget.sh`: warns above `DIFF_BUDGET` (300) added lines; `STRICT=1` makes it fail.

## Rules
1. Run after each task and before the final report. Paste the summary table verbatim.
2. A SKIP is not a pass. List every SKIP under "Not verified".
3. On failure: read the first error, fix the root cause, re-run. Max 3 attempts per gate, then stop and report (`debug-root-cause`).
4. Never edit the gate, tests, lint config or thresholds to get green. Propose changes to the human separately.
5. Flaky = bug. Re-run once to confirm, then fix the nondeterminism.
6. Behavior changes need runtime proof: run the app/CLI and hit the real path (curl, command, browser agent); capture output.
7. Any edit after a green run invalidates it; re-run.
8. Read `git diff` once more after green: no debug leftovers, nothing out of scope.

## Make it unbypassable
- **Pre-commit** (lefthook/husky): `verify.sh --fast`.
- **CI**: same script, full, required on the main branch (`deploy-release/resources/ci.yml`).
- **Reviewer agent**: separate session re-runs the gate and `self-review`.

## Optional hardening
`strict: true` + `noUncheckedIndexedAccess`; `eslint --max-warnings 0`; `knip`/`vulture` (dead code); `jscpd` (duplication);
Stryker/mutmut (mutation tests on core logic); diff-cover (coverage of changed lines); `osv-scanner`; `oasdiff` (API breaks).

## Evidence format
```
VERIFY (full) RESULT: PASSED (exit 0)
PASS file-size | PASS no-bypass | PASS secrets | PASS lint | PASS typecheck | PASS test | PASS build | SKIP audit (offline)
Runtime: GET /v1/jobs/42 -> 200 {"id":42,...}
```
