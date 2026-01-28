<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  
  let filePath = "";
  let fileName = "";
  let target = "json";
  let isProcessing = false;
  let activeTab = "conversion"; // conversion | preview

  async function selectFile() {
    const selected = await open({ 
      filters: [{ name: 'Data Files', extensions: ['json','yaml','yml','xml','csv','toml'] }] 
    });
    if (selected) {
      filePath = selected as string;
      fileName = filePath.split(/[\\\\/]/).pop() || "Selected File";
    }
  }

  async function convert() {
    if (!filePath) return;
    const dir = await open({ directory: true });
    if (!dir) return;

    isProcessing = true;
    try {
      const msg = await invoke("convert_data", { filePath, outputDir: dir, targetFormat: target });
      alert(msg);
    } catch(e) { alert(e); }
    finally { isProcessing = false; }
  }
</script>

<main class="container">
  <div class="content">
    <div class="header">
      <div class="icon-wrapper"><i class="nf-md-database_sync"></i></div>
      <h1>Data Converter</h1>
      <p class="subtitle">Universal converter for structured data files</p>
    </div>

    <div class="card">
      <div class="input-area">
        <button on:click={selectFile} class="btn-input">
          {#if filePath}
            <div class="file-info">
              <i class="nf-md-file_document"></i>
              <span>{fileName}</span>
            </div>
          {:else}
             <div class="placeholder">
               <i class="nf-md-cloud_upload"></i>
               <span>Click to Select File</span>
               <small>JSON, YAML, XML, CSV, TOML</small>
             </div>
          {/if}
        </button>
      </div>

      {#if filePath}
        <div class="controls">
          <label>Output Format</label>
          <div class="grid-fmt">
            {#each ['json', 'yaml', 'xml', 'csv', 'toml'] as fmt}
              <button class:active={target===fmt} on:click={()=>target=fmt}>{fmt.toUpperCase()}</button>
            {/each}
          </div>
          
          <button on:click={convert} disabled={isProcessing} class="btn-primary">
            {#if isProcessing}
              <i class="nf-md-loading nf-spin"></i> Converting...
            {:else}
              <i class="nf-md-swap_horizontal"></i> Convert to {target.toUpperCase()}
            {/if}
          </button>
        </div>
      {/if}
    </div>
  </div>
</main>

<style>
  .container { min-height:100vh; padding:3rem 2rem; background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%); }
  .content { max-width:700px; margin:0 auto; }
  .header { text-align:center; margin-bottom:3rem; }
  .icon-wrapper { display:inline-flex; width:80px; height:80px; background:linear-gradient(135deg, #94e2d5 0%, #89dceb 100%); border-radius:18px; align-items:center; justify-content:center; margin-bottom:1.5rem; box-shadow:0 8px 24px rgba(148,226,213,0.4); animation:float 3s ease-in-out infinite; font-size:3rem; color:#1e1e2e; }
  h1 { background:linear-gradient(135deg, #94e2d5 0%, #89dceb 100%); -webkit-background-clip:text; -webkit-text-fill-color:transparent; font-size:2.5rem; margin-bottom:0.5rem; font-weight:bold; }
  .subtitle { color:#a6adc8; font-size:1.1rem; }
  
  .card { background:rgba(24,24,37,0.7); backdrop-filter:blur(10px); padding:2rem; border-radius:24px; border:1px solid rgba(148,226,213,0.15); box-shadow:0 8px 32px rgba(0,0,0,0.3); animation:slideUp 0.5s ease-out; }
  
  .btn-input { width:100%; height:150px; background:#11111b; border:2px dashed #313244; border-radius:16px; cursor:pointer; transition:0.3s; display:flex; align-items:center; justify-content:center; color:#cdd6f4; overflow:hidden; }
  .btn-input:hover { border-color:#94e2d5; background:rgba(148,226,213,0.05); }
  
  .placeholder { display:flex; flex-direction:column; align-items:center; gap:0.5rem; }
  .placeholder i { font-size:2.5rem; color:#6c7086; }
  .placeholder span { font-size:1.1rem; font-weight:bold; }
  .placeholder small { color:#6c7086; }
  
  .file-info { display:flex; flex-direction:column; align-items:center; gap:0.5rem; color:#94e2d5; }
  .file-info i { font-size:2.5rem; }
  .file-info span { font-size:1.1rem; font-weight:bold; }
  
  .controls { margin-top:2rem; animation:fadeIn 0.3s ease-out; }
  label { display:block; color:#a6adc8; margin-bottom:1rem; font-weight:600; text-align:center; text-transform:uppercase; letter-spacing:1px; font-size:0.9rem; }
  
  .grid-fmt { display:grid; grid-template-columns:repeat(5, 1fr); gap:0.75rem; margin-bottom:2rem; }
  .grid-fmt button { padding:0.75rem; background:#11111b; border:2px solid #313244; color:#cdd6f4; border-radius:10px; cursor:pointer; font-weight:bold; transition:0.3s; font-size:0.9rem; }
  .grid-fmt button.active { border-color:#94e2d5; background:rgba(148,226,213,0.1); color:#94e2d5; transform:translateY(-2px); box-shadow:0 4px 12px rgba(148,226,213,0.2); }
  
  .btn-primary { width:100%; padding:1.25rem; background:linear-gradient(135deg, #94e2d5 0%, #89dceb 100%); color:#1e1e2e; border:none; border-radius:12px; cursor:pointer; font-weight:bold; font-size:1.2rem; display:flex; align-items:center; justify-content:center; gap:0.75rem; transition:0.3s; }
  .btn-primary:hover { transform:translateY(-2px); box-shadow:0 8px 24px rgba(148,226,213,0.3); }
  .btn-primary:disabled { opacity:0.7; cursor:not-allowed; transform:none; }
  
  @keyframes float { 0%, 100% { transform:translateY(0); } 50% { transform:translateY(-10px); } }
  @keyframes slideUp { from { opacity:0; transform:translateY(20px); } to { opacity:1; transform:translateY(0); } }
  @keyframes fadeIn { from { opacity:0; } to { opacity:1; } }
</style>
