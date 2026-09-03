#!/usr/bin/env bash

set -uo pipefail

project_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
grading_dir="$project_dir/grading"
logs_dir="$grading_dir/logs"

mkdir -p "$logs_dir"

score_markdown="$grading_dir/score.md"
score_json="$grading_dir/score.json"

printf '%s\n' \
  '# Week 5 automated grading report' \
  '' \
  '| Lab | Rust tests | Execution /4 | Evidence /3 | Automated /7 | Instructor /3 |' \
  '|---:|---:|---:|---:|---:|---:|' > "$score_markdown"

printf '%s\n' '{' '  "labs": [' > "$score_json"

total_execution=0
total_evidence=0
first_json_row=true

for lab_number in $(seq -w 1 10); do
  test_name="lab_${lab_number}"
  log_file="$logs_dir/${test_name}.log"

  (
    cd "$project_dir"
    cargo test --test "$test_name" -- --nocapture
  ) > "$log_file" 2>&1
  test_status=$?

  passed_tests="$(
    sed -nE 's/.*test result: (ok|FAILED)\. ([0-9]+) passed.*/\2/p' "$log_file" |
      tail -n 1
  )"
  passed_tests="${passed_tests:-0}"

  execution_score="$passed_tests"
  if (( execution_score > 4 )); then
    execution_score=4
  fi

  evidence_file="$project_dir/submissions/${test_name}.md"
  evidence_score="$(bash "$project_dir/grader/check_evidence.sh" "$evidence_file")"
  automated_score=$((execution_score + evidence_score))

  total_execution=$((total_execution + execution_score))
  total_evidence=$((total_evidence + evidence_score))

  if (( test_status != 0 )); then
    printf '::warning title=Lab %s tests::%s public test suite is incomplete; see the grading artifact.\n' \
      "$lab_number" "$test_name"
  fi

  printf '| %s | %s/4 | %s | %s | %s | Manual |%s\n' \
    "$lab_number" "$passed_tests" "$execution_score" "$evidence_score" \
    "$automated_score" "" >> "$score_markdown"

  if [[ "$first_json_row" == false ]]; then
    printf '%s\n' ',' >> "$score_json"
  fi
  first_json_row=false

  printf '    {"lab": %d, "tests_passed": %d, "execution": %d, "evidence": %d, "automated": %d, "instructor_explanation": null}' \
    "$((10#$lab_number))" "$passed_tests" "$execution_score" "$evidence_score" \
    "$automated_score" >> "$score_json"
done

automated_total=$((total_execution + total_evidence))

printf '\n%s\n' \
  '  ],' \
  "  \"execution_total\": $total_execution," \
  "  \"evidence_total\": $total_evidence," \
  "  \"automated_total\": $automated_total," \
  '  "automated_maximum": 70,' \
  '  "instructor_explanation_maximum": 30,' \
  '  "final_score": null' \
  '}' >> "$score_json"

printf '\n%s\n' \
  "## Automated total: ${automated_total}/70" \
  '' \
  'The instructor adds 0–3 explanation points per lab after reviewing the written reasoning and practical evidence.' \
  '' \
  'A completed evidence section is not proof that its contents are accurate. Instructors may adjust evidence marks when reviewing the final submission.' \
  >> "$score_markdown"

cat "$score_markdown"

