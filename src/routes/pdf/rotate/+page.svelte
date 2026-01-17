<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onMount, onDestroy } from "svelte";

  // State
  let selectedFile: string = "";
  let outputPath: string = "";
  let isProcessing: boolean = false;
  let status: string = "Hazır";
  let isDragging: boolean = false;
  let showSuccess: boolean = false;

  // Settings
  let rotation: string = "90";
  let pageInputValue: string = ""; // "1,3-5" etc. (Empty = all)

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
        // Construct default output path
        const directory = selectedFile.substring(
          0,
          selectedFile.lastIndexOf("/")
        );
        const filename = selectedFile.split("/").pop();
        if (filename && !outputPath) {
          outputPath = `${directory}/rotated_${filename}`;
        }
      }
    } catch (error) {
      status = `Hata: ${error}`;
    }
  }

  // Çıktı kaydetme yeri seçme
  async function selectOutput() {
    try {
      const selected = await open({
        directory: true,
      });

      if (selected && !Array.isArray(selected)) {
        const filename = selectedFile
          ? selectedFile.split("/").pop()
          : "rotated.pdf";
        outputPath = `${selected}/${filename}`;
      }
    } catch (error) {
      status = `Hata: ${error}`;
    }
  }

  // Rotate
  async function rotatePdf() {
    if (!selectedFile) {
      status = "Bir PDF dosyası seçmelisiniz!";
      return;
    }

    if (!outputPath) {
      status = "Çıktı yolu belirlemelisiniz!";
      return;
    }

    isProcessing = true;
    status = "Döndürülüyor...";
    showSuccess = false;

    try {
      // Parse pages
      let pages: number[] = [];
      if (pageInputValue.trim()) {
        const parts = pageInputValue.split(",");
        for (const part of parts) {
          if (part.includes("-")) {
            const [start, end] = part.split("-").map(Number);
            if (!isNaN(start) && !isNaN(end)) {
              for (let i = start; i <= end; i++) pages.push(i);
            }
          } else {
            const num = Number(part);
            if (!isNaN(num)) pages.push(num);
          }
        }
      }

      const rot = parseInt(rotation);

      await invoke("pdf_rotate", {
        request: {
          file_path: selectedFile,
          output_path: outputPath,
          pages: pages,
          rotation: rot,
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
            p.toLowerCase().endsWith(".pdf")
          );
          if (pdf) {
            selectedFile = pdf;
            if (!outputPath) {
              const directory = pdf.substring(0, pdf.lastIndexOf("/"));
              const filename = pdf.split("/").pop();
              outputPath = `${directory}/rotated_${filename}`;
            }
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

  async function openOutputFile() {
    if (outputPath) {
      await invoke("open_file", { path: outputPath });
    }
  }
</script>

<div class="layout">
  <!-- Left Panel -->
  <div class="panel left-panel">
    <div class="header">
      <h2>Rotate PDF</h2>
      <p>PDF sayfalarını döndürün</p>
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
      <div class="settings-row">
        <div class="setting-group">
          <label for="rotation">Döndürme Açısı</label>
          <select id="rotation" bind:value={rotation}>
            <option value="90">90° (Saat Yönü)</option>
            <option value="180">180°</option>
            <option value="270">270° (Saat Yönü Tersi)</option>
            <option value="-90">-90° (Saat Yönü Tersi)</option>
          </select>
        </div>
      </div>

      <div class="input-group">
        <label for="pages">Sayfalar (Boş = Tümü):</label>
        <input
          id="pages"
          type="text"
          bind:value={pageInputValue}
          placeholder="Örn: 1,3,5-10"
        />
        <small style="color:var(--text-muted); font-size: 0.75rem;"
          >Virgül ile ayırın veya tire ile aralık belirtin.</small
        >
      </div>

      <div class="input-group">
        <label for="output-path">Çıktı Yolu:</label>
        <div class="path-select">
          <input
            id="output-path"
            type="text"
            bind:value={outputPath}
            placeholder="/path/to/output.pdf"
          />
          <button class="icon-btn" on:click={selectOutput}>
            <i class="nf-md-folder_open"></i>
          </button>
        </div>
      </div>
    </div>

    <div class="action-block">
      <button
        class="merge-btn"
        on:click={rotatePdf}
        disabled={!selectedFile || !outputPath || isProcessing}
      >
        {#if isProcessing}
          <i class="nf-md-loading nf-spin"></i> İşleniyor...
        {:else}
          <i class="nf-md-rotate_right"></i> DÖNDÜR
        {/if}
      </button>

      {#if showSuccess}
        <div class="success-message">
          <i class="nf-md-check_circle"></i>
          <span>İşlem Başarılı!</span>
        </div>
        <button class="success-btn" on:click={openOutputFile}>Dosyayı Aç</button
        >
      {/if}

      {#if status && !status.includes("Başarılı") && !status.includes("Hazır") && !status.includes("İşleniyor")}
        <div class="error-message">
          {status}
        </div>
      {/if}
    </div>
  </div>

  <!-- Right Panel: Info -->
  <div class="panel right-panel">
    <div class="preview-placeholder">
      <i class="nf-md-rotate_right"></i>
      <h3>Rotate PDF</h3>
      <p>PDF sayfa yönünü kolayca değiştirin.</p>
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

  .settings-row {
    display: flex;
    gap: 1rem;
  }

  .setting-group {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }

  .setting-group label,
  .input-group label {
    font-size: 0.85rem;
    color: var(--text-muted);
  }

  select,
  input {
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
    opacity: 0.9;
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
