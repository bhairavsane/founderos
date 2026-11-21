---
description: Prepare a release (checklist, rollback plan) and wait for human approval before deploying
---
1. Load `deploy-release`. Run `verify.sh` (full) on the release commit.
2. Fill `resources/release-checklist.md`: changelog, migrations, env vars, rollback command, flags.
3. Present the deploy plan with exact commands. STOP and wait for explicit human approval.
4. After approval and deploy: run the smoke test, report the release line, watch the soak window.
