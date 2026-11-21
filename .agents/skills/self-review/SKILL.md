---
name: self-review
description: Use after code is written and before reporting done. An adversarial diff review that deletes slop, checks every changed line maps to an acceptance criterion, and hunts edge cases and security issues. Best run by a separate reviewer agent with no builder context.
---

# Self-Review (adversarial pass)

Act as a skeptical senior reviewer who dislikes unnecessary code. Read `git diff` line by line.

## 1. Traceability
For each hunk: which AC requires it? No answer -> delete it. List deleted lines in the report.

## 2. Slop scan (grep the diff)
- Narration comments, restating docstrings, commented-out code, `TODO`, `console.log`/`print` debug.
- Single-use helpers/wrappers, one-impl interfaces, unused exports/params/imports/vars/flags.
- Swallowed errors, catch-all `any`, `@ts-ignore`, `eslint-disable`, `# noqa`, `.skip`, `.only`.
- Hardcoded URLs/IDs/keys/magic numbers; fake data; duplicated logic that already exists in the repo.
- Reformatting or renames outside the change. Extra files nobody asked for.

## 3. Correctness
- Edge cases: empty, null/undefined, 0/1/max, duplicates, unicode, timezone, concurrency, retries, partial failure.
- Every promise awaited; every resource closed; no unbounded loops/queries/memory.
- Backward compatibility of API, DB and events. Idempotency where retried.

## 4. Quality gates
- Any file > 200 lines? Function > 40? Nesting > 3? Split now.
- Names meaningful; behavior tested; test failed before the fix; tests deterministic.
- Load `security-baseline` for any input/auth/data/network touch.

## 5. Re-verify
After any deletion or fix, re-run `verify.sh`. Reviewer must run it independently; do not trust the builder's paste.

## Output
`Review: <approve | changes needed>` with a numbered list of findings (file:line, issue, fix).
Findings are fixed or explicitly deferred with the human's consent.
