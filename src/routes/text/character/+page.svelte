<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let input = "";
  let chars: {
    char: string;
    unicode: string;
    decimal: number;
    entity: string;
  }[] = [];

  async function process() {
    if (!input) {
      chars = [];
      return;
    }

    try {
      // Need to cast or define type
      chars = await invoke("process_char_inspector", { current_text: input });
    } catch (e) {
      console.error(e);
    }
  }

  $: process(), input;
</script>

<main class="tool-container">
  <div class="pane input-pane">
    <div class="pane-header">INPUT (Max 500 chars analyzed)</div>
    <textarea
      class="editor source-editor"
      placeholder="Paste text or emoji..."
      spellcheck="false"
      bind:value={input}
    ></textarea>
  </div>

  <div class="divider"></div>

  <div class="pane output-pane">
    <div class="pane-header result-header">
      <span>ANALYSIS</span>
    </div>

    <div class="table-container">
      <table>
        <thead>
          <tr>
            <th>Char</th>
            <th>Unicode</th>
            <th>Decimal</th>
            <th>HTML Entity</th>
          </tr>
        </thead>
        <tbody>
          {#each chars as item}
            <tr>
              <td class="char-cell">{item.char}</td>
              <td>{item.unicode}</td>
              <td>{item.decimal}</td>
              <td>{item.entity}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>
</main>

<style>
  .tool-container {
    height: 100vh;
    display: flex;
    background-color: var(--bg-app);
    color: var(--text-main);
    font-family: "Consolas", monospace;
    overflow: hidden;
  }

  .pane {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .divider {
    width: 1px;
    background: var(--border-color);
    opacity: 0.5;
  }

  .pane-header {
    padding: 8px 12px;
    background: var(--bg-header);
    border-bottom: 1px solid var(--border-color);
    font-size: 0.75rem;
    font-weight: bold;
    color: var(--text-muted);
    display: flex;
    justify-content: space-between;
    align-items: center;
    user-select: none;
    flex-shrink: 0;
  }

  .editor {
    flex: 1; /* Input smallish */
    max-height: 200px;
    background: transparent;
    border-bottom: 1px solid var(--border-color);
    color: var(--text-main);
    padding: 15px;
    resize: none;
    outline: none;
    font-family: inherit;
    font-size: 1.2rem;
    line-height: 1.6;
  }

  .input-pane {
    flex: 0 0 auto;
  }

  .source-editor {
    background-color: rgba(0, 0, 0, 0.1);
  }

  .output-pane {
    flex: 1;
  }

  .table-container {
    flex: 1;
    overflow-y: auto;
    padding: 0;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.9rem;
  }

  th,
  td {
    text-align: left;
    padding: 8px 15px;
    border-bottom: 1px solid var(--border-color);
  }

  th {
    position: sticky;
    top: 0;
    background: var(--bg-input);
    color: var(--text-muted);
    font-size: 0.75rem;
  }

  .char-cell {
    font-size: 1.5rem;
    color: var(--accent);
    width: 60px;
    text-align: center;
  }

  tr:hover {
    background-color: rgba(255, 255, 255, 0.05);
  }
</style>
