<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  type Row = {
    index: number;
    path: string;
    size: number;
    hash_prefix: string;
    hash_full: string;
  };

  let status = $state("Ready — open a synthetic case. Evidence bytes stay in Rust.");
  let filter = $state("0");
  let rows = $state<Row[]>([]);
  let total = $state(0);
  let detail = $state("");
  let selected = $state<number | null>(null);

  async function openCase() {
    status = "Opening…";
    try {
      const res = await invoke<{
        elapsed_ms: number;
        table_ms: number;
        total: number;
        page: Row[];
        note: string;
      }>("open_case");
      rows = res.page;
      total = res.total;
      status = `Opened in ${res.elapsed_ms} ms; table ${res.table_ms} ms; showing ${rows.length}/${total}. ${res.note}`;
      if (rows[0]) selectRow(rows[0]);
    } catch (e) {
      status = `open failed: ${e}`;
    }
  }

  async function applyFilter() {
    status = "Filtering…";
    try {
      const res = await invoke<{
        elapsed_ms: number;
        total: number;
        page: Row[];
      }>("page_rows", { offset: 0, limit: 200, prefix: filter });
      rows = res.page;
      total = res.total;
      status = `Filter '${filter}' => ${total} matches; page ${rows.length} in ${res.elapsed_ms} ms`;
      if (rows[0]) selectRow(rows[0]);
    } catch (e) {
      status = `filter failed: ${e}`;
    }
  }

  function selectRow(r: Row) {
    selected = r.index;
    detail = `index=${r.index} size=${r.size} hash=${r.hash_full} path=${r.path}`;
  }

  async function startHash() {
    try {
      status = await invoke<string>("start_hash");
    } catch (e) {
      status = `start failed: ${e}`;
    }
  }

  async function cancelHash() {
    try {
      status = await invoke<string>("cancel_hash");
    } catch (e) {
      status = `cancel failed: ${e}`;
    }
  }

  async function crashWorker() {
    try {
      status = await invoke<string>("crash_worker");
    } catch (e) {
      status = `crash failed: ${e}`;
    }
  }

  async function exportCase() {
    try {
      const res = await invoke<{
        path: string;
        elapsed_ms: number;
        export_sha256: string;
        filtered_count: number;
      }>("export_case", { filterPrefix: filter });
      status = `Exported ${res.filtered_count} filtered in ${res.elapsed_ms} ms; sha256=${res.export_sha256}`;
    } catch (e) {
      status = `export failed: ${e}`;
    }
  }
</script>

<main class="wrap">
  <header>
    <h1>Trareon Lab Gate A — C-TAURI</h1>
    <p class="sub">Tauri 2 + Svelte 5 · Rust owns case logic · webview is presentation only</p>
  </header>

  <section class="toolbar" aria-label="Case actions">
    <button type="button" onclick={openCase}>Open case (1M rows)</button>
    <label>
      Filter prefix
      <input bind:value={filter} aria-label="Hash filter prefix" />
    </label>
    <button type="button" onclick={applyFilter}>Apply filter</button>
    <button type="button" onclick={startHash}>Start hash</button>
    <button type="button" onclick={cancelHash}>Cancel hash</button>
    <button type="button" onclick={crashWorker}>Crash worker</button>
    <button type="button" onclick={exportCase}>Export</button>
  </section>

  <p class="status" role="status">{status}</p>

  <section class="panes">
    <div class="table" role="region" aria-label="Virtualized row page">
      <table>
        <thead>
          <tr>
            <th scope="col">Index</th>
            <th scope="col">Prefix</th>
            <th scope="col">Path</th>
          </tr>
        </thead>
        <tbody>
          {#each rows as r (r.index)}
            <tr
              class:selected={selected === r.index}
              tabindex="0"
              onclick={() => selectRow(r)}
              onkeydown={(e) => e.key === "Enter" && selectRow(r)}
            >
              <td>{r.index}</td>
              <td>{r.hash_prefix}</td>
              <td>{r.path}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
    <aside class="detail" aria-label="Detail pane">
      <h2>Detail</h2>
      <pre>{detail || "Select a row"}</pre>
    </aside>
  </section>
</main>

<style>
  :root {
    color: #142019;
    background: #e8f0ea;
    font-family: "IBM Plex Sans", "Segoe UI", sans-serif;
  }
  .wrap {
    margin: 0;
    padding: 1rem 1.25rem 1.5rem;
    min-height: 100vh;
    box-sizing: border-box;
  }
  h1 {
    margin: 0;
    font-size: 1.35rem;
  }
  .sub {
    margin: 0.25rem 0 1rem;
    color: #3d5244;
    font-size: 0.9rem;
  }
  .toolbar {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: end;
    margin-bottom: 0.75rem;
  }
  label {
    display: flex;
    flex-direction: column;
    font-size: 0.75rem;
    gap: 0.2rem;
  }
  input,
  button {
    font: inherit;
    padding: 0.45rem 0.7rem;
    border: 1px solid #6a8572;
    border-radius: 4px;
    background: #fff;
  }
  button {
    cursor: pointer;
    background: #1f4d36;
    color: #fff;
    border-color: #1f4d36;
  }
  button:focus-visible,
  input:focus-visible,
  tr:focus-visible {
    outline: 2px solid #0b6e4f;
    outline-offset: 2px;
  }
  .status {
    margin: 0 0 0.75rem;
    padding: 0.5rem 0.65rem;
    background: #d7e6db;
    border-left: 3px solid #1f4d36;
  }
  .panes {
    display: grid;
    grid-template-columns: 1.6fr 1fr;
    gap: 0.75rem;
    min-height: 420px;
  }
  .table {
    overflow: auto;
    background: #fff;
    border: 1px solid #9bb3a4;
  }
  table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.85rem;
  }
  th,
  td {
    text-align: left;
    padding: 0.35rem 0.5rem;
    border-bottom: 1px solid #d5e2d9;
  }
  tr.selected {
    background: #c5ddcf;
  }
  .detail {
    background: #fff;
    border: 1px solid #9bb3a4;
    padding: 0.75rem;
  }
  .detail h2 {
    margin: 0 0 0.5rem;
    font-size: 1rem;
  }
  pre {
    white-space: pre-wrap;
    word-break: break-word;
    margin: 0;
    font-size: 0.85rem;
  }
  @media (max-width: 800px) {
    .panes {
      grid-template-columns: 1fr;
    }
  }
</style>
