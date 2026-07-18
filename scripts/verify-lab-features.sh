#!/usr/bin/env bash
# Feature → verification matrix for Lab Core Perfect.
# Runs automated flows the agent/CI can execute without interactive GUI pickers.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "${ROOT}"

OUT_DIR="${ROOT}/packaging/out"
mkdir -p "${OUT_DIR}"
REPORT="${OUT_DIR}/feature-verify-$(date -u +%Y%m%dT%H%M%SZ).tsv"
SUMMARY="${OUT_DIR}/feature-verify-latest.tsv"
: >"${REPORT}"

pass=0
fail=0
skip=0

run() {
  local id="$1"
  local feature="$2"
  local flow="$3"
  shift 3
  printf '[%s] %s … ' "${id}" "${feature}"
  local log
  log="$(mktemp)"
  if "$@" >"${log}" 2>&1; then
    echo "PASS"
    printf '%s\tPASS\t%s\t%s\n' "${id}" "${feature}" "${flow}" >>"${REPORT}"
    pass=$((pass + 1))
  else
    echo "FAIL"
    echo "---- ${id} log (tail) ----" >&2
    tail -n 40 "${log}" >&2 || true
    printf '%s\tFAIL\t%s\t%s\n' "${id}" "${feature}" "${flow}" >>"${REPORT}"
    fail=$((fail + 1))
  fi
  rm -f "${log}"
}

note_skip() {
  local id="$1"
  local feature="$2"
  local flow="$3"
  local why="$4"
  printf '[%s] %s … SKIP (%s)\n' "${id}" "${feature}" "${why}"
  printf '%s\tSKIP\t%s\t%s\n' "${id}" "${feature}" "${flow}" >>"${REPORT}"
  skip=$((skip + 1))
}

echo "=== Lab feature verification $(date -u +%Y-%m-%dT%H:%M:%SZ) ==="
echo "cwd=${ROOT}"
echo

# --- Honesty / Perfect gate ---
run F01 "Honesty: demo_seed OFF + NOT court-ready + local_ai off" \
  "static gate script" \
  bash scripts/check-lab-core-perfect-gate.sh

run F02 "Docs gate FR-DOC-002" \
  "offline docs present for validated capabilities" \
  bash scripts/check-fr-doc-002.sh

# --- Case / ledger / lock ---
run F03 "CaseDb ledgers + bookmarks/findings" \
  "cargo test -p lab-case" \
  cargo test -p lab-case --tests

run F04 "Case lock acquire/release" \
  "cargo test -p lab-case --test case_lock" \
  cargo test -p lab-case --test case_lock

# --- LabSession end-to-end paths ---
run F05 "LabSession: import + hex counts" \
  "session_import_and_counts" \
  cargo test -p lab-slint --features gui --lib -- session_import_and_counts

run F06 "LabSession: SoD blocks self-approve" \
  "sod_blocks_self_approve" \
  cargo test -p lab-slint --features gui --lib -- sod_blocks_self_approve

run F07 "LabSession: transfer sign + tamper Invalid" \
  "transfer_tamper_invalid" \
  cargo test -p lab-slint --features gui --lib -- transfer_tamper_invalid

run F08 "LabSession: parser failure → coverage failed" \
  "parser_failure_is_coverage_failure_with_reason" \
  cargo test -p lab-slint --features gui --lib -- parser_failure_is_coverage_failure_with_reason

run F09 "LabSession: append-only notes + audit" \
  "notes_are_append_only_and_ordered" \
  cargo test -p lab-slint --features gui --lib -- notes_are_append_only_and_ordered

run F10 "LabSession: discrepancy + backup/restore" \
  "discrepancies_persist_and_backup_restores_case" \
  cargo test -p lab-slint --features gui --lib -- discrepancies_persist_and_backup_restores_case

run F11 "Open case → real evidence/coverage counts" \
  "case_open_counts integration" \
  cargo test -p lab-slint --features gui --test case_open_counts

# --- Storage / intake ---
run F12 "Raw import + hash" \
  "lab-storage raw/import/hash" \
  cargo test -p lab-storage --test import_raw --test raw_hash --test progress_hash --test cancel_hash

run F13 "E01 Validated subset + hostile fail-closed" \
  "e01_roundtrip + e01_hostile" \
  cargo test -p lab-storage --test e01_roundtrip --test e01_hostile

run F14 "Unsupported formats fail closed" \
  "unsupported_formats" \
  cargo test -p lab-storage --test unsupported_formats --test hostile_raw

run F15 "fsnap hostile / preflight" \
  "hostile_fsnap" \
  cargo test -p lab-fsnap --tests

# --- Filesystems ---
run F16 "APFS Validated subset" \
  "apfs_real" \
  cargo test -p lab-fs --test apfs_real

run F17 "NTFS / FAT / ext4 golden parsers" \
  "ntfs + fat_ext4 + enumerate" \
  cargo test -p lab-fs --test ntfs_real --test fat_ext4_real --test ntfs_enumerate --test fat_enumerate --test unix_deleted --test content_cas

# --- Index / search ---
run F18 "Search operators + coverage disclosure" \
  "search_operators" \
  cargo test -p lab-index --test search_operators

run F19 "Index persist 10k + window paging" \
  "persist_10k + index_fs_meta + windowed_1m" \
  cargo test -p lab-index --test persist_10k --test index_fs_meta --test windowed_1m

# --- YARA / hashset ---
run F20 "YARA-X production scan (Validated)" \
  "lab-yara golden + scan" \
  cargo test -p lab-yara --tests

run F21 "Hash-set lookup + DB pin" \
  "lab-hashset lookup" \
  cargo test -p lab-hashset --tests

# --- Trust / report / export ---
run F22 "Integrity / ScopeGuard / RunManifest / report gate" \
  "lab-core lib + export tests" \
  cargo test -p lab-core --lib --tests

run F23 "Export PDF/A + CASE/UCO + HTML skeleton" \
  "export_formats + export_skeleton" \
  cargo test -p lab-core --test export_formats --test export_skeleton

run F24 "Contract schema fixtures (CASE/evidence/bookmark/transfer)" \
  "schema_fixtures + bookmark_schema" \
  cargo test -p lab-core --test schema_fixtures --test bookmark_schema

run F25 "Transfer Ed25519 package sign/verify" \
  "transfer_sign" \
  cargo test -p lab-transfer --tests

run F26 "Crypto envelope trust states" \
  "envelope_trust" \
  cargo test -p lab-crypto --tests

# --- Artifacts / timeline / carve / antiforensics ---
run F27 "Artifact parsers (registry/evtx/browser)" \
  "lab-artifacts" \
  cargo test -p lab-artifacts --tests

run F28 "Timeline crate" \
  "lab-timeline lib" \
  cargo test -p lab-timeline --lib

run F29 "Carving + antiforensics detect" \
  "lab-carving + lab-antiforensics" \
  bash -c 'cargo test -p lab-carving --tests && cargo test -p lab-antiforensics --tests'

run F30 "Worker cancel/resume" \
  "lab-worker" \
  cargo test -p lab-worker --tests

run F31 "CAS store roundtrip" \
  "lab-store" \
  cargo test -p lab-store --tests

run F32 "Second-method / blind PT hooks" \
  "validation_hooks crate + UI" \
  bash -c 'cargo test -p lab-case --test validation_hooks && cargo test -p lab-slint --features gui --test validation_hooks_ui'

# --- UI shell (non-interactive) ---
run F33 "UI smoke: nav, intake, AI defaults honest" \
  "ui_smoke" \
  cargo test -p lab-slint --features gui --test ui_smoke

run F34 "UI: file list, shortcuts, palette, report gate" \
  "file_list_keyboard" \
  cargo test -p lab-slint --features gui --test file_list_keyboard

run F35 "UI: workbench chrome toggles" \
  "workbench_chrome" \
  cargo test -p lab-slint --features gui --test workbench_chrome

run F35b "UI: DFIR workbench UX (palette/exceptions/timeline import)" \
  "workbench_ux" \
  cargo test -p lab-slint --features gui --test workbench_ux

run F36 "UI: artifacts/timeline/findings/report/transfer model" \
  "artifacts_ui" \
  cargo test -p lab-slint --features gui --test artifacts_ui

run F37 "UI: progress + prefs + offline docs shell" \
  "progress_ui + prefs_ui + docs_shell" \
  cargo test -p lab-slint --features gui --test progress_ui --test prefs_ui --test docs_shell

# --- Packaging / dist ---
run F38 "Packaging smoke (unsigned build artifact)" \
  "packaging/smoke.sh" \
  bash packaging/smoke.sh

run F39 "SHA256SUMS present + gen script" \
  "test -s packaging/SHA256SUMS" \
  test -s packaging/SHA256SUMS

run F40 "Full workspace test (exclude duplicate lab-slint gui already covered)" \
  "cargo test --workspace --exclude lab-slint" \
  cargo test --workspace --exclude lab-slint

# Interactive GUI pickers cannot be driven headlessly here.
note_skip F41 "rfd Open Case / Import Evidence pickers" \
  "manual: launch lab-slint gui and use Open/Import dialogs" \
  "needs interactive display + rfd"

note_skip F42 "Full Slint visual walkthrough (screenshots)" \
  "manual: Case→Evidence→Search→Timeline→Report chrome" \
  "visual QA not automated"

cp "${REPORT}" "${SUMMARY}"
echo
echo "=== Summary ==="
echo "PASS=${pass} FAIL=${fail} SKIP=${skip}"
echo "Report: ${REPORT}"
echo "Latest: ${SUMMARY}"
if [[ "${fail}" -ne 0 ]]; then
  exit 1
fi
