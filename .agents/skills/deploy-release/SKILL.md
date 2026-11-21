---
name: deploy-release
description: Use when preparing, shipping or rolling back a release to staging or production. Covers pre-deploy gates, CI, containerization, backward-compatible migrations, staged rollout, smoke tests and a rollback plan. Never deploys without explicit human approval.
---

# Deploy & Release

## Hard rules
- **Human approval required** before any deploy to a shared/production environment, any
  migration against real data, any infra change, DNS/secret changes, or destructive command
  (`rm -rf`, `DROP`, `terraform destroy`, force-push, `kubectl delete`). Present the exact command and effect; wait.
- Agents prepare releases; humans press the button. Never use prod credentials from the agent session.
- Build once, promote the same artifact: dev -> staging -> prod. Never rebuild per environment.

## Pre-deploy checklist (all must be true)
1. `verify.sh` (full) exit 0 on the exact commit being released; CI green on the same script.
2. Version bumped; changelog entry from conventional commits (`feat:`, `fix:`, `BREAKING CHANGE:`).
3. Migrations are backward compatible with the *previous* app version (expand -> migrate -> contract), tested on a copy of realistic data, with a rollback or forward-only note.
4. Config via env/secret manager; new variables documented in `.env.example` and set in the target environment BEFORE deploy.
5. Health (`/healthz` liveness) and readiness (`/readyz`, checks dependencies) endpoints exist.
6. Rollback plan written in the release note: previous artifact id, exact rollback command, data implications.
7. Observability ready: structured logs, error tracking, key metrics, an alert for the new failure modes.
8. Risky behavior is behind a feature flag defaulting off.

## Containers (if used)
Multi-stage build, pinned base image digest/tag, non-root user, no secrets in layers, `.dockerignore`,
read-only filesystem where possible, healthcheck, small final image, one process per container.

## CI/CD
Pipeline stages: install (frozen lockfile) -> `verify.sh` -> build -> image scan -> publish artifact ->
deploy staging -> smoke test -> manual approval -> canary/prod -> smoke test.
Template: `resources/ci.yml`. Release checklist: `resources/release-checklist.md`.

## Rollout
Prefer progressive: canary 5% -> 25% -> 100%, or blue/green. Watch error rate, latency p95, saturation for
a defined soak window before promoting. Automatic rollback on SLO breach if the platform supports it.

## Smoke test (deterministic, run after every deploy)
A small script hitting real endpoints: health 200, one authenticated read, one write path on a test tenant,
expected version string. Exit non-zero on any mismatch. Keep it < 60 lines.

## Rollback
Redeploy the previous artifact (do not "hotfix forward" under pressure unless rollback is unsafe).
If a migration ran, follow its documented path. Then write a short blameless postmortem (`maintain-evolve`).

## Report format
`Release X.Y.Z | commit | artifact id | env | approvals | migrations | flags | smoke: PASS | rollback: <cmd>`
