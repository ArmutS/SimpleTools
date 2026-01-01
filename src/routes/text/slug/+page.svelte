<script lang="ts">
  let input = "";
  let output = "";

  function toSlug(text: string) {
    return text
      .toString()
      .toLowerCase()
      .trim()
      .replace(/\s+/g, "-") // Replace spaces with -
      .replace(/&/g, "-and-") // Replace & with 'and'
      .replace(/[^\w\-]+/g, "") // Remove all non-word chars
      .replace(/\-\-+/g, "-"); // Replace multiple - with single -
  }

  $: output = toSlug(input);
</script>

<main class="tool-container">
  <div class="pane input-pane">
    <div class="pane-header">INPUT (Title)</div>
    <textarea
      class="editor source-editor"
      placeholder="Enter title here..."
      spellcheck="false"
      bind:value={input}
    ></textarea>
  </div>

  <div class="divider"></div>

  <div class="pane output-pane">
    <div class="pane-header result-header">
      <span>SLUG</span>
      <button
        class="action-btn"
        on:click={() => navigator.clipboard.writeText(output)}>Copy</button
      >
    </div>

    <textarea
      class="editor result-editor"
      placeholder="slug-will-appear-here"
      readonly
      bind:value={output}
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
