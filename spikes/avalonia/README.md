# Spike: Avalonia + Rust FFI

Gate A candidate `C-AVALONIA`.

## Intent

Prove whether Avalonia can ship as a self-contained installer without a separately installed .NET runtime, while keeping forensic business logic in Rust through a typed FFI boundary.

## Required equal workflow

See `../README.md`. Implement the synthetic million-row case workflow; Avalonia must not own evidence hashing, provenance, or case lock semantics.

## Status

Scaffold only. No measurement recorded.
