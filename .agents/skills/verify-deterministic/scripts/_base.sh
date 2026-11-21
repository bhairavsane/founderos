#!/usr/bin/env bash
# Sourced helper: prints the git base to diff against (merge-base with main/master, else HEAD).
agent_base() {
  local b
  for b in origin/main origin/master main master; do
    if git rev-parse --verify -q "$b" >/dev/null; then git merge-base HEAD "$b" 2>/dev/null && return; fi
  done
  git rev-parse --verify -q HEAD || true
}
