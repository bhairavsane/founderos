---
name: maintain-evolve
description: Use for ongoing upkeep after release - dependency updates, dead code removal, tech debt, observability, runbooks, incident postmortems, ADR/doc upkeep, and recording agent lessons so the same mistakes are not repeated.
---

# Maintain & Evolve

## Cadence
| Frequency | Task |
|---|---|
| Every PR | `verify.sh` green; `self-review`; docs/ADR touched only if behavior/decision changed |
| Weekly | Dependency PRs (Renovate/Dependabot), one group at a time, each verified; triage flaky tests |
| Monthly | `check_file_size.sh --all` audit + split the worst offenders, dead-code sweep (`knip`/`vulture`), unused deps/flags/env vars, slow-query review |
| Quarterly | Security audit, secret rotation, restore-from-backup drill, review SLOs and alerts, prune ADRs |

## Dependency updates
Patch/minor: batch low-risk, run full gate. Major: one at a time, read the changelog, add a codemod or test first.
Security advisories: patch within SLA (critical 48h, high 7d). Never `--force`/`--legacy-peer-deps` without a note.

## Deleting is a feature
Remove dead code, stale flags (after full rollout + 2 weeks), unused routes, old migrations paths after contract phase.
Prove it is dead with a search + coverage/knip output, then delete in its own commit.

## Tech-debt ledger
`docs/debt.md`: one line each - what, why it hurts, cost to fix, trigger to fix. Pay down when touching the area.
Do not start unrelated refactors inside a feature task.

## Observability
Structured JSON logs with request id; RED metrics (rate, errors, duration) per endpoint; traces for cross-service calls;
alerts on symptoms users feel (error rate, latency, saturation), each linked to a runbook.

## Runbooks & incidents
- Runbook per alert: what it means, how to confirm, how to mitigate, how to roll back, who to page.
- Incident: stabilize -> communicate -> root cause (`debug-root-cause`) -> blameless postmortem (timeline, cause, what worked, actions with owners).
- Every postmortem action becomes a test, a check in `verify.sh`, an alert, or a rule - not just a note.

## Agent learning loop
When the human corrects you or a bug slips through: append ONE imperative line to `docs/agent-lessons.md`
(`Always check X before Y.` / `Never use Z in this repo; use W.`). Keep it < 60 lines; merge duplicates,
delete obsolete lines. Read it at the start of every task (`plan-spec` Step 0).

## Docs hygiene
Docs describe how to run, test, deploy and why key decisions were made. Update in the same PR as the change.
No generated prose summaries of code that the code already shows.
