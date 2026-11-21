---
name: debug-root-cause
description: Use when something fails - a test, build, runtime error, flaky behavior or production incident. Enforces reproduce-first, hypothesis-driven root cause analysis and the smallest fix with a regression test, forbidding shotgun edits and retry-until-green.
---

# Debug to Root Cause

## Protocol
1. **Reproduce.** Get a deterministic repro (command, input, or failing test). No repro -> add logging/tracing to get one; do not guess-fix.
2. **Read the evidence.** First error, full stack, exact message, recent changes (`git log -p -5`, `git diff`). Quote it.
3. **Hypothesize.** Max 2 hypotheses at a time, each with a cheap experiment that would falsify it.
4. **Narrow.** Bisect: `git bisect`, binary-search inputs, comment out halves, minimal repro file.
5. **Prove the cause.** You must be able to say "X happens because Y, shown by Z". If not, you are still guessing.
6. **Write the failing regression test** that captures it. Run it: red.
7. **Fix the cause** with the smallest change. Not the symptom (no blanket try/catch, null guard, retry, or sleep).
8. **Run `verify.sh`**: regression test green, nothing else red.
9. **Explain in the report**: cause, fix, why it can't recur. If a human had to correct you, add a line to `docs/agent-lessons.md`.

## Rules
- One change per experiment. Revert experiments that did not help (leave no debris).
- Never "fix" by changing the test, mocking the failure away, widening types, or disabling the check.
- 3 failed fix attempts on the same gate -> stop. Report: what you know, what you ruled out, next hypothesis, what you need.
- Heisenbugs (timing, order, concurrency): suspect shared state, unawaited promises, clocks, and ordering; add determinism before fixing.
- Production incidents: stabilize first (rollback/flag off), then root-cause. Never debug by editing prod.
- Check the boring causes first: wrong env/config, stale build/cache, version mismatch, wrong branch, missing migration.

## Useful probes
Node: `node --inspect`, `DEBUG=*`, `--trace-warnings`, `--unhandled-rejections=strict`.
General: `git bisect run <cmd>`, `strace`/`lsof` for I/O and ports, `curl -v`, `EXPLAIN ANALYZE` for slow queries.
