<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onMount, onDestroy } from "svelte";

  // Types
  interface ImageFileInfo {
    path: string;
    name: string;
  }

  // State
  let selectedFiles: ImageFileInfo[] = [];
  let outputPath: string = "";
  let isProcessing: boolean = false;
  let currentFile: string = "";
  let progress: number = 0;
  let status: string = "Hazır";
  let isDragging: boolean = false;
  let draggedIndex: number | null = null;
  let showSuccess: boolean = false;
  let successOutputPath: string = "";

  // Dosya seçme
  async function selectFiles() {
    try {
      const selected = await open({
        multiple: true,
        filters: [
          {
            name: "Images",
            extensions: ["png", "jpg", "jpeg", "bmp", "webp"],
          },
        ],
      });

      if (selected) {
        const paths = Array.isArray(selected) ? selected : [selected];
        addFiles(paths);
      }
    } catch (error) {
      status = `Hata: ${error}`;
    }
  }

  function addFiles(paths: string[]) {
    let newFiles: ImageFileInfo[] = [];

    for (const path of paths) {
      // Check if file already exists
      if (selectedFiles.some((f) => f.path === path)) {
        continue;
      }

      const fileName = path.split("/").pop() || path;

      newFiles.push({
        path,
        name: fileName,
      });
    }

    if (newFiles.length > 0) {
      selectedFiles = [...selectedFiles, ...newFiles];
      // İlk dosya eklendiğinde otomatik output path oluştur
      if (selectedFiles.length > 0 && !outputPath) {
        const firstPath = selectedFiles[0].path;
        const directory = firstPath.substring(0, firstPath.lastIndexOf("/"));
        outputPath = `${directory}/images_output.pdf`;
      }
    }
  }

  // Çıktı klasörü seçme (Save Dialog kullanmak daha mantıklı olabilir ama şimdilik folder select + filename)
  async function selectOutputFolder() {
    try {
      const selected = await open({
        directory: true,
      });

      if (selected && !Array.isArray(selected)) {
        outputPath = selected + "/images_output.pdf";
      }
    } catch (error) {
      status = `Hata: ${error}`;
    }
  }

  // Dosya silme
  function removeFile(index: number) {
    selectedFiles = selectedFiles.filter((_, i) => i !== index);
    if (selectedFiles.length === 0) {
      outputPath = "";
    }
  }

  // Drag & Drop Sıralama
  function handleFileDragStart(e: DragEvent, index: number) {
    draggedIndex = index;
    if (e.dataTransfer) {
      e.dataTransfer.effectAllowed = "move";
    }
  }

  function handleFileDragOver(e: DragEvent, index: number) {
    e.preventDefault();
    e.stopPropagation();
    if (draggedIndex !== null && draggedIndex !== index) {
      const newFiles = [...selectedFiles];
      const draggedFile = newFiles[draggedIndex];
      newFiles.splice(draggedIndex, 1);
      newFiles.splice(index, 0, draggedFile);
      selectedFiles = newFiles;
      draggedIndex = index;
    }
  }

  function handleFileDragEnd() {
    draggedIndex = null;
  }

  let unlistenDrop: () => void;

  onMount(async () => {
    unlistenDrop = await getCurrentWindow().listen(
      "tauri://drag-drop",
      (event) => {
        const payload = event.payload as { paths: string[] };
        if (payload.paths) {
          const imgPaths = payload.paths.filter((p) =>
            /\.(png|jpg|jpeg|bmp|webp)$/i.test(p)
          );
          if (imgPaths.length > 0) {
            addFiles(imgPaths);
          } else {
            status = "Lütfen resim dosyaları sürükleyin";
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

  // Convert
  async function convertToPdf() {
    if (selectedFiles.length === 0) {
      status = "En az 1 resim dosyası seçmelisiniz!";
      return;
    }

    if (!outputPath) {
      status = "Çıktı dosyası yolunu belirleyin!";
      return;
    }

    isProcessing = true;
    status = "İşleniyor...";
    progress = 0;
    showSuccess = false;

    try {
      // Simüle edilmiş ilerleme
      for (let i = 0; i <= 10; i++) {
        progress = i * 10;
        await new Promise((resolve) => setTimeout(resolve, 100));
      }

      const result = await invoke<string>("images_to_pdf", {
        request: {
          image_paths: selectedFiles.map((f) => f.path),
          output_path: outputPath,
          page_size: null, // Default
        },
      });

      status = "Başarıyla tamamlandı!";
      progress = 100;
      showSuccess = true;
      successOutputPath = outputPath;
    } catch (error) {
      status = `Hata: ${error}`;
      showSuccess = false;
    } finally {
      isProcessing = false;
    }
  }

  async function openOutputFolder() {
    try {
      const directory = successOutputPath.substring(
        0,
        successOutputPath.lastIndexOf("/")
      );
      await invoke("open_folder", { path: directory });
    } catch (error) {
      console.error(`Error opening folder: ${error}`);
    }
  }

  async function openOutputFile() {
    try {
      await invoke("open_file", { path: successOutputPath });
    } catch (error) {
      console.error(`Error opening file: ${error}`);
    }
  }
</script>

<div class="layout">
  <!-- Left Panel -->
  <div class="panel left-panel">
    <div class="header">
      <h2>Images to PDF</h2>
      <p>Resimleri PDF'e dönüştürün</p>
    </div>

    <div
      class="file-container"
      class:dragging={isDragging}
      on:dragover={handleDragOver}
      on:dragleave={handleDragLeave}
      on:drop={handleDrop}
      role="region"
      aria-label="File drop zone"
    >
      <div class="file-list-header">
        <span>Eklenen Resimler ({selectedFiles.length})</span>
        <button class="add-btn-small" on:click={selectFiles}>
          <i class="nf-md-plus"></i>
        </button>
      </div>

      <div class="file-list-scroll">
        {#if selectedFiles.length === 0}
          <div class="empty-state">
            <i class="nf-md-image_plus"></i>
            <p>Resim dosyalarını buraya sürükleyin</p>
          </div>
        {:else}
          {#each selectedFiles as file, index (file.path)}
            <div
              class="file-item"
              class:dragging={draggedIndex === index}
              draggable="true"
              on:dragstart={(e) => handleFileDragStart(e, index)}
              on:dragover={(e) => handleFileDragOver(e, index)}
              on:dragend={handleFileDragEnd}
            >
              <div class="file-item-left">
                <i class="nf-md-drag_vertical drag-handle"></i>
                <span class="file-index">{index + 1}</span>
                <span class="file-name" title={file.name}>{file.name}</span>
              </div>
              <button
                class="remove-btn"
                on:click={() => removeFile(index)}
                title="Listeden Çıkar"
              >
                <i class="nf-md-close"></i>
              </button>
            </div>
          {/each}
        {/if}
      </div>
    </div>

    <div class="controls-block">
      <div class="control-row">
        <button class="primary-btn" on:click={selectFiles}>
          <i class="nf-md-image_plus"></i> Resim Ekle
        </button>
        <button class="secondary-btn" on:click={selectOutputFolder}>
          <i class="nf-md-folder_open"></i> Çıktı Seç
        </button>
      </div>

      <div class="input-group">
        <label for="output-path">Çıktı Yolu:</label>
        <input
          id="output-path"
          type="text"
          bind:value={outputPath}
          placeholder="/path/to/images_output.pdf"
        />
      </div>
    </div>

    <div class="action-block">
      {#if isProcessing}
        <div class="progress-section">
          <div class="progress-labels">
            <span>Dönüştürülüyor...</span>
            <span>{progress}%</span>
          </div>
          <div class="progress-track">
            <div class="progress-fill" style="width: {progress}%"></div>
          </div>
        </div>
      {:else}
        <button
          class="merge-btn"
          on:click={convertToPdf}
          disabled={selectedFiles.length === 0 || !outputPath}
        >
          <i class="nf-md-file_pdf_box"></i> PDF OLUŞTUR
        </button>
      {/if}

      {#if showSuccess}
        <div class="success-message">
          <i class="nf-md-check_circle"></i>
          <span>İşlem Başarılı!</span>
        </div>
        <div class="success-buttons">
          <button class="success-btn" on:click={openOutputFolder}
            >Klasörü Aç</button
          >
          <button class="success-btn" on:click={openOutputFile}
            >Dosyayı Aç</button
          >
        </div>
      {/if}

      {#if status && !status.includes("Başarılı") && !status.includes("Hazır") && !status.includes("İşleniyor")}
        <div class="error-message">
          {status}
        </div>
      {/if}
    </div>
  </div>

  <!-- Right Panel: Preview -->
  <div class="panel right-panel">
    <div class="preview-placeholder">
      {#if selectedFiles.length > 0}
        <i class="nf-md-image_multiple"></i>
        <h3>{selectedFiles.length} Resim Seçildi</h3>
        <p>PDF oluşturmak için "PDF OLUŞTUR" butonuna basın.</p>
      {:else}
        <i class="nf-md-eye_off"></i>
        <h3>Önizleme</h3>
        <p>Resimleri seçtiğinizde burada görünecek (Yakında)</p>
      {/if}
    </div>
  </div>
</div>

<style>
  /* Reuse styles from merger, could be shared component but copying for speed/isolation as requested */
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
    flex: 1;
    display: flex;
    flex-direction: column;
    background: var(--bg-input, #313244);
    border-radius: 8px;
    border: 2px dashed var(--border-color, #45475a);
    overflow: hidden;
    transition: all 0.2s;
  }

  .file-container.dragging {
    border-color: var(--accent, #89b4fa);
    background: rgba(137, 180, 250, 0.1);
  }

  .file-list-header {
    padding: 0.75rem 1rem;
    background: rgba(0, 0, 0, 0.2);
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 0.9rem;
    font-weight: 600;
  }

  .add-btn-small {
    background: transparent;
    border: none;
    color: var(--text-main);
    cursor: pointer;
    font-size: 1.1rem;
    padding: 2px 6px;
    border-radius: 4px;
  }

  .add-btn-small:hover {
    background: rgba(255, 255, 255, 0.1);
  }

  .file-list-scroll {
    flex: 1;
    overflow-y: auto;
    padding: 0.5rem;
  }

  .empty-state {
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
    gap: 0.5rem;
    opacity: 0.7;
  }

  .empty-state i {
    font-size: 2.5rem;
  }

  .file-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem 0.75rem;
    background: var(--bg-app);
    margin-bottom: 0.5rem;
    border-radius: 6px;
    cursor: grab;
    border: 1px solid transparent;
  }

  .file-item:hover {
    border-color: var(--border-color);
  }

  .file-item.dragging {
    opacity: 0.5;
    border: 1px dashed var(--accent);
  }

  .file-item-left {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    overflow: hidden;
  }

  .drag-handle {
    color: var(--text-muted);
    cursor: grab;
  }

  .file-index {
    color: var(--accent);
    font-weight: bold;
    font-family: monospace;
    width: 20px;
  }

  .file-name {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 200px;
  }

  .remove-btn {
    background: transparent;
    border: none;
    color: #f38ba8;
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .remove-btn:hover {
    background: rgba(243, 139, 168, 0.1);
  }

  .controls-block {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding-top: 0.5rem;
  }

  .control-row {
    display: flex;
    gap: 1rem;
  }

  .primary-btn,
  .secondary-btn {
    flex: 1;
    padding: 0.75rem;
    border-radius: 6px;
    border: none;
    font-weight: 500;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    transition: opacity 0.2s;
  }

  .primary-btn {
    background: var(--accent, #89b4fa);
    color: var(--bg-app);
  }

  .secondary-btn {
    background: var(--bg-input);
    color: var(--text-main);
    border: 1px solid var(--border-color);
  }

  .primary-btn:hover,
  .secondary-btn:hover {
    opacity: 0.9;
  }

  .input-group {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }

  .input-group label {
    font-size: 0.85rem;
    color: var(--text-muted);
  }

  .input-group input {
    background: var(--bg-input);
    border: 1px solid var(--border-color);
    padding: 0.6rem;
    border-radius: 6px;
    color: var(--text-main);
    font-family: monospace;
  }

  .input-group input:focus {
    outline: none;
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
    background: #a6e3a1;
    color: #1e1e2e;
    border: none;
    border-radius: 8px;
    font-size: 1.1rem;
    font-weight: bold;
    cursor: pointer;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
  }

  .merge-btn:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(166, 227, 161, 0.3);
  }

  .merge-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    background: var(--bg-input);
    color: var(--text-muted);
  }

  .progress-section {
    background: var(--bg-input);
    padding: 1rem;
    border-radius: 8px;
    border: 1px solid var(--accent);
  }

  .progress-labels {
    display: flex;
    justify-content: space-between;
    font-size: 0.85rem;
    margin-bottom: 0.5rem;
    color: var(--text-muted);
  }

  .progress-track {
    height: 10px;
    background: rgba(0, 0, 0, 0.3);
    border-radius: 5px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: var(--accent);
    transition: width 0.3s;
  }

  .success-message {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    color: #a6e3a1;
    font-weight: bold;
  }

  .success-buttons {
    display: flex;
    gap: 0.5rem;
  }

  .success-btn {
    flex: 1;
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
  }

  .preview-placeholder i {
    font-size: 4rem;
    opacity: 0.3;
  }
</style>
