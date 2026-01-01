<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let input = "";
  let output = "";
  let header = "";

  async function decodeJWT() {
    if (!input) {
      output = "";
      header = "";
      return;
    }

    try {
      // Need to type definition or use any
      const res: any = await invoke("process_jwt_decode", { token: input });
      header = res.header;
      output = res.payload;
    } catch (e) {
      output = "Error: " + e;
      header = "";
    }
  }

  $: decodeJWT(), input;
</script>

```
<main class="tool-container">
  <div class="pane input-pane">
    <div class="pane-header">ENCODED TOKEN</div>
    <textarea
      class="editor source-editor"
      placeholder="Paste JWT here (ey...)"
      spellcheck="false"
      bind:value={input}
    ></textarea>
  </div>

  <div class="divider"></div>

  <div class="pane output-pane">
    <div class="pane-header">HEADER</div>
    <textarea
      class="editor result-editor small-editor"
      readonly
      bind:value={header}
    ></textarea>

    <div class="pane-header result-header">
      <span>PAYLOAD</span>
      <button
        class="action-btn"
        on:click={() => navigator.clipboard.writeText(output)}>Copy</button
      >
    </div>
    <textarea class="editor result-editor" readonly bind:value={output}
    ></textarea>
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
    flex: 1;
    background: transparent;
    border: none;
    color: var(--text-main);
    padding: 15px;
    resize: none;
    outline: none;
    font-family: inherit;
    font-size: 0.9rem;
    line-height: 1.6;
  }

  .source-editor {
    background-color: rgba(0, 0, 0, 0.1);
  }

  .result-editor {
    color: var(--accent);
  }

  .small-editor {
    flex: 0.4; /* Header is usually smaller */
    border-bottom: 1px solid var(--border-color);
    color: var(--text-muted); /* Helper color for header */
  }

  .action-btn {
    background: transparent;
    border: 1px solid var(--border-color);
    color: var(--text-main);
    padding: 4px 10px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.75rem;
    font-family: inherit;
    transition: background 0.2s;
  }

  .action-btn:hover {
    background: var(--bg-input);
    border-color: var(--text-muted);
    color: var(--accent);
  }
</style>
