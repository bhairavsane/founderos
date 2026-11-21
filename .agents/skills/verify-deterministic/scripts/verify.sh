#!/usr/bin/env bash
# Deterministic gate. Usage: verify.sh [--fast]   (fast = skip build + audit)
# Exit codes: 0 = all gates passed AND all REQUIRE'd gates ran | 1 = a gate failed | 3 = UNVERIFIED
# (a required gate such as `test` did not run, so "pass" would be a lie).
# Env: REQUIRE="test"  (comma list; e.g. "lint,typecheck,test")   SKIP_AUDIT=1   STRICT=1 (diff budget fails)
set -uo pipefail
cd "$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
HERE="$(cd "$(dirname "$0")" && pwd)"
FAST=0; [ "${1:-}" = "--fast" ] && FAST=1
REQUIRE="${REQUIRE:-test}"
RESULTS=(); FAILED=0
rec() { RESULTS+=("$1 $2"); [ "$1" = "FAIL" ] && FAILED=1; return 0; }
gate() { local name="$1"; shift; echo "▶ $name"; if "$@"; then rec PASS "$name"; else rec FAIL "$name"; fi; }
skip() { echo "▶ $1 (skipped: $2)"; rec SKIP "$1 ($2)"; }

PM=""; if [ -f package.json ]; then
  if [ -f pnpm-lock.yaml ]; then PM=pnpm; elif [ -f yarn.lock ]; then PM=yarn; else PM=npm; fi
fi
has_script() { [ -f package.json ] && node -e "process.exit(((require('./package.json').scripts)||{})['$1']?0:1)" 2>/dev/null; }
npm_run() { "$PM" run --silent "$1"; }
detected=0

gate file-size  bash "$HERE/check_file_size.sh"
gate no-bypass  bash "$HERE/check_bypass.sh"
gate secrets    bash "$HERE/secrets_scan.sh"

if [ -n "$PM" ]; then detected=1
  [ -d node_modules ] || { echo "▶ install"; "$PM" install --frozen-lockfile 2>/dev/null || "$PM" install; }
  if has_script lint; then gate lint npm_run lint; else skip lint "no script"; fi
  if has_script typecheck; then gate typecheck npm_run typecheck
  elif [ -f tsconfig.json ]; then gate typecheck npx --no-install tsc --noEmit
  else skip typecheck "no tsconfig"; fi
  if has_script test; then gate test env CI=1 "$PM" run --silent test; else skip test "no test script"; fi
  if [ $FAST -eq 0 ]; then
    if has_script build; then gate build npm_run build; else skip build "no script"; fi
    if [ "${SKIP_AUDIT:-0}" = "1" ]; then skip audit "SKIP_AUDIT=1"; else gate audit "$PM" audit --audit-level=high; fi
  fi
fi

if [ -f pyproject.toml ] || [ -f requirements.txt ]; then detected=1
  if command -v ruff >/dev/null; then gate lint ruff check .; else skip lint "ruff not installed"; fi
  if command -v mypy >/dev/null && { [ -f mypy.ini ] || grep -qs '\[tool.mypy\]' pyproject.toml; }; then gate typecheck mypy .; else skip typecheck "mypy not configured"; fi
  if command -v pytest >/dev/null; then gate test pytest -q; else skip test "pytest not installed"; fi
fi

if [ -f go.mod ]; then detected=1
  gate lint go vet ./...
  gate test go test ./...
  [ $FAST -eq 0 ] && gate build go build ./...
fi

[ $detected -eq 0 ] && skip project "no supported stack - add checks to verify.sh"
gate diff-budget bash "$HERE/diff_budget.sh"

missing=()
for r in ${REQUIRE//,/ }; do
  printf '%s\n' "${RESULTS[@]}" | grep -Eq "^(PASS|FAIL) $r$" || missing+=("$r")
done

echo; echo "======== VERIFY SUMMARY ($([ $FAST -eq 1 ] && echo fast || echo full)) ========"
printf '%s\n' "${RESULTS[@]}" | sed 's/^/  /'
if [ $FAILED -ne 0 ]; then echo "RESULT: FAILED (exit 1)"; exit 1; fi
if [ ${#missing[@]} -gt 0 ]; then
  echo "RESULT: UNVERIFIED (exit 3) - required gate(s) did not run: ${missing[*]}"
  echo "        Add them to the project, or the human may set REQUIRE to a smaller list."
  exit 3
fi
echo "RESULT: PASSED (exit 0)"; exit 0
