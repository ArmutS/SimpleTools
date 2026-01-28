<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open, save } from "@tauri-apps/plugin-dialog";
  let files: string[] = [];
  let isProcessing = false;

  async function addFiles() {
    const selected = await open({ multiple: true, filters: [{ name: 'PDF', extensions: ['pdf'] }] });
    if (selected) {
      if (Array.isArray(selected)) files = [...files, ...selected];
      else files = [...files, selected as string];
    }
  }

  async function merge() {
    if (files.length < 2) return alert("Need at least 2 files");
    const output = await save({ filters: [{ name: 'PDF', extensions: ['pdf'] }], defaultPath: 'merged.pdf' });
    if (!output) return;
    
    isProcessing = true;
    try {
      const msg = await invoke("pdf_merge", { request: { files, output_path: output } });
      alert(msg);
      files = [];
    } catch(e) { alert(e); }
    finally { isProcessing = false; }
  }
</script>

<main class="container">
  <div class="content">
    <div class="header">
      <div class="icon-wrapper"><i class="nf-md-file_multiple"></i></div>
      <h1>PDF Merger</h1>
      <p class="subtitle">Combine multiple PDF files into one</p>
    </div>

    <div class="input-section">
      <button on:click={addFiles} class="btn-secondary" style="width:100%; margin-bottom:1.5rem"><i class="nf-md-plus"></i> Add Files</button>
      
      {#if files.length > 0}
        <div class="file-list">
          {#each files as file, i}
             <div class="file-item">
               <span>{i+1}.</span>
               <span class="path">{file}</span>
               <button class="btn-del" on:click={()=>files = files.filter((_, idx) => idx !== i)}><i class="nf-md-close"></i></button>
             </div>
          {/each}
        </div>
        
        <button on:click={merge} disabled={isProcessing} class="btn-primary" style="width:100%; margin-top:2rem">
          {isProcessing ? 'Merging...' : 'Merge PDFs'}
        </button>
      {:else}
        <div class="empty-state">No files selected</div>
      {/if}
    </div>
  </div>
</main>

<style>
  /* Reuse basic styles */
  .container { min-height:100vh; padding:3rem 2rem; background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%); }
  .content { max-width:800px; margin:0 auto; }
  .header { text-align:center; margin-bottom:3rem; }
  .icon-wrapper { display:inline-flex; width:80px; height:80px; background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%); border-radius:18px; align-items:center; justify-content:center; margin-bottom:1.5rem; box-shadow:0 8px 24px rgba(243,139,168,0.4); animation:float 3s ease-in-out infinite; font-size:3rem; color:#1e1e2e; }
  h1 { background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%); -webkit-background-clip:text; -webkit-text-fill-color:transparent; font-size:2.5rem; margin-bottom:0.5rem; font-weight:bold; }
  .subtitle { color:#a6adc8; font-size:1.1rem; }
  
  .input-section { background:rgba(24,24,37,0.7); backdrop-filter:blur(10px); padding:2rem; border-radius:20px; border:1px solid rgba(243,139,168,0.15); box-shadow:0 8px 32px rgba(0,0,0,0.3); animation:slideUp 0.5s ease-out; }
  
  .btn-primary { padding:1.5rem; background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%); color:#1e1e2e; border:none; border-radius:12px; cursor:pointer; font-weight:bold; font-size:1.2rem; transition:0.3s; }
  .btn-primary:hover { transform:translateY(-3px); box-shadow:0 10px 25px rgba(243,139,168,0.4); }
  .btn-secondary { padding:1rem; background:#313244; color:#cdd6f4; border:none; border-radius:12px; cursor:pointer; font-weight:600; }
  
  .file-list { display:flex; flex-direction:column; gap:0.5rem; max-height:400px; overflow-y:auto; }
  .file-item { background:#11111b; padding:1rem; border-radius:10px; display:flex; align-items:center; gap:1rem; color:#cdd6f4; font-family:'Consolas',monospace; font-size:0.9rem; }
  .path { flex:1; overflow:hidden; text-overflow:ellipsis; white-space:nowrap; direction:rtl; text-align:left; }
  .btn-del { background:none; border:none; color:#f38ba8; cursor:pointer; font-size:1.2rem; }
  
  .empty-state { text-align:center; color:#6c7086; padding:2rem; border:2px dashed #313244; border-radius:12px; }
  
  @keyframes float { 0%, 100% { transform:translateY(0); } 50% { transform:translateY(-10px); } }
  @keyframes slideUp { from { opacity:0; transform:translateY(20px); } to { opacity:1; transform:translateY(0); } }
</style>
