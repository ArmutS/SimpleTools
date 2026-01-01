<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let input = "";
  let output = "";
  let mode: "rot13" | "reverse" | "base64" = "rot13";

  async function process() {
    if (!input) {
      output = "";
      return;
    }

    try {
      output = await invoke("process_obfuscator", {
        current_text: input,
        mode: mode,
      });
    } catch (e) {
      output = "Error: " + e;
    }
  }

  $: process(), input, mode;
</script>

<main class="tool-container">
  <div class="pane input-pane">
    <div class="pane-header">INPUT</div>
    <textarea
      class="editor source-editor"
      placeholder="Enter text..."
      spellcheck="false"
      bind:value={input}
    ></textarea>
  </div>

  <div class="divider"></div>

  <div class="pane output-pane">
    <div class="controls">
      <div class="filter-label">MODE:</div>

      <label class="toggle-chip" class:active={mode === "rot13"}>
        <input type="radio" bind:group={mode} value="rot13" />
        ROT13
      </label>
      <label class="toggle-chip" class:active={mode === "reverse"}>
        <input type="radio" bind:group={mode} value="reverse" />
        Reverse
      </label>
      <label class="toggle-chip" class:active={mode === "base64"}>
        <input type="radio" bind:group={mode} value="base64" />
        Base64 Encode
      </label>
    </div>

    <div class="pane-header result-header">
      <span>RESULT</span>
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

  .controls {
    padding: 12px;
    background-color: var(--bg-input);
    border-bottom: 1px solid var(--border-color);
    display: flex;
    align-items: center;
    gap: 8px;
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

  .filter-label {
    font-size: 0.7rem;
    color: var(--text-muted);
    font-weight: bold;
    margin-right: 5px;
  }

  .toggle-chip {
    display: inline-flex;
    align-items: center;
    padding: 4px 10px;
    border-radius: 12px;
    border: 1px solid var(--border-color);
    background: var(--bg-app);
    color: var(--text-muted);
    font-size: 0.8rem;
    cursor: pointer;
    transition: all 0.2s ease;
    user-select: none;
  }

  .toggle-chip input {
    display: none;
  }

  .toggle-chip.active {
    background-color: var(--accent);
    border-color: var(--accent);
    color: var(--bg-app);
    font-weight: 600;
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
