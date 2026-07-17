# Storefront Binary Distribution Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Realign Trareon Lab docs and release scripts so v1 sells as a full binary via Lynk.id/Gumroad, with public GPL source on GitHub and **no** product binaries published to GitHub.

**Architecture:** Documentation and script contracts change only—no in-app license/DRM code is added or required. Selling path = local `dist/` build + checksums + storefront upload. Official “O12 / publish” means storefront publish + optional **source-only** git tag. Signing remains optional hardening.

**Tech Stack:** Markdown docs, bash packaging/release scripts, existing `packaging/build-*-unsigned.sh`, `.gitignore` `dist/`

**Spec:** `docs/superpowers/specs/2026-07-17-storefront-binary-distribution-design.md`

## Global Constraints

- Source public on GitHub under GPL-3.0-only; binaries private (operator disk + storefront only).
- No in-app license key, activation, seat count, phone-home, or DRM.
- Do not attach product installers to GitHub Releases or public CI artifacts.
- Signing/notarization/Authenticode are optional, not sell blockers.
- Windows lab work stays queued at end for optional hardening only.
- Honesty required on selling page: unsigned warnings, GPL, non-accreditation.
- Sold binary must publish matching freeze SHA + SHA-256 for GPL correspondence.

---

## File map

| File | Responsibility |
|---|---|
| `docs/DISTRIBUTION-STOREFRONT.md` | Canonical operator path: build → hash → upload Lynk/Gumroad |
| `docs/SELLING-PAGE.md` | Buyer-facing copy aligned with storefront model |
| `docs/SELLING-UNSIGNED.md` | Keep unsigned OS warnings; point to storefront (not GitHub binary) |
| `docs/COMMERCIAL-TERMS.md` | Clarify: no app license mgmt; purchase via storefront |
| `docs/OFFICIAL-END-STATE.md` | Sell-complete vs optional signing |
| `docs/OFFICIAL-RELEASE-RUNBOOK.md` | O12 = storefront + source tag |
| `docs/operator/FINAL-8-GATES.md` | Replace GitHub binary release gates |
| `docs/WINDOWS-LAB-QUEUE.md` | Mark optional post-sell hardening |
| `docs/CHECKLIST-OFFICIAL-REPO-COMPLETE.md` | Sync open items to new model |
| `docs/superpowers/plans/2026-07-17-trareon-lab-3month-official-release.md` | Rephrase Week 13 O12 rows |
| `scripts/publish-storefront-release.sh` | New: verify local dist + checksums; refuse `gh release` binary upload |
| `scripts/publish-official-release.sh` | Retire/redirect to storefront script (fail if used for binary GH upload) |
| `scripts/cut-official-v1.sh` | Source-only tag; do not require signed installer evidence for sell path |
| `scripts/check-no-github-binaries.sh` | Guard: fail if Release assets look like product binaries (doc + local check) |
| `packaging/signing-dry-run.sh` | Assert storefront docs/scripts present |
| `release-evidence/OFFICIAL-1.0.0/GATES.md` / `gather.sh` | Soften O1–O3 as optional for storefront sell; keep as hardening gates |
| `docs/DECISION-REGISTER.md` | ADR-style row for storefront distribution |
| `docs/ai-session-log/2026-07-17-storefront-distribution.md` | Session record |

---

### Task 1: Canonical storefront distribution doc

**Files:**
- Create: `docs/DISTRIBUTION-STOREFRONT.md`
- Modify: `docs/SELLING-UNSIGNED.md` (add top pointer to storefront doc)
- Test: manual — file exists and contains required headings

**Interfaces:**
- Produces: operator checklist steps named Build → Checksum → Upload → Disclose SHA
- Consumes: existing `packaging/build-macos-unsigned.sh`, `packaging/build-linux-unsigned.sh`, `docs/PACKAGING-UNSIGNED-BUILDS.md`

- [ ] **Step 1: Write `docs/DISTRIBUTION-STOREFRONT.md`** with exactly these sections:

```markdown
# Storefront distribution (source public, binary private)

**Spec:** `docs/superpowers/specs/2026-07-17-storefront-binary-distribution-design.md`

## Rule

- GitHub = source + docs + CI tests only.
- Product binaries live on the operator machine and on Lynk.id / Gumroad only.
- Never attach installers to GitHub Releases.

## Build (full binary, no license gate)

```bash
# macOS / Linux examples — see also docs/PACKAGING-UNSIGNED-BUILDS.md
./packaging/build-macos-unsigned.sh   # or build-linux-unsigned.sh
# Windows: follow packaging/WINDOWS-UNSIGNED.md on a Windows host
```

Artifacts under `dist/` (gitignored).

## Checksums

```bash
mkdir -p dist/1.0.0
# after copying platform artifacts into dist/1.0.0/<platform>/
(cd dist/1.0.0 && find . -type f ! -name SHA256SUMS -print0 | sort -z | xargs -0 shasum -a 256) > dist/1.0.0/SHA256SUMS
```

## Upload

1. Upload zip/tarball + `SHA256SUMS` to Lynk.id and/or Gumroad.
2. On the product page list: freeze git SHA, OS matrix, link to public GitHub source, GPL-3.0-only, unsigned/SmartScreen/Gatekeeper warnings (`docs/SELLING-UNSIGNED.md`).

## Optional source-only tag

```bash
git tag -a v1.0.0 -m "source freeze for storefront v1.0.0"
# Do NOT gh release upload binaries
```

## Optional signing

Signing scripts under `packaging/sign-*.sh` remain available for later hardening. They are not required to sell.
```

- [ ] **Step 2: Prepend pointer to `docs/SELLING-UNSIGNED.md`**

At top of file (after title), add:

```markdown
**Binary delivery:** Lynk.id / Gumroad only — see `docs/DISTRIBUTION-STOREFRONT.md`. Do not publish installers on GitHub.
```

- [ ] **Step 3: Verify**

Run: `test -f docs/DISTRIBUTION-STOREFRONT.md && rg -n "Never attach installers" docs/DISTRIBUTION-STOREFRONT.md`
Expected: exit 0; one matching line

- [ ] **Step 4: Commit**

```bash
git add docs/DISTRIBUTION-STOREFRONT.md docs/SELLING-UNSIGNED.md
git commit -m "docs: add storefront distribution operator guide"
```

---

### Task 2: Selling page + commercial terms

**Files:**
- Modify: `docs/SELLING-PAGE.md`
- Modify: `docs/COMMERCIAL-TERMS.md`
- Test: `rg` checks for Gumroad/Lynk and “no license key”

**Interfaces:**
- Consumes: Task 1 `DISTRIBUTION-STOREFRONT.md`
- Produces: buyer-facing copy without “lab/training licensing” ambiguity that implies in-app license

- [ ] **Step 1: Replace `docs/SELLING-PAGE.md` body** with:

```markdown
# Selling page — Trareon Lab 1.0

**Channel:** Lynk.id / Gumroad (paid download). Source: public GitHub (GPL-3.0-only).  
**Build:** full feature binary — **no license key, activation, or DRM in the app.**

## Features

Offline case examination, `.fsnap` import, disk raw ingest, FS browsing, Windows/macOS/Linux artifact subsets, bookmarks, signed offline share packs, timeline, draft/sealed reports.

## Limits

- Installers typically **unsigned** — SmartScreen / Gatekeeper warnings; see `docs/SELLING-UNSIGNED.md`
- Not ISO accredited / not court-ready
- OS matrix: Win10/11 x64, macOS 14/15 arm64, Ubuntu/Debian/Kali x64
- Binaries are **not** published on GitHub Releases

## How buyers get the app

1. Purchase on Lynk.id or Gumroad.
2. Download the OS-specific archive + verify SHA-256 against the listed `SHA256SUMS` / freeze SHA.
3. Source corresponding to that freeze SHA is on the public GitHub repo (`LICENSE` = GPL-3.0-only).

## Operator build path

`docs/DISTRIBUTION-STOREFRONT.md`
```

- [ ] **Step 2: Replace `docs/COMMERCIAL-TERMS.md`** with:

```markdown
# Commercial terms (alongside GPLv3)

- Lab source is GPL-3.0-only on the public GitHub repository.
- Paid storefront copies (Lynk.id / Gumroad) are convenience builds of that source: same GPL rights apply to recipients.
- There is **no** in-app license management, activation server, or seat enforcement.
- Optional paid support, training, or custom engineering may be sold separately; they do not convert the Lab into a proprietary product.
- Product binaries are delivered via storefront only — not via GitHub Release assets.
```

- [ ] **Step 3: Verify**

Run: `rg -n "no license key|Lynk|Gumroad|not.*GitHub Release" docs/SELLING-PAGE.md docs/COMMERCIAL-TERMS.md`
Expected: matches on both files; no line requiring in-app activation

- [ ] **Step 4: Commit**

```bash
git add docs/SELLING-PAGE.md docs/COMMERCIAL-TERMS.md
git commit -m "docs: align selling page with storefront full-binary model"
```

---

### Task 3: Rewrite publish scripts (block GitHub binary upload)

**Files:**
- Create: `scripts/publish-storefront-release.sh`
- Create: `scripts/check-no-github-binaries.sh`
- Modify: `scripts/publish-official-release.sh` (redirect / fail-closed)
- Modify: `scripts/cut-official-v1.sh` (source-only tag path for storefront)
- Test: run scripts; expect fail without `dist/`, succeed dry checks; never call `gh release create` with binaries

**Interfaces:**
- `publish-storefront-release.sh [version]` exits 0 only if `dist/<version>/SHA256SUMS` exists and is non-empty; prints upload checklist; does **not** invoke `gh`
- `publish-official-release.sh` prints deprecation and execs storefront script OR exits 2 with message to use storefront script
- `cut-official-v1.sh` may tag source without requiring O1–O3 signature files when `STOREFRONT_SELL=1`

- [ ] **Step 1: Write `scripts/check-no-github-binaries.sh`**

```bash
#!/usr/bin/env bash
# Fail if a GitHub Release for TAG already has installer-like assets (operator guard).
set -euo pipefail
TAG="${1:-v1.0.0}"
command -v gh >/dev/null 2>&1 || { echo "gh not installed; skip remote check"; exit 0; }
if ! gh release view "$TAG" >/dev/null 2>&1; then
  echo "No GitHub Release $TAG — OK (binaries must stay off GitHub)"
  exit 0
fi
assets="$(gh release view "$TAG" --json assets --jq '.assets[].name' 2>/dev/null || true)"
echo "$assets" | grep -Eii '\.(exe|msi|dmg|pkg|appimage|deb|rpm|zip|tar\.gz)$' && {
  echo "FORBIDDEN: installer-like assets on GitHub Release $TAG" >&2
  exit 1
}
echo "Release $TAG has no installer-like assets — OK"
```

- [ ] **Step 2: Write `scripts/publish-storefront-release.sh`**

```bash
#!/usr/bin/env bash
# Prepare / verify local storefront publish — NEVER uploads to GitHub Releases.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
VER="${1:-1.0.0}"
DIST="${ROOT}/dist/${VER}"
fail() { echo "MISSING: $1" >&2; exit 1; }

test -d "$DIST" || fail "$DIST (build full binaries into dist/${VER}/ first)"
test -s "${DIST}/SHA256SUMS" || fail "${DIST}/SHA256SUMS"
bash "${ROOT}/scripts/check-no-github-binaries.sh" "v${VER}"

echo "Storefront publish checklist for ${VER}:"
echo "  1. Freeze SHA: $(git -C "$ROOT" rev-parse HEAD)"
echo "  2. Upload contents of ${DIST} to Lynk.id / Gumroad"
echo "  3. Paste SHA256SUMS + freeze SHA on the product page"
echo "  4. Link public GitHub source + docs/SELLING-UNSIGNED.md"
echo "  5. Do NOT: gh release upload binaries"
echo "OK — local evidence present; upload is manual."
```

- [ ] **Step 3: Replace body of `scripts/publish-official-release.sh`** with:

```bash
#!/usr/bin/env bash
# Deprecated name: Official sell path is storefront-only (no GitHub binary Release).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
VER="${1:-1.0.0}"
# Accept v1.0.0 or 1.0.0
VER="${VER#v}"
echo "NOTE: publish-official-release.sh no longer creates GitHub Release binaries." >&2
echo "Use storefront upload per docs/DISTRIBUTION-STOREFRONT.md" >&2
exec bash "${ROOT}/scripts/publish-storefront-release.sh" "$VER"
```

- [ ] **Step 4: Update `scripts/cut-official-v1.sh`** so storefront sell can tag without O1–O3:

```bash
#!/usr/bin/env bash
# Cut annotated source tag. Storefront sell: STOREFRONT_SELL=1 skips signed-installer gather.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TAG="${1:-v1.0.0}"
TAGGER_NAME="${OFFICIAL_TAGGER_NAME:-yusufsaaas}"

if [[ "${STOREFRONT_SELL:-}" == "1" ]]; then
  test -s "${ROOT}/dist/${TAG#v}/SHA256SUMS" || {
    echo "STOREFRONT_SELL=1 requires dist/${TAG#v}/SHA256SUMS" >&2
    exit 1
  }
else
  bash "${ROOT}/release-evidence/OFFICIAL-1.0.0/gather.sh"
  bash "${ROOT}/packaging/verify-signatures.sh"
fi

TIP="$(git -C "${ROOT}" rev-parse HEAD)"
git -C "${ROOT}" tag -a "${TAG}" -m "Source freeze ${TAG} by ${TAGGER_NAME}; tip ${TIP}; binaries storefront-only"
echo "Created annotated source tag ${TAG} at ${TIP}"
echo "Next: bash scripts/publish-storefront-release.sh ${TAG#v}"
```

- [ ] **Step 5: chmod + verify**

```bash
chmod +x scripts/publish-storefront-release.sh scripts/check-no-github-binaries.sh scripts/publish-official-release.sh scripts/cut-official-v1.sh
bash scripts/publish-storefront-release.sh 1.0.0; echo exit=$?
```

Expected: exit 1 with `MISSING: .../dist/1.0.0`

- [ ] **Step 6: Commit**

```bash
git add scripts/publish-storefront-release.sh scripts/check-no-github-binaries.sh scripts/publish-official-release.sh scripts/cut-official-v1.sh
git commit -m "feat: storefront publish scripts; block GitHub binary releases"
```

---

### Task 4: Realign Official end-state, runbook, FINAL-8, Windows queue

**Files:**
- Modify: `docs/OFFICIAL-END-STATE.md`
- Modify: `docs/OFFICIAL-RELEASE-RUNBOOK.md` (O12 row + go-live section)
- Modify: `docs/operator/FINAL-8-GATES.md`
- Modify: `docs/WINDOWS-LAB-QUEUE.md`
- Modify: `docs/CHECKLIST-OFFICIAL-REPO-COMPLETE.md`
- Modify: `docs/MACOS-LINUX-SIGNING-QUEUE.md` (optional hardening header)
- Test: `rg` that O12 no longer requires GitHub signed installers as sell gate

**Interfaces:**
- O12 evidence = storefront URL + freeze SHA + `SHA256SUMS` path (local), optional source tag
- Human “8 gates” reduced/reframed: selling no longer blocked on crypto wet-sign / Authenticode

- [ ] **Step 1: Rewrite sell-complete section in `docs/OFFICIAL-END-STATE.md`**

Replace “Human + lab track” with two tracks:

```markdown
## Storefront sell track — COMPLETE when

1. Full binaries built into local `dist/<ver>/` + `SHA256SUMS`
2. Uploaded to Lynk.id / Gumroad with freeze SHA + GPL source link
3. Optional source-only tag `v1.0.0` (no GitHub binary assets)
4. Selling page honesty intact (`docs/SELLING-PAGE.md`)

## Optional hardening (not sell blockers)

- Path C signing / notarization / Authenticode
- Windows lab queue W1–W6
- External crypto review / Indonesia wet sign-off (enterprise/compliance customers)
```

- [ ] **Step 2: Update O12 row in `docs/OFFICIAL-RELEASE-RUNBOOK.md`**

Change O12 requirement/evidence to:

| Gate | Requirement | Evidence |
|---|---|---|
| O12 | Storefront publish of full binary + source-only tag; **no** GitHub installer assets | Storefront URL + freeze SHA + local `dist/.../SHA256SUMS`; `scripts/check-no-github-binaries.sh` |

Update go-live scripts list to call `publish-storefront-release.sh` instead of GitHub binary publish.

- [ ] **Step 3: Rewrite `docs/operator/FINAL-8-GATES.md`** for storefront model

Replace table so sell gates are: freeze SHA, SHA256SUMS, storefront upload, no GH binaries, selling honesty, source tag. Move crypto/Indonesia/Authenticode to “optional compliance”.

- [ ] **Step 4: Header on `docs/WINDOWS-LAB-QUEUE.md`**

```markdown
**Role:** optional post-sell hardening (Authenticode / ThinkPad smoke). **Not required** to sell full binaries on Lynk.id / Gumroad.
```

- [ ] **Step 5: Sync `docs/CHECKLIST-OFFICIAL-REPO-COMPLETE.md`** open list to storefront sell blockers only (local dist + upload), not signed GH release.

- [ ] **Step 6: Verify**

Run: `rg -n "storefront|Lynk|no GitHub installer|optional post-sell" docs/OFFICIAL-END-STATE.md docs/OFFICIAL-RELEASE-RUNBOOK.md docs/operator/FINAL-8-GATES.md docs/WINDOWS-LAB-QUEUE.md`
Expected: matches; `rg "GitHub Release: attach signed installers" docs/operator/FINAL-8-GATES.md` → no matches

- [ ] **Step 7: Commit**

```bash
git add docs/OFFICIAL-END-STATE.md docs/OFFICIAL-RELEASE-RUNBOOK.md docs/operator/FINAL-8-GATES.md docs/WINDOWS-LAB-QUEUE.md docs/CHECKLIST-OFFICIAL-REPO-COMPLETE.md docs/MACOS-LINUX-SIGNING-QUEUE.md
git commit -m "docs: Official sell path is storefront; signing optional"
```

---

### Task 5: Soften gather O1–O3 for storefront; keep hardening mode

**Files:**
- Modify: `release-evidence/OFFICIAL-1.0.0/gather.sh`
- Modify: `release-evidence/OFFICIAL-1.0.0/GATES.md`
- Modify: `packaging/signing-dry-run.sh`
- Test: `STOREFRONT_SELL=1 bash release-evidence/OFFICIAL-1.0.0/gather.sh` behavior

**Interfaces:**
- When `STOREFRONT_SELL=1`: require `dist/*/SHA256SUMS` (or `dist/1.0.0/SHA256SUMS`), `docs/DISTRIBUTION-STOREFRONT.md`, and fail if example stubs claimed as signed evidence; **skip** O1–O3 signature files and O8/O9 wet signatures
- Default (unset): keep existing strict Official hardening gather

- [ ] **Step 1: At top of gather checks, add storefront branch**

```bash
if [[ "${STOREFRONT_SELL:-}" == "1" ]]; then
  VER="${STOREFRONT_VERSION:-1.0.0}"
  test -s "${ROOT}/dist/${VER}/SHA256SUMS" || fail "storefront dist/${VER}/SHA256SUMS"
  test -f "${ROOT}/docs/DISTRIBUTION-STOREFRONT.md" || fail "DISTRIBUTION-STOREFRONT.md"
  test -f "${ROOT}/docs/SELLING-PAGE.md" || fail "SELLING-PAGE.md"
  echo "Storefront gather PASS (signing/O8/O9 not required for sell)."
  exit 0
fi
# … existing strict path unchanged …
```

- [ ] **Step 2: Annotate `GATES.md`** — mark O1–O3/O8/O9 as “optional hardening; not storefront sell blockers”; O12 as storefront.

- [ ] **Step 3: Update `packaging/signing-dry-run.sh`** to require `docs/DISTRIBUTION-STOREFRONT.md` and `scripts/publish-storefront-release.sh`.

- [ ] **Step 4: Verify**

```bash
STOREFRONT_SELL=1 bash release-evidence/OFFICIAL-1.0.0/gather.sh; echo exit=$?
# expect 1 (no dist yet)
bash packaging/signing-dry-run.sh
# expect PASS after Task 1–3 files exist
```

- [ ] **Step 5: Commit**

```bash
git add release-evidence/OFFICIAL-1.0.0/gather.sh release-evidence/OFFICIAL-1.0.0/GATES.md packaging/signing-dry-run.sh
git commit -m "feat: storefront gather mode without signed-installer gates"
```

---

### Task 6: Official plan Week 13 + decision register + ADR note

**Files:**
- Modify: `docs/superpowers/plans/2026-07-17-trareon-lab-3month-official-release.md` (Hari 64–65 O12 bullets)
- Modify: `docs/DECISION-REGISTER.md` (new row)
- Create: `docs/ai-session-log/2026-07-17-storefront-distribution.md`
- Modify: mark spec status Accepted in `docs/superpowers/specs/2026-07-17-storefront-binary-distribution-design.md`
- Test: plan open rows no longer say “GitHub Release signed installers” as unchecked sell requirement without storefront alternative

- [ ] **Step 1: Edit Week 13 Thu/Fri bullets** to:

```markdown
- [ ] Storefront publish: Lynk.id/Gumroad + SHA256SUMS + freeze SHA *(scripts/publish-storefront-release.sh)*
- [ ] Source-only annotated tag `v1.0.0` *(STOREFRONT_SELL=1 scripts/cut-official-v1.sh)*
- [ ] Confirm no installer assets on GitHub Release *(scripts/check-no-github-binaries.sh)*
- [ ] Update PRD/status: storefront v1.0.0 available (source on GitHub)
```

Leave crypto/Indonesia/Authenticode as optional `[ ]` under a new “Optional compliance” note, not sell blockers.

- [ ] **Step 2: Append decision register row**

| ID | Decision | Date | Notes |
|---|---|---|---|
| DIST-001 | Storefront binary distribution; source public; no in-app license; no GitHub installers | 2026-07-17 | Spec `docs/superpowers/specs/2026-07-17-storefront-binary-distribution-design.md` |

- [ ] **Step 3: Spec status → Accepted**; write session log.

- [ ] **Step 4: Verify**

```bash
rg -n "DIST-001|publish-storefront-release|STOREFRONT_SELL" docs/DECISION-REGISTER.md docs/superpowers/plans/2026-07-17-trareon-lab-3month-official-release.md
bash packaging/signing-dry-run.sh
```

- [ ] **Step 5: Commit**

```bash
git add docs/superpowers/plans/2026-07-17-trareon-lab-3month-official-release.md docs/DECISION-REGISTER.md docs/superpowers/specs/2026-07-17-storefront-binary-distribution-design.md docs/ai-session-log/2026-07-17-storefront-distribution.md
git commit -m "docs: record storefront distribution decision and plan sync"
```

---

### Task 7: Final verification + push

**Files:** none new

- [ ] **Step 1: Confirm no in-app license code was introduced**

```bash
rg -n "license_key|activation_server|phone_home|drm_" apps crates --glob '!**/target/**' || true
```

Expected: no new matches for product DRM (pre-existing unrelated “license” words in GPL/docs OK).

- [ ] **Step 2: Run dry-run + storefront publish fail-closed**

```bash
bash packaging/signing-dry-run.sh
bash scripts/publish-storefront-release.sh 1.0.0; echo exit=$?
```

Expected: dry-run PASS; publish exit 1 without `dist/`.

- [ ] **Step 3: `graphify update .`**

- [ ] **Step 4: Push `main`** if user wants remote updated.

---

## Spec coverage self-review

| Spec section | Task |
|---|---|
| §2 Goals full binary, no DRM | Tasks 2, 7 |
| §2 GitHub source only | Tasks 1, 3, 4 |
| §2 Storefront delivery | Tasks 1, 2, 3 |
| §3 Non-goals signing not sell gate | Tasks 4, 5 |
| §4 Distribution diagram | Task 1 |
| §5 Product behavior | Task 2 |
| §6 GPL + freeze SHA | Tasks 1, 2, 3 |
| §7 O12 rewrite / publish scripts | Tasks 3, 4, 6 |
| §8 Operator checklist | Task 1 |
| §9 Accidental GH publish risk | Task 3 `check-no-github-binaries.sh` |
| §10 Success criteria | Task 7 |

Placeholder scan: none intentional.
