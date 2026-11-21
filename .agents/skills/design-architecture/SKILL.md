---
name: design-architecture
description: Use when a task adds or changes a module boundary, API, data model, schema, migration, UI flow or infrastructure. Produces a contract-first design with failure modes, chosen boring technology and a short ADR, before implementation starts.
---

# Design & Architecture

## Goal
Decide the shape once, cheaply, on paper, so the build phase is mechanical and small.

## Principles
- **Contract first.** Define request/response schemas, types, DB columns, events and error
  codes before logic. Prefer a machine-checkable form (zod/TypeBox/OpenAPI/JSON Schema/protobuf)
  so tests and clients derive from one source of truth.
- **Boring technology.** Use what the repo already uses. A new framework/service/library
  needs an ADR and human approval.
- **Smallest surface.** Fewer endpoints, fewer fields, fewer states. Every public thing is a future liability.
- **Separate by reason to change.** Transport (HTTP/CLI) -> service (business rules) ->
  repository (I/O). Business rules never import the HTTP framework or ORM directly.
- **Make illegal states unrepresentable** (enums/unions, NOT NULL, FK, unique constraints).
- **Idempotency and retries** for anything called over a network or queue.

## Procedure
1. List 2 viable options (include "do nothing / reuse existing" when valid). One line each: pros, cons, cost.
2. Pick one. Write why in <= 5 lines.
3. Define the contract (schemas, signatures, DB DDL, error model).
4. List failure modes: invalid input, timeout, partial failure, duplicate request,
   concurrent update, dependency down, bad deploy. State the behavior for each.
5. Set budgets: latency, memory, query count, payload size.
6. Sketch the file layout; each planned file must fit < 200 lines (see `modular-file-size`).
7. Threat sketch (STRIDE-lite): who can call this, what can they send, what must never leak? Then load `security-baseline`.
8. Write an ADR (`resources/adr-template.md`) for decisions that are hard to reverse. Max 40 lines.

## Data & migrations
- Expand -> migrate -> contract: additive change first, backfill, switch reads, remove old later.
- Every migration has a tested down/rollback path or an explicit "forward-only" note.
- Never rename/drop a column in the same release that stops using it.

## UI work
- Reuse existing components and design tokens; do not create new primitives without need.
- Specify all states: loading, empty, error, partial, success, offline. Keyboard + screen-reader behavior.
- Verify visually via the browser agent AND with deterministic assertions (see `test-strategy`).

## Exit gate
Contract exists; failure modes have defined behavior; planned files each < 200 lines;
no new technology without approval. Then hand off to `build-minimal-code`.
