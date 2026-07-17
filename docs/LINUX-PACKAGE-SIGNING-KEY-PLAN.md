# Linux package signing key plan

**Status:** PLAN ONLY — no production private key is stored in this repository.  
**Owner:** Release Engineering  
**Related gates:** Official O3, NFR-SEC-008 platform signing status

## Goal

Provide a reproducible, offline-friendly signing path for Linux `.deb` and/or AppImage artifacts without committing secrets.

## Key material policy

| Item | Location | Rule |
|---|---|---|
| Private signing key | Operator HSM / offline encrypted store only | Never commit; never paste into chat logs |
| Passphrase / recovery | Operator sealed record | Outside git |
| Public key | `release-evidence/OFFICIAL-1.0.0/linux-signing-pubkey.asc` (when issued) | Commit only the public key and fingerprint |
| Key algorithm preference | OpenPGP (GnuPG) Ed25519 or RSA-4096; age for encrypted operator backups | Prefer Ed25519 if tooling on all release hosts supports it |

## Operator steps (when Path C is ready)

1. Generate a dedicated release signing key on an air-gapped or locked-down operator machine.
2. Record fingerprint, creation date, and expiry in `docs/OFFICIAL-RELEASE-RUNBOOK.md` dependency register.
3. Export public key only into `release-evidence/OFFICIAL-1.0.0/linux-signing-pubkey.asc`.
4. Sign release packages; write verification output to `release-evidence/OFFICIAL-1.0.0/linux-sig.txt`.
5. Publish the public key with the GitHub Release notes so buyers can verify offline.

## Non-goals

- Committing private keys, `.asc` secret keyrings, or age identities.
- Claiming O3 `PASS` before signed artifacts and verification logs exist.
