---
description: Run the deterministic gate and report evidence only
---
1. Run `bash .agents/skills/verify-deterministic/scripts/verify.sh`.
2. Paste the summary table and exit code verbatim. Exit 3 (UNVERIFIED) is not done: add the missing gate or ask the human. List every SKIP under "Not verified".
3. On failure: fix root cause (max 3 attempts per gate), re-run, never edit the gate.
