# Cryptographic Profile

**ADR-003:** ACCEPTED (pending named external cryptography review before production signing keys)  
**Date:** 2026-07-17

## Digests

- **Authoritative integrity digest:** SHA-256, always recorded with algorithm identifier `sha256`.
- **Optional second modern digest:** may be configured (e.g. SHA-512/BLAKE3) only when algorithm identifier is stored beside the digest.
- **Legacy correlation only:** MD5 and SHA-1 may be computed solely for correlating with legacy tools and must be labeled `non_authoritative_legacy_correlation` in UI and exports.

## Signature envelopes

Canonical JSON payload → SHA-256 payload digest → Ed25519 signature. Envelope fields per `schemas/signature-envelope.schema.json`.

Trust states exactly: `VALID_TRUSTED`, `VALID_UNTRUSTED`, `INVALID`, `EXPIRED_OR_REVOKED`, `NOT_SIGNED`, `NOT_CHECKED`.

Indonesia deployment profile: organization PKI may wrap Ed25519 keys in offline trust bundles; evidence verification must not require Internet access. Revocation data is imported as a signed offline trust bundle.

## Key lifecycle

Generation (OS CSPRNG), import, hardware-backed availability when platform keystore present, encrypted storage at rest, backup/recovery with dual control, rotation, revocation, offline trust bundles, key loss handling (invalidate signatures, require re-sign), and export of public material only. No proprietary cryptographic primitives.
