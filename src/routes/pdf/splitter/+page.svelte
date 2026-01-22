<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onMount, onDestroy } from "svelte";

  // State
  let selectedFile: string = "";
  let outputDir: string = "";
  let isProcessing: boolean = false;
  let status: string = "Hazır";
  let isDragging: boolean = false;
  let showSuccess: boolean = false;

  // Split Settings
  let mode: "individual" | "range" = "individual";
  let startPage: number = 1;
  let endPage: number = 1;

  // Dosya seçme
  async function selectFile() {
    try {
      await getCurrentWindow().hide();
      const selected = await open({
        multiple: false,
        filters: [
          {
            name: "PDF",
            extensions: ["pdf"],
          },
        ],
      });
      await getCurrentWindow().show();
      await getCurrentWindow().setFocus();

      if (selected && !Array.isArray(selected)) {
        selectedFile = selected;
        // Default output dir to same as file
        if (!outputDir) {
          outputDir = selectedFile.substring(0, selectedFile.lastIndexOf("/"));
        }
      }
    } catch (error) {
      status = `Hata: ${error}`;
    }
  }

  // Çıktı klasörü seçme
  async function selectOutputDir() {
    try {
      await getCurrentWindow().hide();
      const selected = await open({
        directory: true,
      });
      await getCurrentWindow().show();
      await getCurrentWindow().setFocus();

      if (selected && !Array.isArray(selected)) {
        outputDir = selected;
      }
    } catch (error) {
      status = `Hata: ${error}`;
    }
  }

  // Split Action
  async function splitPdf() {
    if (!selectedFile) {
      status = "Bir PDF dosyası seçmelisiniz!";
      return;
    }

    if (!outputDir) {
      status = "Çıktı klasörü seçmelisiniz!";
      return;
    }

    if (mode === "range" && startPage > endPage) {
      status = "Başlangıç sayfası bitiş sayfasından büyük olamaz!";
      return;
    }

    isProcessing = true;
    status = "Ayrıştırılıyor...";
    showSuccess = false;

    try {
      await invoke("pdf_split", {
        request: {
          file_path: selectedFile,
          mode: mode,
          start_page: mode === "range" ? startPage : null,
          end_page: mode === "range" ? endPage : null,
          output_dir: outputDir,
        },
      });

      status = "Başarıyla tamamlandı!";
      showSuccess = true;
    } catch (error) {
      status = `Hata: ${error}`;
      showSuccess = false;
    } finally {
      isProcessing = false;
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
            p.toLowerCase().endsWith(".pdf"),
          );
          if (pdf) {
            selectedFile = pdf;
            if (!outputDir) {
              outputDir = pdf.substring(0, pdf.lastIndexOf("/"));
            }
          } else {
            status = "Lütfen bir PDF dosyası sürükleyin.";
          }
        }
      },
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

  async function openOutputDir() {
    if (outputDir) {
      await invoke("open_folder", { path: outputDir });
    }
  }
</script>

<div class="layout">
  <!-- Left Panel -->
  <div class="panel left-panel">
    <div class="header">
      <h2>PDF Splitter</h2>
      <p>PDF dosyasını sayfalara ayırın veya belirli aralığı alın</p>
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

    <div class="controls-block">
      <label>Ayarlar</label>

      <div class="radio-group">
        <label class="radio-label">
          <input type="radio" bind:group={mode} value="individual" />
          <span>Her sayfayı ayrı dosya yap</span>
        </label>
        <label class="radio-label">
          <input type="radio" bind:group={mode} value="range" />
          <span>Sayfa Aralığını Çıkar</span>
        </label>
      </div>

      {#if mode === "range"}
        <div class="range-inputs">
          <div class="input-group">
            <label for="start">Başlangıç:</label>
            <input id="start" type="number" min="1" bind:value={startPage} />
          </div>
          <div class="input-group">
            <label for="end">Bitiş:</label>
            <input id="end" type="number" min="1" bind:value={endPage} />
          </div>
        </div>
      {/if}

      <div class="input-group">
        <label for="output-path">Çıktı Klasörü:</label>
        <div class="path-select">
          <input
            id="output-path"
            type="text"
            bind:value={outputDir}
            placeholder="/path/to/folder"
          />
          <button class="icon-btn" on:click={selectOutputDir}>
            <i class="nf-md-folder_open"></i>
          </button>
        </div>
      </div>
    </div>

    <div class="action-block">
      <button
        class="merge-btn"
        on:click={splitPdf}
        disabled={!selectedFile || !outputDir || isProcessing}
      >
        {#if isProcessing}
          <i class="nf-md-loading nf-spin"></i> İşleniyor...
        {:else}
          <i class="nf-md-call_split"></i> AYRIŞTIR
        {/if}
      </button>

      {#if showSuccess}
        <div class="success-message">
          <i class="nf-md-check_circle"></i>
          <span>İşlem Başarılı!</span>
        </div>
        <button class="success-btn" on:click={openOutputDir}>Klasörü Aç</button>
      {/if}

      {#if status && !status.includes("Başarılı") && !status.includes("Hazır") && !status.includes("İşleniyor") && !status.includes("Ayrıştırılıyor")}
        <div class="error-message">
          {status}
        </div>
      {/if}
    </div>
  </div>

  <!-- Right Panel: Info -->
  <div class="panel right-panel">
    <div class="preview-placeholder">
      <i class="nf-md-call_split"></i>
      <h3>PDF Splitter</h3>
      <p>Büyük PDF dosyalarını bölün veya istediğiniz sayfaları ayıklayın.</p>
    </div>
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
    width: 50%;
    border-right: 1px solid var(--border-color, #45475a);
    gap: 1.5rem;
  }

  .right-panel {
    width: 50%;
    align-items: center;
    justify-content: center;
    background-color: rgba(0, 0, 0, 0.2);
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

  .controls-block {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding-top: 0.5rem;
  }

  .radio-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .radio-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
    font-size: 0.95rem;
  }

  .range-inputs {
    display: flex;
    gap: 1rem;
  }

  .input-group {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
    flex: 1;
  }

  .input-group label {
    font-size: 0.85rem;
    color: var(--text-muted);
  }

  input[type="text"],
  input[type="number"] {
    background: var(--bg-input);
    border: 1px solid var(--border-color);
    padding: 0.6rem;
    border-radius: 6px;
    color: var(--text-main);
  }

  select:focus,
  input:focus {
    outline: none;
    border-color: var(--accent);
  }

  .path-select {
    display: flex;
    gap: 0.5rem;
  }

  .path-select input {
    flex: 1;
    font-family: monospace;
  }

  .icon-btn {
    background: var(--bg-input);
    border: 1px solid var(--border-color);
    color: var(--text-main);
    width: 40px;
    border-radius: 6px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .icon-btn:hover {
    border-color: var(--accent);
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

  .success-btn {
    padding: 0.5rem;
    background: transparent;
    border: 1px solid #a6e3a1;
    color: #a6e3a1;
    border-radius: 4px;
    cursor: pointer;
  }
  .success-btn:hover {
    background: rgba(166, 227, 161, 0.1);
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
    gap: 1rem;
    color: var(--text-muted);
    text-align: center;
    padding: 2rem;
  }

  .preview-placeholder i {
    font-size: 4rem;
    opacity: 0.3;
  }
</style>
