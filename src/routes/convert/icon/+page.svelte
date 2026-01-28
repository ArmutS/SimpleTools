<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onMount, onDestroy } from "svelte";

  const ACCEPTED_EXTENSIONS = ["png", "jpg", "svg", "ico", "icns", "webp"];
  const TARGET_FORMATS = [
    { id: "ico", label: "Windows Icon (.ico)" },
    { id: "icns", label: "macOS Icon (.icns)" },
    { id: "png", label: "PNG Image (.png)" },
    { id: "fav", label: "Favicon Bundle" },
  ];

  interface FileInfo {
    path: string;
    name: string;
    extension: string;
    size: string;
  }

  let selectedFiles: FileInfo[] = [];
  let selectedTargets: string[] = [];
  let outputPath: string = "";
  let isProcessing: boolean = false;
  let status: string = "Ready";
  let isDragging: boolean = false;
  let showSuccess: boolean = false;
  let successOutput: string = "";
  let unlistenDrop: () => void;

  async function getFileInfo(path: string): Promise<FileInfo> {
    const name = path.split(/[/\\]/).pop() || path;
    const extension = name.split(".").pop()?.toLowerCase() || "";
    return { path, name, extension, size: "Unknown" };
  }

  async function selectFiles() {
    try {
      await getCurrentWindow().hide();
      const selected = await open({
        multiple: true,
        filters: [{ name: "Images", extensions: ACCEPTED_EXTENSIONS }],
      });
      await getCurrentWindow().show();
      await getCurrentWindow().setFocus();
      if (selected) {
        addFiles(Array.isArray(selected) ? selected : [selected]);
      }
    } catch (error) {
      status = `Error: ${error}`;
    }
  }

  async function addFiles(paths: string[]) {
    for (const path of paths) {
      if (selectedFiles.some((f) => f.path === path)) continue;
      const info = await getFileInfo(path);
      if (ACCEPTED_EXTENSIONS.includes(info.extension)) {
        selectedFiles = [...selectedFiles, info];
      }
    }
    if (selectedFiles.length > 0 && !outputPath) {
      const first = selectedFiles[0].path;
      outputPath = first.substring(0, first.lastIndexOf("/"));
    }
  }

  function removeFile(index: number) {
    selectedFiles = selectedFiles.filter((_, i) => i !== index);
    if (selectedFiles.length === 0) outputPath = "";
  }

  onMount(async () => {
    unlistenDrop = await getCurrentWindow().listen(
      "tauri://drag-drop",
      (event) => {
        const payload = event.payload as { paths: string[] };
        if (payload.paths) addFiles(payload.paths);
      },
    );
  });

  onDestroy(() => {
    if (unlistenDrop) unlistenDrop();
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

  async function selectOutputFolder() {
    try {
      await getCurrentWindow().hide();
      const selected = await open({ directory: true });
      await getCurrentWindow().show();
      await getCurrentWindow().setFocus();
      if (selected && !Array.isArray(selected)) outputPath = selected;
    } catch (e) {
      status = `Error: ${e}`;
    }
  }

  async function convert() {
    if (selectedFiles.length === 0) return (status = "Please select files!");
    if (selectedTargets.length === 0)
      return (status = "Please select target format!");
    if (!outputPath) return (status = "Please select output folder!");

    isProcessing = true;
    showSuccess = false;
    let successCount = 0;
    let errorCount = 0;

    try {
      for (const file of selectedFiles) {
        for (const target of selectedTargets) {
          status = `Converting ${file.name} to ${target.toUpperCase()}...`;

          try {
            await invoke("convert_icon", {
              filePath: file.path,
              outputDir: outputPath,
              targetFormat: target,
            });
            successCount++;
          } catch (e) {
            console.error(`Failed to convert ${file.name} to ${target}:`, e);
            errorCount++;
          }
        }
      }

      if (errorCount === 0) {
        status = `Successfully converted ${successCount} file(s)`;
        showSuccess = true;
        successOutput = outputPath;
      } else {
        status = `Completed with ${successCount} success, ${errorCount} error(s)`;
      }
    } catch (e) {
      status = `Error: ${e}`;
    } finally {
      isProcessing = false;
    }
  }

  async function openFolder() {
    await invoke("open_folder", { path: successOutput });
  }
</script>

<div class="layout">
  <div class="panel main-panel">
    <div class="unified-card">
      <div class="header">
        <h2>Icon Converter</h2>
        <p>Convert PNG, SVG to ICO, ICNS</p>
      </div>

      <div
        class="file-container"
        class:dragging={isDragging}
        role="region"
        aria-label="Upload Drop Zone"
        on:dragover={handleDragOver}
        on:dragleave={handleDragLeave}
        on:drop={handleDrop}
      >
        <div class="file-list-header">
          <span>Files ({selectedFiles.length})</span>
          <button
            class="add-btn-small"
            on:click={selectFiles}
            aria-label="Add Files"><i class="nf-md-plus"></i></button
          >
        </div>
        <div class="file-list-scroll">
          {#if selectedFiles.length === 0}
            <div class="empty-state">
              <i class="nf-md-emoticon_outline"></i>
              <p>Drag icons/images here</p>
            </div>
          {:else}
            {#each selectedFiles as file, i}
              <div class="file-item">
                <div class="file-info">
                  <span class="file-index">{i + 1}</span>
                  <span class="file-name" title={file.path}>{file.name}</span>
                </div>
                <button
                  class="remove-btn"
                  on:click={() => removeFile(i)}
                  aria-label="Remove File"><i class="nf-md-close"></i></button
                >
              </div>
            {/each}
          {/if}
        </div>
      </div>

      <div class="controls-block">
        <div class="format-selection">
          <span class="section-label">Target Formats:</span>
          <div class="checkbox-grid">
            {#each TARGET_FORMATS as fmt}
              <label class="checkbox-label">
                <input
                  type="checkbox"
                  value={fmt.id}
                  bind:group={selectedTargets}
                />
                {fmt.label}
              </label>
            {/each}
          </div>
        </div>
        <div class="path-selection">
          <button class="secondary-btn" on:click={selectOutputFolder}
            ><i class="nf-md-folder_open"></i> Select Folder</button
          >
          <input
            type="text"
            bind:value={outputPath}
            placeholder="Output folder..."
            readonly
          />
        </div>
      </div>

      <div class="action-block">
        {#if !isProcessing && !showSuccess}
          <button
            class="convert-btn"
            on:click={convert}
            disabled={selectedFiles.length === 0 ||
              selectedTargets.length === 0}
          >
            <i class="nf-md-swap_horizontal"></i> CONVERT
          </button>
        {:else if isProcessing}
          <div class="processing-state">
            <i class="nf-md-loading nf-spin"></i> Processing...
          </div>
        {:else if showSuccess}
          <div class="success-actions">
            <div class="success-msg"><i class="nf-md-check"></i> Completed</div>
            <button class="success-btn" on:click={openFolder}
              >Open Folder</button
            >
          </div>
        {/if}
        {#if status && !status.includes("Ready") && !status.includes("Converting") && !showSuccess}
          <div class="status-msg">{status}</div>
        {/if}
      </div>
    </div>
  </div>
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    overflow: hidden;
    background: var(--bg-app, #1e1e2e);
    color: var(--text-main, #cdd6f4);
    font-family: "Segoe UI", sans-serif;
  }
  .layout {
    display: flex;
    width: 100vw;
    height: 100vh;
  }
  .panel {
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
  }
  .main-panel {
    width: 100%;
  }
  .unified-card {
    background: var(--bg-app);
    border-radius: 12px;
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    height: 100%;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.5);
    border: 1px solid var(--border-color);
    color: var(--text-main, #cdd6f4);
  }
  .header {
    margin-bottom: 0.5rem;
  }
  .header h2 {
    margin: 0;
    color: var(--accent, #89b4fa);
  }
  .header p {
    margin: 0;
    color: var(--text-muted, #a6adc8);
    font-size: 0.9rem;
  }
  .file-container {
    flex: 1;
    border: 2px dashed var(--border-color, #45475a);
    border-radius: 8px;
    background: var(--bg-input, #313244);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
  .file-container.dragging {
    border-color: var(--accent);
    background: var(--bg-app);
  }
  .file-list-header {
    padding: 0.5rem 1rem;
    background: var(--bg-app);
    border-bottom: 1px solid var(--border-color);
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-weight: bold;
    font-size: 0.9rem;
    color: var(--text-main);
  }
  .add-btn-small {
    background: none;
    border: none;
    color: inherit;
    cursor: pointer;
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
    opacity: 1;
    gap: 0.5rem;
  }
  .empty-state i {
    font-size: 3rem;
    color: var(--border-color);
  }
  .file-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem;
    background: var(--bg-app);
    margin-bottom: 0.5rem;
    border-radius: 4px;
    border: 1px solid var(--border-color);
    color: var(--text-main);
  }
  .file-info {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    overflow: hidden;
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
    max-width: 400px;
    font-size: 0.9rem;
  }
  .remove-btn {
    background: none;
    border: none;
    color: #f38ba8;
    cursor: pointer;
  }
  .controls-block {
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  .section-label {
    font-size: 0.85rem;
    font-weight: bold;
    color: var(--text-muted);
    display: block;
    margin-bottom: 0.5rem;
  }
  .checkbox-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 0.5rem;
  }
  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.9rem;
    cursor: pointer;
  }
  .path-selection {
    display: flex;
    gap: 0.5rem;
  }
  .secondary-btn {
    padding: 0.5rem 1rem;
    background: var(--bg-app);
    border: 1px solid var(--border-color);
    color: inherit;
    border-radius: 4px;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    white-space: nowrap;
  }
  .path-selection input {
    flex: 1;
    background: var(--bg-app);
    border: 1px solid var(--border-color);
    color: inherit;
    padding: 0.5rem;
    border-radius: 4px;
    font-family: monospace;
  }
  .action-block {
    margin-top: auto;
  }
  .convert-btn {
    width: 100%;
    padding: 1rem;
    background: var(--accent);
    color: var(--bg-app);
    border: none;
    border-radius: 8px;
    font-weight: bold;
    cursor: pointer;
    font-size: 1.1rem;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
  }
  .convert-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    background: var(--bg-input);
    color: var(--text-muted);
  }
  .convert-btn:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(250, 179, 135, 0.3);
  }
  .processing-state {
    text-align: center;
    font-weight: bold;
    color: var(--accent);
    padding: 1rem;
  }
  .success-actions {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  .success-msg {
    color: #a6e3a1;
    font-weight: bold;
    text-align: center;
  }
  .success-btn {
    width: 100%;
    padding: 0.75rem;
    background: transparent;
    border: 1px solid #a6e3a1;
    color: #a6e3a1;
    border-radius: 6px;
    cursor: pointer;
  }
  .success-btn:hover {
    background: var(--bg-app);
    border-color: #a6e3a1;
  }
  .status-msg {
    margin-top: 0.5rem;
    color: #f38ba8;
    text-align: center;
    font-size: 0.9rem;
  }
</style>
