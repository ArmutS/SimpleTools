<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let type: "paragraphs" | "sentences" | "words" = "paragraphs";
  let count = 5;
  let output = "";

  async function generate() {
    if (count < 1) count = 1;
    if (count > 1000) count = 1000;

    try {
      output = await invoke("process_lorem", {
        count: count,
        mode: type,
      });
    } catch (e) {
      output = "Error: " + e;
    }
  }

  // Generate initial on load
  $: generate(), type, count;
</script>

<main class="tool-container">
  <div class="pane output-pane">
    <div class="controls">
      <div class="filter-label">GENERATE:</div>

      <input
        type="number"
        bind:value={count}
        min="1"
        max="1000"
        class="number-input"
      />

      <label class="toggle-chip" class:active={type === "paragraphs"}>
        <input type="radio" bind:group={type} value="paragraphs" />
        Paragraphs
      </label>
      <label class="toggle-chip" class:active={type === "sentences"}>
        <input type="radio" bind:group={type} value="sentences" />
        Sentences
      </label>
      <label class="toggle-chip" class:active={type === "words"}>
        <input type="radio" bind:group={type} value="words" />
        Words
      </label>

      <button class="action-btn generate-btn" on:click={generate}
        >Regenerate</button
      >
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
    background: var(--bg-app);
    border: none;
    color: var(--text-main);
    padding: 15px;
    resize: none;
    outline: none;
    font-family: inherit;
    font-size: 0.9rem;
    line-height: 1.6;
  }

  .filter-label {
    font-size: 0.7rem;
    color: var(--text-muted);
    font-weight: bold;
    margin-right: 5px;
  }

  .number-input {
    background: var(--bg-app);
    border: 1px solid var(--border-color);
    color: var(--text-main);
    padding: 4px;
    width: 60px;
    border-radius: 4px;
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

  .generate-btn {
    margin-left: auto;
  }
</style>
