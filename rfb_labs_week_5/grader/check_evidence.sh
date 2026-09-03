#!/usr/bin/env bash

set -u

evidence_file="${1:-}"

if [[ -z "$evidence_file" || ! -f "$evidence_file" ]]; then
  echo 0
  exit 0
fi

section_content() {
  local heading="$1"
  awk -v heading="$heading" '
    $0 == heading { inside = 1; next }
    inside && /^## / { exit }
    inside { print }
  ' "$evidence_file"
}

section_is_complete() {
  local heading="$1"
  local content
  content="$(section_content "$heading")"
  content="$(printf '%s' "$content" | sed '/^[[:space:]]*$/d; /<!--.*-->/d')"

  if [[ ${#content} -lt 20 ]]; then
    return 1
  fi

  if printf '%s' "$content" | grep -Eqi 'TODO|replace this|not completed|write here'; then
    return 1
  fi

  return 0
}

score=0

for heading in "## Commands used" "## Terminal output" "## Evidence references"; do
  if section_is_complete "$heading"; then
    score=$((score + 1))
  fi
done

echo "$score"

