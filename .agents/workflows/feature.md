---
description: Full lifecycle for a new feature - plan, design, build, test, review (stops before deploy)
---
1. Load `engineering-core`. Read `docs/agent-lessons.md`.
2. Load `plan-spec`: orient, write acceptance criteria and tasks. If large/risky, stop for human approval.
3. If a contract, schema or boundary changes, load `design-architecture` and define it first.
4. For each task: write failing test -> minimal code (`build-minimal-code`, `modular-file-size`) -> `verify.sh --fast`.
5. Run `verify.sh` (full) and paste the summary.
6. Load `self-review`, then `security-baseline` if input/auth/data is touched. Fix findings, re-verify.
7. Give the final report (Changed / Verified / Not verified / Risks). Do NOT deploy.
