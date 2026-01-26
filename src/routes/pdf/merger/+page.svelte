<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onMount, onDestroy } from "svelte";

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
  let status: string = "Ready";
  let isDragging: boolean = false;
  let draggedIndex: number | null = null;
  let showSuccess: boolean = false;
  let successOutputPath: string = "";

  // Select Files
  async function selectFiles() {
    try {
      await getCurrentWindow().hide();
      const selected = await open({
        multiple: true,
        filters: [
          {
            name: "PDF",
            extensions: ["pdf"],
          },
        ],
      });
      await getCurrentWindow().show();
      await getCurrentWindow().setFocus();

      if (selected) {
        const paths = Array.isArray(selected) ? selected : [selected];
        await addFiles(paths);
      }
    } catch (error) {
      status = `Error: ${error}`;
    }
  }

  // Add Files
  async function addFiles(paths: string[]) {
    console.log("Adding files:", paths);
    let newFiles: PdfFileInfo[] = [];

    for (const path of paths) {
      try {
        console.log("Processing:", path);
        // Check if file already exists
        if (selectedFiles.some((f) => f.path === path)) {
          console.log("File already exists:", path);
          continue;
        }

        const info = await invoke<{
          page_count: number;
          file_size_formatted: string;
          is_encrypted: boolean;
          error: string | null;
        }>("get_pdf_info", { file_path: path });

        console.log("Info received:", info);

        const fileName = path.split("/").pop() || path;

        newFiles.push({
          path,
          name: fileName,
          pageCount: info.page_count,
          fileSize: info.file_size_formatted,
          isEncrypted: info.is_encrypted,
          error: info.error,
        });
      } catch (error) {
        console.error(`Error reading PDF info for ${path}: ${error}`);
        status = `Error: ${error}`; // Show error to user
      }
    }

    if (newFiles.length > 0) {
      selectedFiles = [...selectedFiles, ...newFiles];
      // Auto set output path on first file if empty
      if (selectedFiles.length > 0 && !outputPath) {
        const firstPath = selectedFiles[0].path;
        const directory = firstPath.substring(0, firstPath.lastIndexOf("/"));
        const originalName = selectedFiles[0].name;
        // Strip extension
        const baseName =
          originalName.substring(0, originalName.lastIndexOf(".")) ||
          originalName;
        outputPath = `${directory}/${baseName}_merged.pdf`;
      }
    }
  }

  // Select Output
  async function selectOutputFolder() {
    try {
      await getCurrentWindow().hide();
      const selected = await open({
        directory: true,
      });
      await getCurrentWindow().show();
      await getCurrentWindow().setFocus();

      if (selected && !Array.isArray(selected)) {
        outputPath = selected + "/merged_output.pdf";
      }
    } catch (error) {
      status = `Error: ${error}`;
    }
  }

  // Remove File
  function removeFile(index: number) {
    selectedFiles = selectedFiles.filter((_, i) => i !== index);

    // Clear output path if empty
    if (selectedFiles.length === 0) {
      outputPath = "";
    }
  }

  // Drag & Drop Sorting
  function handleFileDragStart(e: DragEvent, index: number) {
    draggedIndex = index;
    if (e.dataTransfer) {
      e.dataTransfer.effectAllowed = "move";
    }
  }

  function handleFileDragOver(e: DragEvent, index: number) {
    e.preventDefault();
    e.stopPropagation(); // Prevent bubbling to main drop zone
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
        console.log("Tauri Drop Event:", event);
        const payload = event.payload as { paths: string[] };
        if (payload.paths) {
          console.log("Dropped paths:", payload.paths);
          const pdfPaths = payload.paths.filter((p) =>
            p.toLowerCase().endsWith(".pdf"),
          );
          if (pdfPaths.length > 0) {
            addFiles(pdfPaths);
          } else {
            status = "Please drag PDF files";
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

  // HTML5 DnD handlers
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
    // Actual file handling is done by tauri://drop listener
  }

  // PDF Merge
  async function mergePDFs() {
    if (selectedFiles.length < 2) {
      status = "You must select at least 2 PDF files!";
      return;
    }

    if (!outputPath) {
      status = "Please specify output file path!";
      return;
    }

    // Encrypted check
    const encryptedFiles = selectedFiles.filter((f) => f.isEncrypted);
    if (encryptedFiles.length > 0) {
      status = `Error: ${encryptedFiles.length} encrypted files found. Cannot process encrypted files!`;
      return;
    }

    isProcessing = true;
    status = "Processing...";
    progress = 0;
    showSuccess = false;

    try {
      // Simulated progress
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

      status = "Successfully completed!";
      progress = 100;
      showSuccess = true;
      successOutputPath = outputPath;
    } catch (error) {
      status = `Error: ${error}`;
      showSuccess = false;
    } finally {
      isProcessing = false;
      currentFile = "";
    }
  }

  // Open Folder/File
  async function openOutputFolder() {
    try {
      const directory = successOutputPath.substring(
        0,
        successOutputPath.lastIndexOf("/"),
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
  <!-- Left Panel: Controls & File List -->
  <div class="panel left-panel">
    <!-- Header -->
    <div class="header">
      <h2>PDF Merger</h2>
      <p>Merge multiple PDF files into one</p>
    </div>

    <!-- File List & Drop Zone (Top Block) -->
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
        <span>Added Files ({selectedFiles.length})</span>
        <button
          class="add-btn-small"
          on:click={selectFiles}
          aria-label="Add Files"
        >
          <i class="nf-md-plus"></i>
        </button>
      </div>

      <div class="file-list-scroll">
        {#if selectedFiles.length === 0}
          <div class="empty-state">
            <i class="nf-md-file_upload"></i>
            <p>Drag PDF files here</p>
          </div>
        {:else}
          {#each selectedFiles as file, index (file.path)}
            <div
              class="file-item"
              class:encrypted={file.isEncrypted}
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
                title="Remove from list"
                aria-label="Remove File"
              >
                <i class="nf-md-close"></i>
              </button>
            </div>
          {/each}
        {/if}
      </div>
    </div>

    <!-- Controls (Middle Block) -->
    <div class="controls-block">
      <div class="control-row">
        <button class="primary-btn" on:click={selectFiles}>
          <i class="nf-md-file_plus"></i> Add Files
        </button>
        <button class="secondary-btn" on:click={selectOutputFolder}>
          <i class="nf-md-folder_open"></i> Output Path
        </button>
      </div>

      <div class="input-group">
        <label for="output-path">Output Path:</label>
        <input
          id="output-path"
          type="text"
          bind:value={outputPath}
          placeholder="/path/to/merged_output.pdf"
        />
      </div>
    </div>

    <!-- Action & Progress (Bottom Block) -->
    <div class="action-block">
      {#if isProcessing}
        <div class="progress-section">
          <div class="progress-labels">
            <span>{currentFile}</span>
            <span>{progress}%</span>
          </div>
          <div class="progress-track">
            <div class="progress-fill" style="width: {progress}%"></div>
          </div>
        </div>
      {:else}
        <button
          class="merge-btn"
          on:click={mergePDFs}
          disabled={selectedFiles.length < 2 || !outputPath}
        >
          <i class="nf-md-merge"></i> MERGE
        </button>
      {/if}

      {#if showSuccess}
        <div class="success-message">
          <i class="nf-md-check_circle"></i>
          <span>Operation Successful!</span>
        </div>
        <div class="success-buttons">
          <button class="success-btn" on:click={openOutputFolder}
            >Open Folder</button
          >
          <button class="success-btn" on:click={openOutputFile}
            >Open File</button
          >
        </div>
      {/if}

      {#if status && !status.includes("Successful") && !status.includes("Ready") && !status.includes("Processing")}
        <div class="error-message">
          {status}
        </div>
      {/if}
    </div>
  </div>

  <!-- Right Panel: Preview (Placeholder) -->
  <div class="panel right-panel">
    <div class="preview-placeholder">
      <i class="nf-md-eye_off"></i>
      <h3>Preview</h3>
      <p>Preview will appear here when files are selected (Soon)</p>
    </div>
  </div>
</div>

<style>
  /* Layout */
  :global(body) {
    margin: 0;
    padding: 0;
    overflow: hidden;
  }

  .layout {
    display: flex;
    width: 100vw;
    height: 100vh;
    background-color: var(--bg-app, #1e1e2e); /* Solid background */
    color: var(--text-main, #cdd6f4);
  }

  .panel {
    display: flex;
    flex-direction: column;
    padding: 1.5rem;
    height: 100%;
  }

  /* Left Panel */
  .left-panel {
    width: 50%;
    border-right: 1px solid var(--border-color, #45475a);
    gap: 1.5rem;
  }

  /* Right Panel */
  .right-panel {
    width: 50%;
    align-items: center;
    justify-content: center;
    background-color: rgba(0, 0, 0, 0.2);
  }

  /* Header */
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

  /* File Container */
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

  /* File Item */
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

  /* Controls Block */
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

  /* Action Block */
  .action-block {
    margin-top: auto;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .merge-btn {
    width: 100%;
    padding: 1rem;
    background: #a6e3a1; /* Green */
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

  /* Progress */
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

  /* Success Message */
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

  /* Preview Placeholder */
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
