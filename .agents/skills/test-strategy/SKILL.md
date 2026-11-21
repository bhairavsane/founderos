---
name: test-strategy
description: Use when adding behavior, fixing a bug, or deciding what to test. Defines red-green test-first workflow, deterministic test rules (fixed clock, seeded random, no network), the right test level for each change, and how to prove tests actually catch regressions.
---

# Test Strategy

## Goal
Tests are the executable form of the acceptance criteria. They must fail for the right reason first,
pass for the right reason after, and never flake.

## Workflow (red -> green -> refactor)
1. Write the test from an AC (`AC2: expired token returns 401`). Name it after the behavior.
2. Run it. It MUST fail, and fail with the expected message. A test that never failed proves nothing.
3. Write the minimal code to pass. Run again.
4. Refactor only with green tests. Run `verify.sh --fast`.
For bug fixes: reproduce with a failing test BEFORE touching the fix (see `debug-root-cause`).

## Pick the cheapest level that proves it
| Change | Level | Tooling examples |
|---|---|---|
| Pure logic, parsing, validation | Unit (+ property tests for parsers/serializers) | vitest/jest/pytest, fast-check/hypothesis |
| Service + DB/queue behavior | Integration with real ephemeral dependency | testcontainers, sqlite/pg in docker |
| HTTP API contract | Contract/API test against schema | supertest, schemathesis, oasdiff |
| Critical user flow (login, checkout, interview start) | E2E, few and stable | Playwright |
| UI states | Component test + one visual check | Testing Library, Playwright screenshots |
Keep ~70% unit / 20% integration / 10% E2E. Do not mock what you own; mock only network and time.

## Determinism rules
- Inject clock and RNG; use fake timers or a fixed `now`. Seed random. No `sleep`; await conditions.
- No real network, no shared global state, no test-order dependence, each test creates its own data.
- Unique temp dirs/ports/DB schemas per test run. Clean up in `afterEach`.
- Assert on outcomes (return values, DB rows, HTTP status/body), not on internals or call counts, unless the call IS the behavior.
- Snapshot tests only for small, stable output. Never snapshot a giant object to "have coverage".

## Coverage the right way
- Cover each AC + each failure mode from the design + boundaries (0, 1, max, empty, null, unicode, duplicate).
- Measure coverage of **changed lines**, not a global percentage. Never write tests just to move a number.
- Security-relevant paths (authz denied, invalid input, rate limit) always have a negative test.

## Prove the tests are real
- Temporarily break the implementation (flip a condition); the test must fail. Revert.
- On core logic, run mutation testing (Stryker/mutmut) and kill surviving mutants that matter.

## UI verification with the browser agent
The browser agent is great for exploration and screenshots, but **not** a deterministic gate.
After exploring, encode what you saw as a Playwright test with explicit assertions
(role/text/URL/network response), then run it headless in `verify.sh`.

## Forbidden
`it.skip`, `xit`, `.only`, `@pytest.mark.skip` without a linked issue; deleting/loosening assertions;
`expect(true).toBe(true)`; catching errors inside tests to make them pass; retry loops to hide flakiness;
testing private functions through hacks.
