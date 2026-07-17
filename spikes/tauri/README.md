# Spike: Tauri 2 + Svelte 5

Gate A candidate `C-TAURI`.

## Intent

Prove whether Tauri 2 can satisfy Trareon Lab mandatory gates without placing evidence bytes or secrets in webview-accessible storage, and without requiring a separately installed language runtime on Windows, macOS, and Linux.

## Build / interactive

```bash
cd spikes/tauri
npm install
npm run tauri dev
```

## Measure

See `../COPY-PASTE-TAURI.md` or:

```bash
bash spikes/scripts/measure-tauri.sh macos
```

Rust owns case logic (`lab-spike-core`). The Svelte UI only receives paginated row slices over IPC commands.
