#!/usr/bin/env bash
set -euo pipefail

GRAMMAR_DIR="/Users/f00lg/github/c3/c3-zed/grammars/c3"
HIGHLIGHTS_DIR="queries/highlights.scm"
HIGHLIGHTS_DIR="/Users/f00lg/github/c3/c3-zed/languages/c3/highlights.scm"

INPUT_FILE="${1:-/Users/f00lg/github/c3/c3-zed/test.c3}"
OUT_DIR="${2:-/Users/f00lg/github/c3/c3-zed}"
RUN_TESTS="${RUN_TESTS:-0}"

BASENAME="$(basename "${INPUT_FILE%.*}")"
HIGHLIGHT_OUT="${OUT_DIR}/${BASENAME}.highlights.txt"
TREE_OUT="${OUT_DIR}/${BASENAME}.tree.txt"

if [[ ! -f "$INPUT_FILE" ]]; then
  echo "Input file not found: $INPUT_FILE" >&2
  exit 1
fi

if [[ ! -d "$GRAMMAR_DIR" ]]; then
  echo "Grammar dir not found: $GRAMMAR_DIR" >&2
  exit 1
fi

mkdir -p "$OUT_DIR"

cd "$GRAMMAR_DIR"

TS_CLI=(npx -y tree-sitter-cli@0.26.5)

"${TS_CLI[@]}" generate

if [[ "$RUN_TESTS" == "1" ]]; then
  if ! "${TS_CLI[@]}" test; then
    echo "Warning: tree-sitter tests failed; continuing to produce output files." >&2
  fi
fi

"${TS_CLI[@]}" query -p "$GRAMMAR_DIR" "$HIGHLIGHTS_DIR" "$INPUT_FILE" > "$HIGHLIGHT_OUT"
"${TS_CLI[@]}" parse -p "$GRAMMAR_DIR" "$INPUT_FILE" > "$TREE_OUT"

echo "Wrote:"
echo "  $HIGHLIGHT_OUT"
echo "  $TREE_OUT"
