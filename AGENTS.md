# Engineering Constitution (always on - keep this file short)

1. **Minimal diff.** Do only what was asked. No speculative features, fallbacks, shims or drive-by refactors.
2. **Search before creating.** Reuse existing utilities, types and patterns. Delete dead code; never comment it out.
3. **Rule of Three.** No abstraction, wrapper or generic helper until the third real use.
4. **Small files.** Aim <= 200 lines, hard cap 250 per file; functions <= 40. Split by responsibility, never `part1/part2`.
5. **No phantom dependencies or APIs.** Add packages only with approval; check real signatures/versions, never guess.
6. **No verification cheats.** Never use `@ts-ignore`, `any`, `eslint-disable`, `# noqa`, `.skip`/`.only`, or weaker assertions.
7. **Proof, not claims.** "Done" = `bash .agents/skills/verify-deterministic/scripts/verify.sh` exit 0, summary pasted.
   Exit 3 (UNVERIFIED) is not done. Re-run after the last edit.
8. **Ask first** (stop and wait) if a change touches > 2 files, alters a schema/API contract, adds a dependency,
   or touches auth/billing/infra. Never deploy or run destructive commands without explicit approval.
9. **One feature per thread.** At the end of each phase write a <= 10 line state summary; start fresh threads for new features.
10. **Learn.** When the human corrects you, add one line to `docs/agent-lessons.md`.

Start every coding task by loading the `engineering-core` skill.
