<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";

  let inputText = "";
  let selectedFile = "";
  let isFileMode = false;
  let result: any = null;
  let isProcessing = false;

  async function selectFile() {
    const selected = await open({
      multiple: false,
      directory: false,
    });

    if (selected) {
      selectedFile = selected as string;
      isFileMode = true;
    }
  }

  async function generateHash() {
    if (!inputText && !selectedFile) {
      alert("Please enter text or select a file");
      return;
    }

    isProcessing = true;
    try {
      const input = isFileMode ? selectedFile : inputText;
      result = await invoke("hash_generate", {
        input: input,
        isFile: isFileMode,
      });
    } catch (error) {
      alert(`Error: ${error}`);
    } finally {
      isProcessing = false;
    }
  }

  function copyToClipboard(text: string) {
    navigator.clipboard.writeText(text);
  }

  function reset() {
    inputText = "";
    selectedFile = "";
    result = null;
    isFileMode = false;
  }
</script>

<main class="container">
  <div class="content">
    <div class="header">
      <div class="icon-wrapper">
        <i class="nf-md-pound hash-icon"></i>
      </div>
      <h1>Hash Generator</h1>
      <p class="subtitle">
        Generate MD5, SHA256, and SHA512 hashes for text or files
      </p>
    </div>

    <div class="input-section">
      <div class="mode-toggle">
        <button
          class="mode-btn {!isFileMode ? 'active' : ''}"
          on:click={() => {
            isFileMode = false;
            selectedFile = "";
          }}
        >
          <i class="nf-md-format_text"></i>
          <span>Text Input</span>
        </button>
        <button
          class="mode-btn {isFileMode ? 'active' : ''}"
          on:click={() => {
            isFileMode = true;
            inputText = "";
          }}
        >
          <i class="nf-md-file"></i>
          <span>File Input</span>
        </button>
      </div>

      {#if !isFileMode}
        <div class="input-wrapper">
          <textarea
            bind:value={inputText}
            placeholder="Enter text to hash..."
            rows="8"
          ></textarea>
        </div>
      {:else}
        <div class="file-select">
          <button class="select-file-btn" on:click={selectFile}>
            <i class="nf-md-folder_open"></i>
            <span>Select File</span>
          </button>
          {#if selectedFile}
            <div class="selected-file">
              <i class="nf-md-file_check"></i>
              <span>{selectedFile}</span>
            </div>
          {/if}
        </div>
      {/if}

      <div class="actions">
        <button
          class="btn-primary"
          on:click={generateHash}
          disabled={isProcessing}
        >
          <i class="nf-md-flash"></i>
          <span>{isProcessing ? "Processing..." : "Generate Hash"}</span>
        </button>
        <button class="btn-secondary" on:click={reset}>
          <i class="nf-md-refresh"></i>
          <span>Reset</span>
        </button>
      </div>
    </div>

    {#if result}
      <div class="result-section">
        <h2><i class="nf-md-check_decagram"></i> Results</h2>

        <div class="hash-result md5">
          <div class="hash-header">
            <div class="hash-label">
              <i class="nf-md-numeric_1_box"></i>
              <span>MD5</span>
            </div>
            <button
              class="copy-btn"
              on:click={() => copyToClipboard(result.md5)}
              aria-label="Copy MD5 hash"
            >
              <i class="nf-md-content_copy"></i>
              <span>Copy</span>
            </button>
          </div>
          <div class="hash-value">
            <code>{result.md5}</code>
          </div>
        </div>

        <div class="hash-result sha256">
          <div class="hash-header">
            <div class="hash-label">
              <i class="nf-md-numeric_2_box"></i>
              <span>SHA256</span>
            </div>
            <button
              class="copy-btn"
              on:click={() => copyToClipboard(result.sha256)}
              aria-label="Copy SHA256 hash"
            >
              <i class="nf-md-content_copy"></i>
              <span>Copy</span>
            </button>
          </div>
          <div class="hash-value">
            <code>{result.sha256}</code>
          </div>
        </div>

        <div class="hash-result sha512">
          <div class="hash-header">
            <div class="hash-label">
              <i class="nf-md-numeric_3_box"></i>
              <span>SHA512</span>
            </div>
            <button
              class="copy-btn"
              on:click={() => copyToClipboard(result.sha512)}
              aria-label="Copy SHA512 hash"
            >
              <i class="nf-md-content_copy"></i>
              <span>Copy</span>
            </button>
          </div>
          <div class="hash-value">
            <code>{result.sha512}</code>
          </div>
        </div>
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
    max-width: 900px;
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
    background: linear-gradient(135deg, #89b4fa 0%, #74c7ec 100%);
    border-radius: 20px;
    margin-bottom: 1.5rem;
    box-shadow: 0 8px 24px rgba(137, 180, 250, 0.3);
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

  .hash-icon {
    font-size: 3rem;
    color: #1e1e2e;
  }

  h1 {
    background: linear-gradient(135deg, #89b4fa 0%, #74c7ec 100%);
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
    margin-bottom: 0;
  }

  .input-section {
    background: rgba(24, 24, 37, 0.7);
    backdrop-filter: blur(10px);
    padding: 2.5rem;
    border-radius: 20px;
    margin-bottom: 2rem;
    border: 1px solid rgba(137, 180, 250, 0.1);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
  }

  .mode-toggle {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
    margin-bottom: 2rem;
    background: #11111b;
    padding: 0.5rem;
    border-radius: 12px;
  }

  .mode-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.75rem;
    padding: 1rem;
    background: transparent;
    border: 2px solid transparent;
    border-radius: 10px;
    color: #a6adc8;
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    font-size: 1rem;
    font-weight: 500;
  }

  .mode-btn i {
    font-size: 1.3rem;
  }

  .mode-btn:hover {
    background: rgba(137, 180, 250, 0.1);
    color: #cdd6f4;
  }

  .mode-btn.active {
    background: linear-gradient(135deg, #89b4fa 0%, #74c7ec 100%);
    color: #1e1e2e;
    border-color: transparent;
    box-shadow: 0 4px 12px rgba(137, 180, 250, 0.4);
    transform: translateY(-2px);
  }

  .input-wrapper {
    margin-bottom: 1.5rem;
  }

  textarea {
    width: 100%;
    padding: 1.25rem;
    background: #11111b;
    border: 2px solid #313244;
    border-radius: 12px;
    color: #cdd6f4;
    font-family: "Consolas", "Monaco", monospace;
    font-size: 1rem;
    resize: vertical;
    transition: all 0.3s ease;
  }

  textarea:focus {
    outline: none;
    border-color: #89b4fa;
    box-shadow: 0 0 0 3px rgba(137, 180, 250, 0.1);
  }

  .file-select {
    padding: 2rem;
    text-align: center;
  }

  .select-file-btn {
    padding: 1.25rem 2.5rem;
    background: linear-gradient(135deg, #89b4fa 0%, #74c7ec 100%);
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
    box-shadow: 0 4px 12px rgba(137, 180, 250, 0.3);
  }

  .select-file-btn:hover {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(137, 180, 250, 0.4);
  }

  .select-file-btn:active {
    transform: translateY(0);
  }

  .select-file-btn i {
    font-size: 1.3rem;
  }

  .selected-file {
    margin-top: 1.5rem;
    padding: 1rem 1.5rem;
    background: #11111b;
    border-radius: 10px;
    color: #cdd6f4;
    display: inline-flex;
    align-items: center;
    gap: 0.75rem;
    border: 1px solid #313244;
  }

  .selected-file i {
    color: #a6e3a1;
    font-size: 1.2rem;
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
    background: linear-gradient(135deg, #89b4fa 0%, #74c7ec 100%);
    color: #1e1e2e;
    box-shadow: 0 4px 12px rgba(137, 180, 250, 0.3);
  }

  .btn-primary:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(137, 180, 250, 0.4);
  }

  .btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-primary i,
  .btn-secondary i {
    font-size: 1.2rem;
  }

  .btn-secondary {
    background: rgba(49, 50, 68, 0.5);
    color: #cdd6f4;
    border: 2px solid #313244;
  }

  .btn-secondary:hover {
    background: rgba(49, 50, 68, 0.8);
    border-color: #89b4fa;
  }

  .result-section {
    background: rgba(24, 24, 37, 0.7);
    backdrop-filter: blur(10px);
    padding: 2.5rem;
    border-radius: 20px;
    border: 1px solid rgba(137, 180, 250, 0.1);
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

  .result-section h2 {
    color: #cdd6f4;
    margin-bottom: 2rem;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    font-size: 1.5rem;
  }

  .result-section h2 i {
    color: #a6e3a1;
    font-size: 1.8rem;
  }

  .hash-result {
    margin-bottom: 1.5rem;
    padding: 1.5rem;
    background: #11111b;
    border-radius: 12px;
    border: 2px solid #313244;
    transition: all 0.3s ease;
  }

  .hash-result:hover {
    border-color: #89b4fa;
    box-shadow: 0 4px 12px rgba(137, 180, 250, 0.2);
    transform: translateX(4px);
  }

  .hash-result:last-child {
    margin-bottom: 0;
  }

  .hash-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .hash-label {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    color: #89b4fa;
    font-weight: bold;
    font-size: 1.1rem;
  }

  .hash-label i {
    font-size: 1.5rem;
  }

  .hash-value {
    background: rgba(17, 17, 27, 0.5);
    padding: 1.25rem;
    border-radius: 10px;
    border: 1px solid #313244;
  }

  code {
    color: #cdd6f4;
    font-family: "Consolas", "Monaco", monospace;
    font-size: 0.95rem;
    word-break: break-all;
    line-height: 1.6;
  }

  .copy-btn {
    padding: 0.6rem 1.25rem;
    background: rgba(137, 180, 250, 0.1);
    border: 1px solid rgba(137, 180, 250, 0.3);
    border-radius: 8px;
    color: #89b4fa;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    transition: all 0.2s ease;
    font-size: 0.9rem;
    font-weight: 500;
  }

  .copy-btn:hover {
    background: #89b4fa;
    color: #1e1e2e;
    border-color: #89b4fa;
    transform: scale(1.05);
  }

  .copy-btn:active {
    transform: scale(0.95);
  }

  .copy-btn i {
    font-size: 1.1rem;
  }
</style>
