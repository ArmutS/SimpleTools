<script lang="ts">
  import { marked } from "marked";

  let input = "# Hello World\n\nStart typing markdown...";
  let output = "";

  async function processMarkdown() {
    if (!input) {
      output = "";
      return;
    }
    try {
      output = await marked.parse(input);
    } catch (e) {
      output = "Error parsing markdown";
    }
  }

  $: processMarkdown(), input;
</script>

<main class="tool-container">
  <div class="pane input-pane">
    <div class="pane-header">MARKDOWN INPUT</div>
    <textarea class="editor source-editor" spellcheck="false" bind:value={input}
    ></textarea>
  </div>

  <div class="divider"></div>

  <div class="pane output-pane">
    <div class="pane-header result-header">
      <span>PREVIEW</span>
    </div>

    <div class="preview-area">
      {@html output}
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
    font-size: 0.9rem;
    line-height: 1.6;
  }

  .source-editor {
    background-color: rgba(0, 0, 0, 0.1);
  }

  .preview-area {
    flex: 1;
    padding: 20px;
    overflow-y: auto;
    color: var(--text-main);
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
      "Helvetica Neue", Arial, sans-serif;
  }

  /* Basic Markdown Styles for Preview */
  :global(.preview-area h1),
  :global(.preview-area h2),
  :global(.preview-area h3) {
    color: var(--accent);
    border-bottom: 1px solid var(--border-color);
    padding-bottom: 0.3em;
  }
  :global(.preview-area a) {
    color: var(--accent);
  }
  :global(.preview-area code) {
    background: var(--bg-input);
    padding: 2px 4px;
    border-radius: 4px;
    font-family: "Consolas", monospace;
  }
  :global(.preview-area pre) {
    background: var(--bg-input);
    padding: 10px;
    border-radius: 5px;
    overflow-x: auto;
  }
  :global(.preview-area blockquote) {
    border-left: 4px solid var(--border-color);
    margin: 0;
    padding-left: 10px;
    color: var(--text-muted);
  }
  :global(.preview-area img) {
    max-width: 100%;
  }
</style>
