# Dependency and License Policy

**Project license:** Trareon Lab source is **GPL-3.0-only** (see root `LICENSE`).

**Meaning of “no external dependency” for R1:** the installed application requires no separately installed language runtime or forensic application for core R1 workflows.

Bundled libraries are permitted only when source/provenance, license, version pin, hashes, SBOM entry, update policy, vulnerability process, and validation coverage are controlled.

Do not rewrite mature parsers or codecs merely to avoid packaging a dependency. Reimplementation requires a documented security, licensing, determinism, or portability reason plus equivalent validation evidence.

## Compatible dependency licenses

Preferred for bundled code: MIT, Apache-2.0, BSD-2/3, public domain/SQLite, Unicode, and other licenses that are compatible with distributing Lab under GPL-3.0-only.

Dependencies with GPL-incompatible terms must not be linked into the Lab binary without Legal review and an explicit exception or replacement plan.
