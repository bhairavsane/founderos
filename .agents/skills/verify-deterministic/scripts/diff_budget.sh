#!/usr/bin/env bash
# Report added lines vs base. Warn when over budget (DIFF_BUDGET, default 300). STRICT=1 makes it fail.
set -uo pipefail
cd "$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
. "$(dirname "$0")/_base.sh"
BASE="${1:-$(agent_base)}"; BUDGET="${DIFF_BUDGET:-300}"
[ -n "$BASE" ] || { echo "  no git base; skipping"; exit 0; }
added=$(git diff --numstat "$BASE" -- . ':!*lock*' ':!*.snap' 2>/dev/null | awk '{a+=$1} END{print a+0}')
files=$(git diff --name-only "$BASE" 2>/dev/null | wc -l | tr -d ' ')
echo "  +$added lines across $files files vs $BASE (budget $BUDGET)"
if [ "$added" -gt "$BUDGET" ]; then
  echo "  WARN over budget: split the task or justify in the report"
  [ "${STRICT:-0}" = "1" ] && exit 1
fi
exit 0
