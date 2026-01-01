<script lang="ts">
  import cronstrue from "cronstrue";

  let input = "";
  let output = "";
  let nextDates: string[] = [];

  function processCron() {
    if (!input) {
      output = "";
      return;
    }

    try {
      output = cronstrue.toString(input, { locale: "en" });
    } catch (e) {
      output = "Invalid cron expression.";
    }
  }

  $: processCron(), input;
</script>

<main class="tool-container">
  <div class="pane input-pane">
    <div class="pane-header">CRON EXPRESSION</div>
    <textarea
      class="editor source-editor"
      placeholder="*/15 0 * * 1-5"
      spellcheck="false"
      bind:value={input}
    ></textarea>
  </div>

  <div class="divider"></div>

  <div class="pane output-pane">
    <div class="pane-header result-header">
      <span>HUMAN READABLE</span>
      <button
        class="action-btn"
        on:click={() => navigator.clipboard.writeText(output)}>Copy</button
      >
    </div>

    <div class="result-display">
      {output}
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
    flex: 1;
    background: transparent;
    border: none;
    color: var(--text-main);
    padding: 15px;
    resize: none;
    outline: none;
    font-family: inherit;
    font-size: 1.2rem;
    line-height: 1.6;
  }

  .source-editor {
    background-color: rgba(0, 0, 0, 0.1);
  }

  .result-display {
    flex: 1;
    padding: 20px;
    font-size: 1.5rem;
    color: var(--accent);
    display: flex;
    align-items: center;
    justify-content: center;
    text-align: center;
    word-break: break-word;
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
