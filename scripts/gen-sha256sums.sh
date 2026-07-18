#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUTPUT="${ROOT}/packaging/SHA256SUMS"
TMP="${OUTPUT}.tmp"

if command -v sha256sum >/dev/null 2>&1; then
  hash_file() { sha256sum "$1"; }
elif command -v shasum >/dev/null 2>&1; then
  hash_file() { shasum -a 256 "$1"; }
else
  echo "sha256sum or shasum is required" >&2
  exit 1
fi

: > "${TMP}"
while IFS= read -r file; do
  [[ -f "${ROOT}/${file}" ]] || continue
  (cd "${ROOT}" && hash_file "${file}") >> "${TMP}"
done < <(
  cd "${ROOT}"
  {
    find packaging -type f \( -name '*.md' -o -name '*.txt' -o -name '*.example' \)
    if [[ -d release-evidence ]]; then
      find release-evidence -type f
    fi
  } | grep -vE '(^|/)\.DS_Store$|^packaging/(SHA256SUMS|out/)' | LC_ALL=C sort
)

mv "${TMP}" "${OUTPUT}"
echo "wrote ${OUTPUT}"
