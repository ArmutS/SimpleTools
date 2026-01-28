<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";

  let files: string[] = [];
  let findPattern = "";
  let replaceWith = "";
  let useRegex = false;
  let previews: any[] = [];
  let isProcessing = false;

  async function selectFiles() {
    const selected = await open({
      multiple: true,
      directory: false,
    });

    if (selected) {
      files = Array.isArray(selected) ? selected : [selected];
      previews = [];
    }
  }

  async function generatePreview() {
    if (files.length === 0 || !findPattern) {
      alert("Please select files and enter a pattern");
      return;
    }

    isProcessing = true;
    try {
      previews = await invoke("file_rename_batch", {
        request: {
          files,
          find_pattern: findPattern,
          replace_with: replaceWith,
          use_regex: useRegex,
        },
        previewOnly: true,
      });
    } catch (error) {
      alert(`Error: ${error}`);
    } finally {
      isProcessing = false;
    }
  }

  async function applyRename() {
    if (previews.length === 0) {
      alert("Please generate preview first");
      return;
    }

    isProcessing = true;
    try {
      await invoke("file_rename_batch", {
        request: {
          files,
          find_pattern: findPattern,
          replace_with: replaceWith,
          use_regex: useRegex,
        },
        previewOnly: false,
      });
      alert("Files renamed successfully!");
      reset();
    } catch (error) {
      alert(`Error: ${error}`);
    } finally {
      isProcessing = false;
    }
  }

  function reset() {
    files = [];
    findPattern = "";
    replaceWith = "";
    previews = [];
  }
</script>

<main class="container">
  <div class="content">
    <div class="header">
      <div class="icon-wrapper">
        <i class="nf-md-rename_box rename-icon"></i>
      </div>
      <h1>File Renamer</h1>
      <p class="subtitle">
        Batch rename files with regex support and live preview
      </p>
    </div>

    <div class="input-section">
      <div class="file-select-area">
        <button class="select-btn" on:click={selectFiles}>
          <i class="nf-md-file_multiple"></i>
          <span>Select Files</span>
        </button>
        {#if files.length > 0}
          <div class="file-count">
            <i class="nf-md-check_circle"></i>
            <span>{files.length} file(s) selected</span>
          </div>
        {/if}
      </div>

      <div class="pattern-grid">
        <div class="input-group">
          <label for="find">
            <i class="nf-md-magnify"></i>
            <span>Find Pattern</span>
          </label>
          <input
            id="find"
            type="text"
            bind:value={findPattern}
            placeholder="Enter pattern to find..."
          />
        </div>

        <div class="input-group">
          <label for="replace">
            <i class="nf-md-find_replace"></i>
            <span>Replace With</span>
          </label>
          <input
            id="replace"
            type="text"
            bind:value={replaceWith}
            placeholder="Enter replacement..."
          />
        </div>
      </div>

      <div class="checkbox-wrapper">
        <input type="checkbox" id="regex" bind:checked={useRegex} />
        <label for="regex">
          <i class="nf-md-regex"></i>
          <span>Use Regular Expression</span>
        </label>
      </div>

      <div class="actions">
        <button
          class="btn-primary"
          on:click={generatePreview}
          disabled={isProcessing}
        >
          <i class="nf-md-eye"></i>
          <span>{isProcessing ? "Processing..." : "Preview Changes"}</span>
        </button>
        <button class="btn-secondary" on:click={reset}>
          <i class="nf-md-refresh"></i>
          <span>Reset</span>
        </button>
      </div>
    </div>

    {#if previews.length > 0}
      <div class="preview-section">
        <div class="preview-header">
          <h2>
            <i class="nf-md-file_document_multiple"></i>
            <span>Preview ({previews.length} files)</span>
          </h2>
        </div>

        <div class="preview-table">
          <div class="table-header">
            <div class="col">Old Name</div>
            <div class="col-arrow">→</div>
            <div class="col">New Name</div>
          </div>
          <div class="table-body">
            {#each previews as preview, i}
              <div class="table-row" style="animation-delay: {i * 50}ms">
                <div class="col old-name">{preview.old_name}</div>
                <div class="col-arrow">
                  <i class="nf-md-arrow_right"></i>
                </div>
                <div class="col new-name">{preview.new_name}</div>
              </div>
            {/each}
          </div>
        </div>

        <button
          class="btn-apply"
          on:click={applyRename}
          disabled={isProcessing}
        >
          <i class="nf-md-check_all"></i>
          <span>Apply Rename</span>
        </button>
      </div>
    {/if}
  </div>
</main>

<style>
  .container {
    min-height: 100vh;
    padding: 3rem 2rem;
    background: linear-gradient(135deg, #1e1e2e 0%, #181825 100%);
  }

  .content {
    max-width: 1100px;
    margin: 0 auto;
  }

  .header {
    text-align: center;
    margin-bottom: 3rem;
  }

  .icon-wrapper {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 80px;
    height: 80px;
    background: linear-gradient(135deg, #f9e2af 0%, #fab387 100%);
    border-radius: 20px;
    margin-bottom: 1.5rem;
    box-shadow: 0 8px 24px rgba(250, 179, 135, 0.4);
    animation: float 3s ease-in-out infinite;
  }

  @keyframes float {
    0%,
    100% {
      transform: translateY(0px);
    }
    50% {
      transform: translateY(-10px);
    }
  }

  .rename-icon {
    font-size: 3rem;
    color: #1e1e2e;
  }

  h1 {
    background: linear-gradient(135deg, #f9e2af 0%, #fab387 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    font-size: 2.5rem;
    margin-bottom: 0.5rem;
    font-weight: bold;
  }

  .subtitle {
    color: #a6adc8;
    font-size: 1.1rem;
  }

  .input-section {
    background: rgba(24, 24, 37, 0.7);
    backdrop-filter: blur(10px);
    padding: 2.5rem;
    border-radius: 20px;
    margin-bottom: 2rem;
    border: 1px solid rgba(249, 226, 175, 0.1);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
  }

  .file-select-area {
    text-align: center;
    margin-bottom: 2rem;
  }

  .select-btn {
    padding: 1.25rem 2.5rem;
    background: linear-gradient(135deg, #f9e2af 0%, #fab387 100%);
    color: #1e1e2e;
    border: none;
    border-radius: 12px;
    cursor: pointer;
    font-size: 1rem;
    font-weight: 600;
    display: inline-flex;
    align-items: center;
    gap: 0.75rem;
    transition: all 0.3s ease;
    box-shadow: 0 4px 12px rgba(250, 179, 135, 0.3);
  }

  .select-btn:hover {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(250, 179, 135, 0.5);
  }

  .select-btn i {
    font-size: 1.3rem;
  }

  .file-count {
    margin-top: 1.5rem;
    padding: 1rem 1.5rem;
    background: #11111b;
    border-radius: 10px;
    display: inline-flex;
    align-items: center;
    gap: 0.75rem;
    border: 1px solid #313244;
  }

  .file-count i {
    color: #a6e3a1;
    font-size: 1.2rem;
  }

  .file-count span {
    color: #cdd6f4;
    font-weight: 500;
  }

  .pattern-grid {
    display: grid;
    gap: 1.5rem;
    margin-bottom: 1.5rem;
  }

  .input-group {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  label {
    color: #f9e2af;
    font-size: 0.95rem;
    font-weight: 600;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  label i {
    font-size: 1.1rem;
  }

  input[type="text"] {
    padding: 1rem 1.25rem;
    background: #11111b;
    border: 2px solid #313244;
    border-radius: 10px;
    color: #cdd6f4;
    font-family: "Consolas", "Monaco", monospace;
    font-size: 1rem;
    transition: all 0.3s ease;
  }

  input[type="text"]:focus {
    outline: none;
    border-color: #f9e2af;
    box-shadow: 0 0 0 3px rgba(249, 226, 175, 0.1);
  }

  .checkbox-wrapper {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem;
    background: rgba(17, 17, 27, 0.5);
    border-radius: 10px;
    margin-bottom: 1.5rem;
  }

  input[type="checkbox"] {
    width: 20px;
    height: 20px;
    cursor: pointer;
  }

  .checkbox-wrapper label {
    cursor: pointer;
    margin: 0;
    color: #cdd6f4;
  }

  .actions {
    display: grid;
    grid-template-columns: 2fr 1fr;
    gap: 1rem;
  }

  .btn-primary,
  .btn-secondary {
    padding: 1rem 2rem;
    border: none;
    border-radius: 12px;
    cursor: pointer;
    font-size: 1rem;
    font-weight: 600;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.75rem;
    transition: all 0.3s ease;
  }

  .btn-primary {
    background: linear-gradient(135deg, #f9e2af 0%, #fab387 100%);
    color: #1e1e2e;
    box-shadow: 0 4px 12px rgba(250, 179, 135, 0.3);
  }

  .btn-primary:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(250, 179, 135, 0.5);
  }

  .btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-secondary {
    background: rgba(49, 50, 68, 0.5);
    color: #cdd6f4;
    border: 2px solid #313244;
  }

  .btn-secondary:hover {
    background: rgba(49, 50, 68, 0.8);
    border-color: #f9e2af;
  }

  .preview-section {
    background: rgba(24, 24, 37, 0.7);
    backdrop-filter: blur(10px);
    padding: 2.5rem;
    border-radius: 20px;
    border: 1px solid rgba(249, 226, 175, 0.1);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
    animation: slideUp 0.4s ease-out;
  }

  @keyframes slideUp {
    from {
      opacity: 0;
      transform: translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .preview-header h2 {
    color: #cdd6f4;
    margin-bottom: 2rem;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    font-size: 1.5rem;
  }

  .preview-header h2 i {
    color: #f9e2af;
    font-size: 1.8rem;
  }

  .preview-table {
    background: #11111b;
    border-radius: 12px;
    overflow: hidden;
    margin-bottom: 1.5rem;
    border: 2px solid #313244;
  }

  .table-header {
    display: grid;
    grid-template-columns: 1fr 60px 1fr;
    gap: 1rem;
    padding: 1.25rem 1.5rem;
    background: linear-gradient(
      135deg,
      rgba(249, 226, 175, 0.2),
      rgba(250, 179, 135, 0.2)
    );
    color: #f9e2af;
    font-weight: bold;
    font-size: 0.95rem;
  }

  .table-body {
    max-height: 400px;
    overflow-y: auto;
  }

  .table-row {
    display: grid;
    grid-template-columns: 1fr 60px 1fr;
    gap: 1rem;
    padding: 1.25rem 1.5rem;
    border-bottom: 1px solid #313244;
    transition: all 0.2s ease;
    animation: fadeIn 0.3s ease-out forwards;
    opacity: 0;
  }

  @keyframes fadeIn {
    to {
      opacity: 1;
    }
  }

  .table-row:last-child {
    border-bottom: none;
  }

  .table-row:hover {
    background: rgba(249, 226, 175, 0.05);
  }

  .col {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .old-name {
    color: #a6adc8;
  }

  .col-arrow {
    text-align: center;
    color: #f9e2af;
    font-weight: bold;
  }

  .col-arrow i {
    font-size: 1.2rem;
  }

  .new-name {
    color: #cdd6f4;
    font-weight: 600;
  }

  .btn-apply {
    width: 100%;
    padding: 1.25rem;
    background: linear-gradient(135deg, #a6e3a1 0%, #94e2d5 100%);
    color: #1e1e2e;
    border: none;
    border-radius: 12px;
    cursor: pointer;
    font-size: 1.1rem;
    font-weight: bold;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.75rem;
    transition: all 0.3s ease;
    box-shadow: 0 4px 12px rgba(166, 227, 161, 0.3);
  }

  .btn-apply:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(166, 227, 161, 0.5);
  }

  .btn-apply:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-apply i {
    font-size: 1.3rem;
  }
</style>
