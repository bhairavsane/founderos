#!/usr/bin/env bash
# Usage: check_file_size.sh [--all]
# Default = RATCHET: only files changed vs base (plus untracked) are checked, so old debt never
# blocks you but nothing you touch may exceed MAX_LINES (250). --all audits the whole repo.
# Exclusions: generated/vendor dirs, lockfiles, snapshots, and globs in .agents/size-ignore.
set -euo pipefail
cd "$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
. "$(dirname "$0")/_base.sh"
SOFT="${SOFT_LINES:-200}"; MAX="${MAX_LINES:-250}"
EXT='\.(ts|tsx|js|jsx|mjs|cjs|py|go|rs|java|kt|swift|rb|php|cs|sql|css|scss|vue|svelte|sh)$'
IGNORE='(^|/)(node_modules|dist|build|coverage|\.next|vendor|migrations|__generated__|generated)/|\.(min|generated)\.|\.d\.ts$|\.snap$'
MODE=changed; [ "${1:-}" = "--all" ] && MODE=all
extra=()
if [ -f .agents/size-ignore ]; then
  while IFS= read -r g; do [[ -z "$g" || "$g" == \#* ]] || extra+=("$g"); done < .agents/size-ignore
fi
skip() { local f="$1" g; for g in "${extra[@]:-}"; do [ -n "$g" ] && [[ "$f" == $g ]] && return 0; done; return 1; }
list() {
  if [ "$MODE" = all ] || ! git rev-parse --verify -q HEAD >/dev/null 2>&1; then
    git ls-files --cached --others --exclude-standard
  else
    { git diff --name-only --diff-filter=d "$(agent_base)"; git ls-files --others --exclude-standard; } | sort -u
  fi
}
fail=0; n_checked=0
while IFS= read -r f; do
  [ -f "$f" ] || continue
  skip "$f" && continue
  n=$(wc -l < "$f"); n_checked=$((n_checked+1))
  if [ "$n" -gt "$MAX" ]; then echo "  FAIL $n lines (max $MAX): $f  -> split by responsibility"; fail=1
  elif [ "$n" -gt "$SOFT" ]; then echo "  WARN $n lines (soft $SOFT): $f"; fi
done < <(list | grep -E "$EXT" | grep -Ev "$IGNORE" || true)
[ "$fail" -eq 0 ] && echo "  size ok: $n_checked file(s) checked [$MODE], max $MAX"
exit "$fail"
