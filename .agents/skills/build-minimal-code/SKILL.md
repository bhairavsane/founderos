---
name: build-minimal-code
description: Use while writing or editing code. Prevents AI slop and over-generation - enforces smallest diff, reuse-before-write, no speculative abstractions, no narration comments, incremental writing one small file at a time with verification after each step.
---

# Build Minimal Code

## Goal
Write only the code the acceptance criteria require. Nothing more.

## Before writing (30-second gate)
1. Which AC does this change satisfy? If none, do not write it.
2. Did I search the repo for an existing function/type/component that does this? Reuse it.
3. Does the standard library or an already-installed dependency cover it? Use it.
4. Have I read the real signature/type of every API I'm about to call? (Never guess.)
5. What is the smallest edit? Prefer editing 3 lines over adding a file.

## Write incrementally
- Build **vertical slices** (one working path end to end), never ten empty scaffolded files.
- One file or one function at a time. After each: typecheck/lint/test that unit (`verify.sh --fast`).
- Order: types/contract -> failing test -> implementation -> wire-up. Never dump a whole feature in one response.
- Edit in place with targeted replacements. Do not regenerate whole files to change a few lines.
- Keep the repo runnable after every step.

## Rule of Three
First use: write it inline. Second: copy-tolerate it, note it. Third: extract, with a name that fits all three uses.
Never create a factory, base class, generic wrapper, plugin point or options object for one caller.

## Slop catalog - do NOT produce
- Narration comments (`// loop through users`), docstrings that restate the signature.
- Defensive code for impossible states; `try/catch` that logs and swallows; catch-all `any`.
- Wrapper/helper used once; class where a function works; interface with one implementation.
- Unused parameters, exports, imports, variables, feature flags, env vars, options objects.
- Premature generics, plugin systems, factories, "manager"/"helper"/"utils" dumping grounds.
- Duplicate types or constants that already exist elsewhere.
- Placeholder `TODO`, fake/mock data, hardcoded URLs/keys, `console.log` leftovers.
- Extra README, example, demo or markdown files nobody asked for.
- Reformatting, import reordering or renames in lines you didn't need to touch.
- Re-implementing validation/auth/logging that the project already centralizes.

## Style rules
- Names carry meaning (`expiresAt`, not `data2`). Booleans read as questions (`isExpired`).
- Early return over nesting; nesting depth <= 3; function <= 40 lines; params <= 4 (else an object).
- Errors: throw/return typed, specific errors at the boundary where they can be handled; include context, never secrets.
- Comments only for *why* (constraint, trade-off, link to ticket/spec). Max a few per file.
- Pure functions for logic; I/O at the edges. Inject time/random/network so tests are deterministic.
- Follow the existing codebase's patterns even if you'd prefer another. Consistency beats taste.

## Dependencies
Adding one requires: why not stdlib/existing, maintenance health, license, size, `npm audit`/equivalent clean.
State it in one line in the plan. Pin versions. No dependency for something under ~20 lines.

## After writing
Run `bash .agents/skills/verify-deterministic/scripts/verify.sh --fast`, then do the
`self-review` diff pass: delete every line you cannot map to an AC.
