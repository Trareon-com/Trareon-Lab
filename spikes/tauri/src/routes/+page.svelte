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
    <p class="brand">TRAREON <span>|</span> LAB</p>
    <h1>Gate A — C-TAURI spike</h1>
    <p class="sub">Spike only · Tauri 2 + Svelte 5 · copper shell tokens match Lab Slint · not the shipping UI</p>
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
    --canvas: #f4f6f8;
    --surface: #ffffff;
    --border: #d8dde3;
    --text: #1e2a36;
    --mute: #5f6f7f;
    --copper: #af622e;
    --copper-muted: #fff4ec;
    --shadow: #00000018;
    color: var(--text);
    background: var(--canvas);
    font-family: Inter, "Segoe UI", sans-serif;
  }
  .wrap {
    margin: 0;
    padding: 1rem 1.25rem 1.5rem;
    min-height: 100vh;
    box-sizing: border-box;
  }
  .brand {
    margin: 0 0 0.35rem;
    font-size: 0.7rem;
    font-weight: 800;
    letter-spacing: 0.08em;
    color: var(--mute);
  }
  .brand span {
    color: var(--copper);
  }
  h1 {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 800;
  }
  .sub {
    margin: 0.25rem 0 1rem;
    color: var(--mute);
    font-size: 0.85rem;
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
    color: var(--mute);
    font-weight: 600;
  }
  input,
  button {
    font: inherit;
    padding: 0.45rem 0.7rem;
    border: 1px solid var(--border);
    border-radius: 6px;
    background: var(--surface);
  }
  button {
    cursor: pointer;
    background: var(--copper);
    color: #fff;
    border-color: var(--copper);
    font-weight: 700;
  }
  button:focus-visible,
  input:focus-visible,
  tr:focus-visible {
    outline: 2px solid #c96a38;
    outline-offset: 2px;
  }
  .status {
    margin: 0 0 0.75rem;
    padding: 0.5rem 0.65rem;
    background: var(--copper-muted);
    border-left: 3px solid var(--copper);
    border-radius: 0 6px 6px 0;
  }
  .panes {
    display: grid;
    grid-template-columns: 1.6fr 1fr;
    gap: 0.75rem;
    min-height: 420px;
  }
  .table {
    overflow: auto;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 8px;
    box-shadow: 0 2px 10px var(--shadow);
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
    border-bottom: 1px solid var(--border);
  }
  tr.selected {
    background: var(--copper-muted);
  }
  .detail {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 8px;
    box-shadow: 0 2px 10px var(--shadow);
    padding: 0.75rem;
  }
  .detail h2 {
    margin: 0 0 0.5rem;
    font-size: 0.7rem;
    font-weight: 800;
    letter-spacing: 0.06em;
    color: var(--mute);
    text-transform: uppercase;
  }
  pre {
    white-space: pre-wrap;
    word-break: break-word;
    margin: 0;
    font-size: 0.85rem;
    font-family: ui-monospace, monospace;
  }
  @media (max-width: 800px) {
    .panes {
      grid-template-columns: 1fr;
    }
  }
</style>
