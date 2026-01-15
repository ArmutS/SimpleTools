<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";

  // Types
  interface PdfFileInfo {
    path: string;
    name: string;
    pageCount: number;
    fileSize: string;
    isEncrypted: boolean;
    error: string | null;
  }

  // State
  let selectedFiles: PdfFileInfo[] = [];
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
            name: "PDF",
            extensions: ["pdf"],
          },
        ],
      });

      if (selected) {
        const paths = Array.isArray(selected) ? selected : [selected];
        await addFiles(paths);
      }
    } catch (error) {
      status = `Hata: ${error}`;
    }
  }

  // Dosya bilgilerini al ve listeye ekle
  async function addFiles(paths: string[]) {
    for (const path of paths) {
      try {
        const info = await invoke<{
          page_count: number;
          file_size_formatted: string;
          is_encrypted: boolean;
          error: string | null;
        }>("get_pdf_info", { filePath: path });

        const fileName = path.split("/").pop() || path;

        selectedFiles = [
          ...selectedFiles,
          {
            path,
            name: fileName,
            pageCount: info.page_count,
            fileSize: info.file_size_formatted,
            isEncrypted: info.is_encrypted,
            error: info.error,
          },
        ];

        // İlk dosya eklendiğinde otomatik output path oluştur
        if (selectedFiles.length === 1 && !outputPath) {
          const directory = path.substring(0, path.lastIndexOf("/"));
          outputPath = `${directory}/merged_output.pdf`;
        }
      } catch (error) {
        console.error(`Error reading PDF info: ${error}`);
      }
    }
  }

  // Çıktı klasörü seçme
  async function selectOutputFolder() {
    try {
      const selected = await open({
        directory: true,
      });

      if (selected && !Array.isArray(selected)) {
        outputPath = selected + "/merged_output.pdf";
      }
    } catch (error) {
      status = `Hata: ${error}`;
    }
  }

  // Dosya silme
  function removeFile(index: number) {
    selectedFiles = selectedFiles.filter((_, i) => i !== index);

    // Son dosya silindiyse output path'i temizle
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

  // Sürükle-bırak (dosya ekleme)
  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    isDragging = true;
  }

  function handleDragLeave() {
    isDragging = false;
  }

  async function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDragging = false;

    if (e.dataTransfer?.files) {
      const files = Array.from(e.dataTransfer.files);

      // In Tauri, we need to get the actual file paths
      // The File object has a path property in Tauri
      const pdfFiles = files.filter((f) =>
        f.name.toLowerCase().endsWith(".pdf")
      );

      // @ts-ignore - Tauri adds path property to File objects
      const pdfPaths = pdfFiles.map((f) => f.path).filter(Boolean);

      if (pdfPaths.length > 0) {
        await addFiles(pdfPaths);
      } else {
        status = "Lütfen PDF dosyaları sürükleyin";
      }
    }
  }

  // PDF Birleştirme
  async function mergePDFs() {
    if (selectedFiles.length < 2) {
      status = "En az 2 PDF dosyası seçmelisiniz!";
      return;
    }

    if (!outputPath) {
      status = "Çıktı dosyası yolunu belirleyin!";
      return;
    }

    // Şifreli dosya kontrolü
    const encryptedFiles = selectedFiles.filter((f) => f.isEncrypted);
    if (encryptedFiles.length > 0) {
      status = `Hata: ${encryptedFiles.length} şifreli dosya var. Şifreli dosyalar işlenemez!`;
      return;
    }

    isProcessing = true;
    status = "İşleniyor...";
    progress = 0;
    showSuccess = false;

    try {
      // Simüle edilmiş ilerleme
      for (let i = 0; i < selectedFiles.length; i++) {
        currentFile = selectedFiles[i].name;
        progress = Math.round(((i + 1) / selectedFiles.length) * 100);
        await new Promise((resolve) => setTimeout(resolve, 500));
      }

      const result = await invoke<string>("pdf_merge", {
        request: {
          files: selectedFiles.map((f) => f.path),
          output_path: outputPath,
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
      currentFile = "";
    }
  }

  // Klasör/Dosya aç
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

<main class="main">
  <div class="container">
    <h1>PDF Merger (Birleştirici)</h1>
    <p class="description">
      Birden fazla PDF dosyasını tek bir dosyada birleştirin. Dosyalar
      seçtiğiniz sırada birleştirilecektir.
    </p>

    <!-- Input Section -->
    <div class="section">
      <h3>📁 Dosya Seçimi</h3>

      <div
        class="drop-zone"
        class:dragging={isDragging}
        class:has-files={selectedFiles.length > 0}
        on:dragover={handleDragOver}
        on:dragleave={handleDragLeave}
        on:drop={handleDrop}
        role="region"
        aria-label="PDF file drop zone"
      >
        {#if selectedFiles.length === 0}
          <div class="empty-state">
            <i class="nf-md-file_upload" style="font-size: 3rem; opacity: 0.5;"
            ></i>
            <p>PDF dosyalarını buraya sürükleyin</p>
            <span>veya</span>
            <button class="select-button" on:click={selectFiles}>
              Dosya Seç
            </button>
          </div>
        {:else}
          <div class="file-list">
            <div class="file-list-header">
              <h4>Seçilen Dosyalar ({selectedFiles.length})</h4>
              <button class="select-button small" on:click={selectFiles}>
                + Ekle
              </button>
            </div>
            {#each selectedFiles as file, index (file.path)}
              <div
                class="file-card"
                class:encrypted={file.isEncrypted}
                class:dragging={draggedIndex === index}
                draggable="true"
                on:dragstart={(e) => handleFileDragStart(e, index)}
                on:dragover={(e) => handleFileDragOver(e, index)}
                on:dragend={handleFileDragEnd}
                role="listitem"
                aria-label="PDF file {index + 1}: {file.name}"
              >
                <div class="file-card-left">
                  <span class="file-number">{index + 1}</span>
                  <i class="nf-md-file_pdf file-icon"></i>
                  <div class="file-info">
                    <span class="file-name">{file.name}</span>
                    <div class="file-meta">
                      {#if file.isEncrypted}
                        <span class="encrypted-badge">
                          <i class="nf-md-lock"></i> Şifreli
                        </span>
                      {:else if file.error}
                        <span class="error-badge">Hata: {file.error}</span>
                      {:else}
                        <span>{file.pageCount} sayfa</span>
                        <span>•</span>
                        <span>{file.fileSize}</span>
                      {/if}
                    </div>
                  </div>
                </div>
                <button
                  class="icon-btn delete"
                  on:click={() => removeFile(index)}
                  title="Kaldır"
                >
                  <i class="nf-md-delete"></i>
                </button>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </div>

    <!-- Output Section -->
    <div class="section">
      <h3>💾 Çıktı Ayarları</h3>

      <div class="output-controls">
        <input
          type="text"
          class="path-input"
          bind:value={outputPath}
          placeholder="/home/user/Documents/merged_output.pdf"
        />
        <button class="select-button" on:click={selectOutputFolder}>
          Klasör Seç
        </button>
      </div>
    </div>

    <!-- Processing Section -->
    {#if isProcessing}
      <div class="section processing">
        <h3>⚙️ İşleniyor...</h3>
        <div class="progress-info">
          <span>Dosya: {currentFile}</span>
          <span>{progress}%</span>
        </div>
        <div class="progress-bar">
          <div class="progress-fill" style="width: {progress}%"></div>
        </div>
      </div>
    {/if}

    <!-- Success Section -->
    {#if showSuccess}
      <div class="section success">
        <h3>✅ Başarılı!</h3>
        <p>PDF dosyaları başarıyla birleştirildi.</p>
        <div class="success-actions">
          <button class="action-button secondary" on:click={openOutputFolder}>
            <i class="nf-md-folder_open"></i> Klasörü Aç
          </button>
          <button class="action-button secondary" on:click={openOutputFile}>
            <i class="nf-md-eye"></i> Dosyayı Aç
          </button>
        </div>
      </div>
    {/if}

    <!-- Action Section -->
    <div class="section">
      <button
        class="action-button"
        on:click={mergePDFs}
        disabled={isProcessing || selectedFiles.length < 2 || !outputPath}
      >
        {isProcessing ? "İşleniyor..." : "PDF'leri Birleştir"}
      </button>
      <p class="status" class:error={status.includes("Hata")}>{status}</p>
    </div>
  </div>
</main>

<style>
  .container {
    padding: 2rem;
    max-width: 1000px;
    margin: 0 auto;
  }

  h1 {
    color: var(--accent);
    margin-bottom: 0.5rem;
  }

  .description {
    color: var(--text-muted);
    margin-bottom: 2rem;
    font-size: 1rem;
  }

  .section {
    background: var(--bg-input);
    padding: 1.5rem;
    border-radius: 8px;
    margin-bottom: 1.5rem;
    border: 1px solid var(--border-color);
  }

  .section h3 {
    color: var(--text-main);
    margin-bottom: 1rem;
    font-size: 1.1rem;
  }

  /* Drop Zone */
  .drop-zone {
    border: 2px dashed var(--border-color);
    border-radius: 8px;
    min-height: 200px;
    transition: all 0.3s;
  }

  .drop-zone.dragging {
    border-color: var(--accent);
    background: var(--bg-app);
  }

  .drop-zone.has-files {
    padding: 1.5rem;
    border-style: solid;
  }

  /* Empty State */
  .empty-state {
    padding: 3rem 2rem;
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
  }

  .empty-state p {
    color: var(--text-muted);
    margin: 0;
  }

  .empty-state span {
    color: var(--text-muted);
    font-size: 0.9rem;
  }

  /* Buttons */
  .select-button {
    background: var(--accent);
    color: var(--bg-app);
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 6px;
    font-size: 1rem;
    cursor: pointer;
    transition: opacity 0.2s;
  }

  .select-button:hover {
    opacity: 0.8;
  }

  .select-button.small {
    padding: 0.5rem 1rem;
    font-size: 0.9rem;
  }

  /* File List */
  .file-list {
    width: 100%;
  }

  .file-list-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .file-list h4,
  .file-list-header h4 {
    color: var(--text-main);
    margin: 0;
    font-size: 0.95rem;
  }

  /* File Card */
  .file-card {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1rem;
    background: var(--bg-app);
    border-radius: 8px;
    margin-bottom: 0.75rem;
    border: 2px solid transparent;
    transition: all 0.2s;
    cursor: move;
  }

  .file-card:hover {
    border-color: var(--accent);
  }

  .file-card.dragging {
    opacity: 0.5;
    border-color: var(--accent);
  }

  .file-card.encrypted {
    border-color: #f38ba8;
    background: rgba(243, 139, 168, 0.1);
  }

  .file-card-left {
    display: flex;
    align-items: center;
    gap: 1rem;
    flex: 1;
    min-width: 0;
  }

  .file-number {
    background: var(--accent);
    color: var(--bg-app);
    width: 32px;
    height: 32px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.9rem;
    font-weight: bold;
    flex-shrink: 0;
  }

  .file-icon {
    font-size: 2rem;
    color: #f38ba8;
    flex-shrink: 0;
  }

  .file-info {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .file-name {
    color: var(--text-main);
    font-size: 0.95rem;
    font-weight: 500;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .file-meta {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    color: var(--text-muted);
    font-size: 0.85rem;
  }

  .encrypted-badge {
    color: #f38ba8;
    font-weight: 500;
  }

  .error-badge {
    color: #fab387;
    font-size: 0.8rem;
  }

  .icon-btn {
    background: transparent;
    border: 1px solid var(--border-color);
    color: var(--text-muted);
    width: 36px;
    height: 36px;
    border-radius: 6px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s;
    flex-shrink: 0;
  }

  .icon-btn:hover {
    border-color: #f38ba8;
    color: #f38ba8;
  }

  /* Output Controls */
  .output-controls {
    display: flex;
    gap: 1rem;
  }

  .path-input {
    flex: 1;
    background: var(--bg-app);
    border: 1px solid var(--border-color);
    color: var(--text-main);
    padding: 0.75rem;
    border-radius: 6px;
    font-size: 0.95rem;
    font-family: monospace;
  }

  .path-input:focus {
    outline: none;
    border-color: var(--accent);
  }

  /* Processing */
  .processing {
    border: 2px solid var(--accent);
  }

  .progress-info {
    display: flex;
    justify-content: space-between;
    margin-bottom: 0.75rem;
    color: var(--text-muted);
    font-size: 0.9rem;
  }

  .progress-bar {
    width: 100%;
    height: 24px;
    background: var(--bg-app);
    border-radius: 12px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, var(--accent), #94e2d5);
    transition: width 0.3s ease;
  }

  /* Success */
  .success {
    border: 2px solid #a6e3a1;
    background: rgba(166, 227, 161, 0.05);
  }

  .success h3 {
    color: #a6e3a1;
  }

  .success p {
    color: var(--text-muted);
    margin-bottom: 1rem;
  }

  .success-actions {
    display: flex;
    gap: 1rem;
  }

  /* Action Button */
  .action-button {
    background: var(--accent);
    color: var(--bg-app);
    padding: 1rem 2rem;
    border: none;
    border-radius: 6px;
    font-size: 1.1rem;
    font-weight: bold;
    cursor: pointer;
    transition: opacity 0.2s;
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
  }

  .action-button:hover:not(:disabled) {
    opacity: 0.8;
  }

  .action-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .action-button.secondary {
    background: var(--bg-app);
    color: var(--text-main);
    border: 1px solid var(--border-color);
    width: auto;
    flex: 1;
    font-size: 0.95rem;
    padding: 0.75rem 1.5rem;
  }

  .status {
    margin-top: 1rem;
    color: var(--text-muted);
    text-align: center;
    font-size: 0.95rem;
  }

  .status.error {
    color: #f38ba8;
  }
</style>
