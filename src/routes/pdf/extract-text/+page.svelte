<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onMount, onDestroy } from "svelte";

  // State
  let selectedFile: string = "";
  let isProcessing: boolean = false;
  let status: string = "Hazır";
  let isDragging: boolean = false;
  let showSuccess: boolean = false;

  // Results
  let extractedText: string = "";
  let pageCount: number = 0;

  // Dosya seçme
  async function selectFile() {
    try {
      const selected = await open({
        multiple: false,
        filters: [
          {
            name: "PDF",
            extensions: ["pdf"],
          },
        ],
      });

      if (selected && !Array.isArray(selected)) {
        selectedFile = selected;
        // Reset results on new file
        extractedText = "";
        showSuccess = false;
      }
    } catch (error) {
      status = `Hata: ${error}`;
    }
  }

  // Extract
  async function extractText() {
    if (!selectedFile) {
      status = "Bir PDF dosyası seçmelisiniz!";
      return;
    }

    isProcessing = true;
    status = "Metin çıkarılıyor...";
    showSuccess = false;
    extractedText = "";

    try {
      const result = await invoke<{ text: string; page_count: number }>(
        "pdf_extract_text",
        {
          request: {
            file_path: selectedFile,
            pages: null, // All pages
          },
        }
      );

      extractedText = result.text;
      pageCount = result.page_count;

      status = "Başarıyla tamamlandı!";
      showSuccess = true;
    } catch (error) {
      status = `Hata: ${error}`;
      showSuccess = false;
    } finally {
      isProcessing = false;
    }
  }

  async function copyToClipboard() {
    if (extractedText) {
      try {
        await navigator.clipboard.writeText(extractedText);
        status = "Metin panoya kopyalandı!";
        setTimeout(() => {
          if (showSuccess) status = "Başarıyla tamamlandı!";
        }, 2000);
      } catch (e) {
        status = "Kopyalama başarısız!";
      }
    }
  }

  // Drag & Drop
  let unlistenDrop: () => void;

  onMount(async () => {
    unlistenDrop = await getCurrentWindow().listen(
      "tauri://drag-drop",
      (event) => {
        const payload = event.payload as { paths: string[] };
        if (payload.paths && payload.paths.length > 0) {
          const pdf = payload.paths.find((p) =>
            p.toLowerCase().endsWith(".pdf")
          );
          if (pdf) {
            selectedFile = pdf;
            extractedText = "";
            showSuccess = false;
          } else {
            status = "Lütfen bir PDF dosyası sürükleyin.";
          }
        }
      }
    );
  });

  onDestroy(() => {
    if (unlistenDrop) {
      unlistenDrop();
    }
  });

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    isDragging = true;
  }

  function handleDragLeave() {
    isDragging = false;
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDragging = false;
  }
</script>

<div class="layout">
  <!-- Left Panel -->
  <div class="panel left-panel">
    <div class="header">
      <h2>Extract Text</h2>
      <p>PDF içeriğindeki metinleri çıkarın</p>
    </div>

    <div
      class="file-container"
      class:dragging={isDragging}
      on:dragover={handleDragOver}
      on:dragleave={handleDragLeave}
      on:drop={handleDrop}
    >
      {#if selectedFile}
        <div class="selected-file-display">
          <i class="nf-md-file_pdf_box file-icon"></i>
          <div class="file-info">
            <span class="file-path">{selectedFile}</span>
          </div>
          <button class="remove-btn" on:click={() => (selectedFile = "")}>
            <i class="nf-md-close"></i>
          </button>
        </div>
      {:else}
        <div class="empty-state" on:click={selectFile}>
          <i class="nf-md-file_upload"></i>
          <p>PDF dosyasını buraya sürükleyin veya seçin</p>
        </div>
      {/if}
    </div>

    <div class="action-block">
      <button
        class="merge-btn"
        on:click={extractText}
        disabled={!selectedFile || isProcessing}
      >
        {#if isProcessing}
          <i class="nf-md-loading nf-spin"></i> İşleniyor...
        {:else}
          <i class="nf-md-text_box"></i> METNİ ÇIKAR
        {/if}
      </button>

      {#if status && !status.includes("Başarılı") && !status.includes("Hazır") && !status.includes("İşleniyor") && !status.includes("Kopyalandı")}
        <div class="error-message">
          {status}
        </div>
      {/if}

      {#if showSuccess || status.includes("Kopyalandı")}
        <div class="success-message">
          <i class="nf-md-check_circle"></i>
          <span
            >{status.includes("Kopyalandı")
              ? "Kopyalandı"
              : "İşlem Başarılı!"}</span
          >
        </div>
      {/if}
    </div>
  </div>

  <!-- Right Panel: Result -->
  <div class="panel right-panel">
    {#if extractedText}
      <div class="result-container">
        <div class="result-header">
          <span>{pageCount} sayfa tarandı</span>
          <button class="copy-btn" on:click={copyToClipboard}>
            <i class="nf-md-content_copy"></i> Kopyala
          </button>
        </div>
        <textarea readonly bind:value={extractedText}></textarea>
      </div>
    {:else}
      <div class="preview-placeholder">
        <i class="nf-md-text_subject"></i>
        <h3>Extract Text</h3>
        <p>Çıkarılan metin burada görünecektir.</p>
      </div>
    {/if}
  </div>
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    overflow: hidden;
  }

  .layout {
    display: flex;
    width: 100vw;
    height: 100vh;
    background-color: var(--bg-app, #1e1e2e);
    color: var(--text-main, #cdd6f4);
  }

  .panel {
    display: flex;
    flex-direction: column;
    padding: 1.5rem;
    height: 100%;
  }

  .left-panel {
    width: 40%;
    border-right: 1px solid var(--border-color, #45475a);
    gap: 1.5rem;
  }

  .right-panel {
    width: 60%;
    background-color: rgba(0, 0, 0, 0.2);
    overflow: hidden; /* For textarea scroll */
  }

  .header h2 {
    margin: 0;
    font-size: 1.5rem;
    color: var(--accent, #89b4fa);
  }

  .header p {
    margin: 0.25rem 0 0;
    color: var(--text-muted, #a6adc8);
    font-size: 0.9rem;
  }

  .file-container {
    height: 150px;
    display: flex;
    flex-direction: column;
    background: var(--bg-input, #313244);
    border-radius: 8px;
    border: 2px dashed var(--border-color, #45475a);
    overflow: hidden;
    transition: all 0.2s;
    justify-content: center;
  }

  .file-container.dragging {
    border-color: var(--accent, #89b4fa);
    background: rgba(137, 180, 250, 0.1);
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
    gap: 0.5rem;
    cursor: pointer;
    height: 100%;
  }

  .empty-state:hover {
    color: var(--accent);
  }

  .empty-state i {
    font-size: 2.5rem;
  }

  .selected-file-display {
    display: flex;
    align-items: center;
    padding: 1rem;
    gap: 1rem;
    background: rgba(0, 0, 0, 0.2);
    height: 100%;
  }

  .file-icon {
    font-size: 2.5rem;
    color: #f38ba8;
  }

  .file-info {
    flex: 1;
    overflow: hidden;
  }

  .file-path {
    font-family: monospace;
    font-size: 0.9rem;
    word-break: break-all;
  }

  .remove-btn {
    background: transparent;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    font-size: 1.2rem;
  }
  .remove-btn:hover {
    color: #f38ba8;
  }

  .action-block {
    margin-top: auto;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .merge-btn {
    width: 100%;
    padding: 1rem;
    background: var(--accent);
    color: var(--bg-app);
    border: none;
    border-radius: 8px;
    font-size: 1.1rem;
    font-weight: bold;
    cursor: pointer;
    text-transform: uppercase;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
  }

  .merge-btn:hover:not(:disabled) {
    transform: translateY(-2px);
  }

  .merge-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    background: var(--bg-input);
    color: var(--text-muted);
  }

  .success-message {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    color: #a6e3a1;
    font-weight: bold;
  }

  .error-message {
    color: #f38ba8;
    text-align: center;
    font-size: 0.9rem;
    padding: 0.5rem;
    background: rgba(243, 139, 168, 0.1);
    border-radius: 4px;
  }

  .preview-placeholder {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 1rem;
    color: var(--text-muted);
    text-align: center;
    height: 100%;
  }

  .preview-placeholder i {
    font-size: 4rem;
    opacity: 0.3;
  }

  .result-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    gap: 0.5rem;
  }

  .result-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid var(--border-color);
  }

  .copy-btn {
    background: var(--bg-input);
    border: 1px solid var(--border-color);
    color: var(--text-main);
    padding: 0.3rem 0.8rem;
    border-radius: 4px;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  .copy-btn:hover {
    border-color: var(--accent);
  }

  textarea {
    flex: 1;
    background: var(--bg-input);
    border: 1px solid var(--border-color);
    padding: 1rem;
    border-radius: 8px;
    color: var(--text-main);
    resize: none;
    font-family: monospace;
    line-height: 1.5;
  }
  textarea:focus {
    outline: none;
    border-color: var(--accent);
  }
</style>
