# Path C escalation — cert ETA after 2026-09-15

If Apple or Windows certificate ETA slips past **2026-09-15**:

1. Record vendor ETA and alternate vendor options in `docs/operator/PATH-C-PROCUREMENT.md`.
2. Do **not** fake signatures or claim Official Production.
3. Continue shipping Engineering Alpha / unsigned RC (`v0.9.0-sellable` / `v1.0.0-rc1-unsigned`).
4. Shift Official date via ADR-008 schedule amendment.
5. Notify Product Lead same day.

Windows lab work remains queued in `docs/WINDOWS-LAB-QUEUE.md` and does not unblock notarization by itself.
