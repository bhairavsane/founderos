#!/usr/bin/env bash
# Scans ADDED lines (diff vs base + untracked files) for check-bypass markers and debug leftovers.
# FAIL: type/lint suppressions, `any`, skipped/focused tests.  WARN: console.log, debugger, TODO/FIXME.
set -uo pipefail
cd "$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
. "$(dirname "$0")/_base.sh"
CODE='\.(ts|tsx|js|jsx|mjs|cjs|py|go|rs|java|kt|rb|php|cs|vue|svelte|sh):'
FAIL_PAT='@ts-ignore|@ts-nocheck|eslint-disable|#[[:space:]]*type:[[:space:]]*ignore|#[[:space:]]*noqa|//[[:space:]]*nolint|:[[:space:]]*any([^A-Za-z0-9_]|$)|[[:space:]]as[[:space:]]+any([^A-Za-z0-9_]|$)|\b(it|test|describe)\.(skip|only)\(|\bx(it|describe)\(|pytest\.mark\.skip|@Disabled|t\.Skip\('
WARN_PAT='console\.(log|debug)\(|\bdebugger\b|TODO|FIXME'
added() {
  if git rev-parse --verify -q HEAD >/dev/null 2>&1; then
    git diff -U0 --no-color "$(agent_base)" -- . ':!*.md' ':!.agents/*' ':!*lock*' | awk '
      /^\+\+\+ b\//{f=substr($0,7);next}
      /^@@/{split($3,a,","); n=substr(a[1],2)+0; next}
      /^\+/{print f":"n":"substr($0,2); n++}'
  fi
  git ls-files --others --exclude-standard | grep -Ev '\.md$|^\.agents/|lock' | while IFS= read -r u; do
    [ -f "$u" ] && awk -v f="$u" '{print f":"NR":"$0}' "$u"
  done
}
lines=$(added | grep -E "$CODE" || true)
bad=$(printf '%s\n' "$lines" | grep -E -- "$FAIL_PAT" || true)
warn=$(printf '%s\n' "$lines" | grep -E -- "$WARN_PAT" || true)
[ -n "$warn" ] && { echo "  WARN debug/TODO leftovers:"; echo "$warn" | head -10 | cut -c1-140 | sed 's/^/    /'; }
if [ -n "$bad" ]; then
  echo "  FAIL bypass markers in added lines (fix the cause, do not silence the check):"
  echo "$bad" | head -20 | cut -c1-140 | sed 's/^/    /'; exit 1
fi
echo "  no bypass markers in added lines"
