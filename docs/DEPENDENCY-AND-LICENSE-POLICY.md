# Dependency and License Policy

**Meaning of “no external dependency” for R1:** the installed application requires no separately installed language runtime or forensic application for core R1 workflows.

Bundled libraries are permitted only when source/provenance, license, version pin, hashes, SBOM entry, update policy, vulnerability process, and validation coverage are controlled.

Do not rewrite mature parsers or codecs merely to avoid packaging a dependency. Reimplementation requires a documented security, licensing, determinism, or portability reason plus equivalent validation evidence.

Allowed license classes for bundled code: MIT, Apache-2.0, BSD-2/3, public domain/SQLite, Unicode. Copyleft that forces Lab source disclosure on proprietary distribution requires Legal approval before merge.
