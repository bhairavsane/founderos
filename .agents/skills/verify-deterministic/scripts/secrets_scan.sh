#!/usr/bin/env bash
# Fail if likely secrets are present in tracked/untracked files. Uses gitleaks if installed.
set -uo pipefail
cd "$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
if command -v gitleaks >/dev/null 2>&1; then
  gitleaks detect --no-banner --redact -s . && echo "  gitleaks clean" ; exit $?
fi
PAT='AKIA[0-9A-Z]{16}|AIza[0-9A-Za-z_-]{35}|-----BEGIN ((RSA|EC|DSA|OPENSSH|PGP) )?PRIVATE KEY-----|xox[baprs]-[0-9A-Za-z-]{10,}|gh[pousr]_[0-9A-Za-z]{36}|sk-[A-Za-z0-9]{32,}|(api[_-]?key|secret|token|passw(or)?d)["'"'"' ]*[:=]["'"'"' ]*[A-Za-z0-9/+_=-]{16,}'
hits=$(git ls-files --cached --others --exclude-standard 2>/dev/null \
  | grep -Ev '(^|/)(node_modules|dist|build|vendor)/|lock|\.snap$|\.(png|jpg|jpeg|gif|ico|pdf|woff2?)$|secrets_scan\.sh$|\.example$|\.md$' \
  | xargs -r grep -InEi -- "$PAT" 2>/dev/null | sed -E 's/(.{80}).*/\1.../' || true)
envs=$(git ls-files 2>/dev/null | grep -E '(^|/)\.env($|\.[^e]|\.e[^x])' | grep -Ev '\.example$' || true)
if [ -n "$hits$envs" ]; then
  echo "  FAIL possible secrets:"; [ -n "$hits" ] && echo "$hits" | sed 's/^/    /'
  [ -n "$envs" ] && echo "$envs" | sed 's/^/    tracked env file: /'; exit 1
fi
echo "  no secrets found (regex scan; install gitleaks for deeper coverage)"
