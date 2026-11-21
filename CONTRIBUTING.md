# Contributing to FounderOS

Thank you for your interest in contributing to **FounderOS**! We are building the private, 100% offline AI operating system for founders.

To keep the codebase maintainable, secure, and blazing fast, all contributors must adhere to our **Engineering Constitution**.

---

## The Engineering Constitution

1. **Minimal Diff**: Solve exactly what was requested. No speculative features, premature abstractions, shims, or drive-by refactorings.
2. **Search Before Creating**: Reuse existing utilities, domain models, and storage repositories. Delete dead code; never comment it out.
3. **Rule of Three**: Do not introduce a generic abstraction, wrapper, or trait until the 3rd real call site.
4. **Small Files**:
   - Aim for $\le 200$ lines per file (hard cap of $250$ lines).
   - Functions must be $\le 40$ lines with single responsibilities.
   - Split by responsibility, never by `part1`/`part2`.
5. **No Phantom Dependencies or APIs**:
   - Avoid adding new third-party crates or npm packages unless strictly necessary.
   - Always check real compiler signatures and documentation; never guess.
6. **No Verification Cheats**:
   - Never use `any`, `@ts-ignore`, `eslint-disable`, `# noqa`, `.skip()`, or `.only()`.
   - Never weaken assertions or test thresholds to bypass checks.
7. **Proof, Not Claims**:
   - Every PR must pass the deterministic verification suite before merging:
     ```bash
     bash .agents/skills/verify-deterministic/scripts/verify.sh
     ```
     Exit code `0` is required.
8. **100% Offline & Privacy First**:
   - FounderOS is an air-gapped system. Any PR introducing unapproved cloud telemetry, external API calls, or leaky logging will be rejected immediately.

---

## Development & PR Workflow

### 1. Branch Protection & Naming
The `main` branch is strictly protected:
- Direct pushes and deletions are blocked by GitHub rulesets.
- All contributions must arrive via Pull Requests.

Branch naming conventions:
- Features: `feat/your-feature-name` or `wip-your-feature`
- Bug fixes: `fix/issue-description`
- Docs/Chore: `docs/topic-name`

### 2. Running Verification Locally
Before opening a PR, ensure all tests, lint checks, file size constraints, and builds pass:

```bash
# Run unit and integration tests across all Cargo crates
cargo test --workspace

# Build the desktop frontend
cd apps/desktop && npm run build && cd ../..

# Run full deterministic verification gate
bash .agents/skills/verify-deterministic/scripts/verify.sh
```

### 3. Submitting a Pull Request
1. Fork the repository and create your feature branch.
2. Commit your changes with clear, semantic commit messages (e.g. `feat(pipeline): add docx table parser`).
3. Open a Pull Request into `main`.
4. Include the output summary from `verify.sh` in your PR description.
5. All CI matrix checks (macOS, Windows, Linux) must turn green before merge.
