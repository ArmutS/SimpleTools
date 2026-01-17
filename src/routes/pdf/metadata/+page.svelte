<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onMount, onDestroy } from "svelte";

  // Types
  interface PdfMetadata {
    title: string | null;
    author: string | null;
    subject: string | null;
    keywords: string | null;
    creator: string | null;
    producer: string | null;
    creation_date: string | null;
    modification_date: string | null;
  }

  // State
  let selectedFile: string = "";
  let outputPath: string = "";
  let isProcessing: boolean = false;
  let status: string = "Hazır";
  let isDragging: boolean = false;
  let showSuccess: boolean = false;

  // Metadata Form
  let metadata: PdfMetadata = {
    title: "",
    author: "",
    subject: "",
    keywords: "",
    creator: "",
    producer: "",
    creation_date: "",
    modification_date: "",
  };

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
          outputPath = `${directory}/metadata_updated_${filename}`;
        }

        // Read existing metadata
        await readMetadata();
      }
    } catch (error) {
      status = `Hata: ${error}`;
    }
  }

  // Read Metadata
  async function readMetadata() {
    if (!selectedFile) return;
    status = "Metadata okunuyor...";
    try {
      const meta = await invoke<PdfMetadata>("pdf_read_metadata", {
        path: selectedFile,
      });
      metadata = {
        title: meta.title || "",
        author: meta.author || "",
        subject: meta.subject || "",
        keywords: meta.keywords || "",
        creator: meta.creator || "",
        producer: meta.producer || "",
        creation_date: meta.creation_date || "",
        modification_date: meta.modification_date || "",
      };
      status = "Metadata yüklendi.";
    } catch (e) {
      status = `Metadata okuma hatası: ${e}`;
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
          : "updated.pdf";
        outputPath = `${selected}/${filename}`;
      }
    } catch (error) {
      status = `Hata: ${error}`;
    }
  }

  // Update Metadata
  async function updateMetadata() {
    if (!selectedFile) {
      status = "Bir PDF dosyası seçmelisiniz!";
      return;
    }

    if (!outputPath) {
      status = "Çıktı yolu belirlemelisiniz!";
      return;
    }

    isProcessing = true;
    status = "Güncelleniyor...";
    showSuccess = false;

    try {
      // Prepare metadata object (convert empty strings to null if desired, or keep as string)
      // Backend handles Option<String>, so we pass string.

      await invoke("pdf_metadata", {
        request: {
          file_path: selectedFile,
          output_path: outputPath,
          metadata: {
            title: metadata.title || null,
            author: metadata.author || null,
            subject: metadata.subject || null,
            keywords: metadata.keywords || null,
            creator: metadata.creator || null,
            producer: metadata.producer || null,
            // Dates are complex, mostly read-only in this simple editor unless validated format
            creation_date: metadata.creation_date || null,
            modification_date: metadata.modification_date || null,
          },
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
              outputPath = `${directory}/metadata_updated_${filename}`;
            }
            readMetadata();
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
      <h2>Metadata Editor</h2>
      <p>PDF künye bilgilerini düzenleyin</p>
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

    <div class="controls-block scrollable">
      <label>Metadata</label>

      <div class="form-grid">
        <div class="input-group">
          <label for="title">Başlık (Title):</label>
          <input id="title" type="text" bind:value={metadata.title} />
        </div>

        <div class="input-group">
          <label for="author">Yazar (Author):</label>
          <input id="author" type="text" bind:value={metadata.author} />
        </div>

        <div class="input-group">
          <label for="subject">Konu (Subject):</label>
          <input id="subject" type="text" bind:value={metadata.subject} />
        </div>

        <div class="input-group">
          <label for="keywords">Anahtar Kelimeler:</label>
          <input id="keywords" type="text" bind:value={metadata.keywords} />
        </div>

        <div class="input-group">
          <label for="creator">Oluşturan (Creator):</label>
          <input id="creator" type="text" bind:value={metadata.creator} />
        </div>

        <div class="input-group">
          <label for="producer">Üretici (Producer):</label>
          <input id="producer" type="text" bind:value={metadata.producer} />
        </div>
      </div>

      <div class="input-group output-group">
        <label for="output-path">Çıktı Yolu:</label>
        <div class="path-select">
          <input
            id="output-path"
            type="text"
            bind:value={outputPath}
            placeholder="/path/to/updated.pdf"
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
        on:click={updateMetadata}
        disabled={!selectedFile || !outputPath || isProcessing}
      >
        {#if isProcessing}
          <i class="nf-md-loading nf-spin"></i> İşleniyor...
        {:else}
          <i class="nf-md-content_save_edit"></i> GÜNCELLE
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

      {#if status && !status.includes("Başarılı") && !status.includes("Hazır") && !status.includes("İşleniyor") && !status.includes("yüklendi")}
        <div class="error-message">
          {status}
        </div>
      {/if}
    </div>
  </div>

  <!-- Right Panel: Info -->
  <div class="panel right-panel">
    <div class="preview-placeholder">
      <i class="nf-md-file_document_edit"></i>
      <h3>Metadata Editor</h3>
      <p>PDF dosyanızın başlık, yazar ve diğer bilgilerini düzenleyin.</p>
    </div>

    {#if metadata.title || metadata.author}
      <div class="meta-preview">
        <h4>Önizleme</h4>
        <p><strong>Başlık:</strong> {metadata.title || "-"}</p>
        <p><strong>Yazar:</strong> {metadata.author || "-"}</p>
        <p><strong>Konu:</strong> {metadata.subject || "-"}</p>
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
    width: 55%;
    border-right: 1px solid var(--border-color, #45475a);
    gap: 1.5rem;
  }

  .right-panel {
    width: 45%;
    align-items: center;
    justify-content: center;
    background-color: rgba(0, 0, 0, 0.2);
    display: flex;
    flex-direction: column;
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
    height: 100px;
    min-height: 100px;
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
    font-size: 2rem;
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
    font-size: 2rem;
    color: #f38ba8;
  }

  .file-info {
    flex: 1;
    overflow: hidden;
  }

  .file-path {
    font-family: monospace;
    font-size: 0.8rem;
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
    flex: 1;
    overflow-y: auto;
  }

  .scrollable {
    padding-right: 0.5rem;
  }

  .form-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.8rem;
  }

  .input-group {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
  }

  .output-group {
    margin-top: 1rem;
  }

  .input-group label {
    font-size: 0.8rem;
    color: var(--text-muted);
  }

  input[type="text"] {
    background: var(--bg-input);
    border: 1px solid var(--border-color);
    padding: 0.5rem;
    border-radius: 6px;
    color: var(--text-main);
    width: 100%;
    box-sizing: border-box;
    font-size: 0.9rem;
  }

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
    padding-top: 1rem;
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
    align-self: center;
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

  .meta-preview {
    margin-top: 2rem;
    padding: 1rem;
    background: var(--bg-input);
    border-radius: 8px;
    width: 80%;
    text-align: left;
  }

  .meta-preview h4 {
    margin: 0 0 1rem 0;
    color: var(--accent);
    border-bottom: 1px solid var(--border-color);
    padding-bottom: 0.5rem;
  }

  .meta-preview p {
    margin: 0.5rem 0;
    font-size: 0.9rem;
  }
</style>
