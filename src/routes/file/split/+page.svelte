<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  let mode = "split";
  let filePath = "";
  let chunkSize = 10;
  let outputDir = "";
  async function selectFile() {
    const selected = await open({ multiple: false });
    if (selected) filePath = selected as string;
  }
  async function selectDir() {
    const selected = await open({ directory: true });
    if (selected) outputDir = selected as string;
  }
  async function execute() {
    if (!filePath || !outputDir) return alert("Select file and output directory");
    try {
      if (mode === "split") {
        await invoke("split_file", { filePath, chunkSizeMb: chunkSize, outputDir });
        alert("File split successfully!");
      } else {
        await invoke("merge_file_chunks", { chunksPattern: filePath, outputPath: outputDir });
        alert("File merged successfully!");
      }
    } catch (e) { alert(`Error: ${e}`); }
  }
</script>
<main style="min-height:100vh;padding:2rem;background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%)">
  <div style="max-width:900px;margin:0 auto">
    <div style="text-align:center;margin-bottom:2rem">
      <div style="display:inline-flex;width:80px;height:80px;background:linear-gradient(135deg, #f5c2e7 0%, #eba0ac 100%);border-radius:18px;align-items:center;justify-content:center;margin-bottom:1.5rem;box-shadow:0 8px 24px rgba(245,194,231,0.4);animation:float 3s ease-in-out infinite">
        <i class="nf-md-file_delimited" style="font-size:3rem;color:#1e1e2e"></i>
      </div>
      <h1 style="background:linear-gradient(135deg, #f5c2e7 0%, #eba0ac 100%);-webkit-background-clip:text;-webkit-text-fill-color:transparent;font-size:2.5rem;margin-bottom:0.5rem;font-weight:bold">File Splitter</h1>
      <p style="color:#a6adc8;font-size:1.1rem">Split large files or merge chunks</p>
    </div>
    <div style="background:rgba(24,24,37,0.7);backdrop-filter:blur(10px);padding:2.5rem;border-radius:20px;border:1px solid rgba(245,194,231,0.15);box-shadow:0 8px 32px rgba(0,0,0,0.3)">
      <div style="display:grid;grid-template-columns:1fr 1fr;gap:1.5rem;margin-bottom:2.5rem;background:#11111b;padding:0.75rem;border-radius:16px">
        <button on:click={() => mode="split"} style="padding:1.25rem;background:{mode==='split'?'linear-gradient(135deg, #f5c2e7 0%, #eba0ac 100%)':'transparent'};color:{mode==='split'?'#1e1e2e':'#cdd6f4'};border:none;border-radius:12px;cursor:pointer;font-weight:600;font-size:1.1rem;transition:all 0.3s">Split File</button>
        <button on:click={() => mode="merge"} style="padding:1.25rem;background:{mode==='merge'?'linear-gradient(135deg, #f5c2e7 0%, #eba0ac 100%)':'transparent'};color:{mode==='merge'?'#1e1e2e':'#cdd6f4'};border:none;border-radius:12px;cursor:pointer;font-weight:600;font-size:1.1rem;transition:all 0.3s">Merge Chunks</button>
      </div>
      
      <div style="display:grid;gap:1.5rem">
        <button on:click={selectFile} style="width:100%;padding:1.5rem;background:rgba(245,194,231,0.1);color:#f5c2e7;border:2px dashed #f5c2e7;border-radius:14px;cursor:pointer;font-weight:600;font-size:1.1rem;transition:all 0.3s">
          <i class="nf-md-file"></i> {filePath ? 'Change File' : 'Select Target File'}
        </button>
        {#if filePath}<p style="color:#cdd6f4;font-family:Consolas,monospace;background:#11111b;padding:1rem;border-radius:10px;text-align:center">{filePath}</p>{/if}

        {#if mode === "split"}
          <div style="animation:fadeIn 0.3s ease-out">
            <label style="color:#f5c2e7;margin-bottom:0.75rem;display:block;font-weight:600">Chunk Size (MB)</label>
            <input type="number" bind:value={chunkSize} min="1" style="width:100%;padding:1rem;background:#11111b;border:2px solid #313244;border-radius:12px;color:#cdd6f4;font-size:1rem"/>
          </div>
        {/if}

        <button on:click={selectDir} style="width:100%;padding:1.5rem;background:rgba(49,50,68,0.5);color:#cdd6f4;border:2px solid #313244;border-radius:14px;cursor:pointer;font-weight:600;font-size:1.1rem;transition:all 0.3s">
          <i class="nf-md-folder_open"></i> {outputDir ? 'Change Directory' : 'Select Output Directory'}
        </button>
        {#if outputDir}<p style="color:#cdd6f4;font-family:Consolas,monospace;background:#11111b;padding:1rem;border-radius:10px;text-align:center">{outputDir}</p>{/if}

        <button on:click={execute} style="width:100%;padding:1.5rem;background:linear-gradient(135deg, #a6e3a1 0%, #94e2d5 100%);color:#1e1e2e;border:none;border-radius:14px;cursor:pointer;font-weight:600;font-size:1.2rem;margin-top:1rem;box-shadow:0 4px 16px rgba(166,227,161,0.3)">
          <i class="nf-md-play"></i> {mode === 'split' ? 'Split File' : 'Merge Files'}
        </button>
      </div>
    </div>
  </div>
</main>
<style>
  @keyframes float { 0%, 100% { transform: translateY(0px); } 50% { transform: translateY(-10px); } }
  @keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }
  button:hover { transform: translateY(-3px); }
  input:focus { border-color: #f5c2e7; outline: none; }
</style>
