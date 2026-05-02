#!/bin/bash
INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

if [[ "$FILE_PATH" != *.rs ]]; then
  exit 0
fi

if ! command -v rustfmt >/dev/null 2>&1; then
  exit 0
fi

rustfmt --edition 2021 "$FILE_PATH" >/dev/null 2>&1
exit 0
