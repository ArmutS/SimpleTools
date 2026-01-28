<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  
  let files: string[] = [];
  let target = "png";
  let quality = 90;
  let isProcessing = false;
  let maintainAspect = true;
  let resizeW: number|null = null;
  let resizeH: number|null = null;

  async function addFiles() {
    const selected = await open({ 
      multiple: true, 
      filters: [{ name: 'Images', extensions: ['png','jpg','jpeg','webp','gif','bmp','tiff'] }] 
    });
    if (selected) {
      if (Array.isArray(selected)) files = [...files, ...selected];
      else files = [...files, selected as string];
    }
  }

  async function convert() {
    if (files.length === 0) return;
    const dir = await open({ directory: true });
    if (!dir) return;

    isProcessing = true;
    try {
      for (const file of files) {
        if (resizeW || resizeH) {
             await invoke("resize_image", { 
                 filePath: file, 
                 outputDir: dir, 
                 targetFormat: target, 
                 width: resizeW || null, 
                 height: resizeH || null, 
                 maintainAspect 
             });
        } else {
             await invoke("convert_image", { 
                 filePath: file, 
                 outputDir: dir, 
                 targetFormat: target, 
                 quality 
             });
        }
      }
      alert(\`Processed \${files.length} images!\`);
      files = [];
    } catch(e) { alert(e); }
    finally { isProcessing = false; }
  }
</script>

<main class="container">
  <div class="content">
    <div class="header">
      <div class="icon-wrapper"><i class="nf-md-image_filter_hdr"></i></div>
      <h1>Image Converter</h1>
      <p class="subtitle">Batch convert and resize images</p>
    </div>

    <div class="card">
      <div class="input-area">
        <button on:click={addFiles} class="btn-input">
          {#if files.length > 0}
            <div class="file-grid">
               {#each files.slice(0, 4) as f}
                 <div class="f-badge">
                   <i class="nf-md-image"></i>
                   <span>{f.split(/[\\\\/]/).pop()}</span>
                 </div>
               {/each}
               {#if files.length > 4}
                 <div class="f-more">+{files.length - 4} more</div>
               {/if}
            </div>
            <div class="add-more"><i class="nf-md-plus_circle"></i> Add More</div>
          {:else}
             <div class="placeholder">
               <i class="nf-md-image_plus"></i>
               <span>Add Images</span>
               <small>Drag & Drop or Click</small>
             </div>
          {/if}
        </button>
        {#if files.length > 0}
            <button on:click={()=>files=[]} class="btn-clear">Clear All</button>
        {/if}
      </div>

      {#if files.length > 0}
        <div class="controls">
           <div class="row">
             <label>Format</label>
             <div class="fmt-grid">
               {#each ['png','jpg','webp','gif','bmp','tiff'] as f}
                 <button class:active={target===f} on:click={()=>target=f}>{f.toUpperCase()}</button>
               {/each}
             </div>
           </div>

           <div class="row advanced">
             <label>Resize (Optional)</label>
             <div class="resize-row">
                <input type="number" bind:value={resizeW} placeholder="Width" />
                <span>x</span>
                <input type="number" bind:value={resizeH} placeholder="Height" />
                <button class="toggle" class:active={maintainAspect} on:click={()=>maintainAspect=!maintainAspect}>
                  <i class="nf-md-aspect_ratio"></i> Aspect
                </button>
             </div>
           </div>
          
          <button on:click={convert} disabled={isProcessing} class="btn-primary">
            {#if isProcessing}
              <i class="nf-md-loading nf-spin"></i> Processing...
            {:else}
              <i class="nf-md-play"></i> Start Processing
            {/if}
          </button>
        </div>
      {/if}
    </div>
  </div>
</main>
<style>
  /* Teal Theme */
  .container { min-height:100vh; padding:3rem 2rem; background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%); }
  .content { max-width:700px; margin:0 auto; }
  .header { text-align:center; margin-bottom:3rem; }
  .icon-wrapper { display:inline-flex; width:80px; height:80px; background:linear-gradient(135deg, #94e2d5 0%, #89dceb 100%); border-radius:18px; align-items:center; justify-content:center; margin-bottom:1.5rem; box-shadow:0 8px 24px rgba(148,226,213,0.4); animation:float 3s ease-in-out infinite; font-size:3rem; color:#1e1e2e; }
  h1 { background:linear-gradient(135deg, #94e2d5 0%, #89dceb 100%); -webkit-background-clip:text; -webkit-text-fill-color:transparent; font-size:2.5rem; margin-bottom:0.5rem; font-weight:bold; }
  .subtitle { color:#a6adc8; font-size:1.1rem; }
  
  .card { background:rgba(24,24,37,0.7); backdrop-filter:blur(10px); padding:2rem; border-radius:24px; border:1px solid rgba(148,226,213,0.15); box-shadow:0 8px 32px rgba(0,0,0,0.3); animation:slideUp 0.5s ease-out; }
  
  .btn-input { width:100%; min-height:150px; background:#11111b; border:2px dashed #313244; border-radius:16px; cursor:pointer; transition:0.3s; color:#cdd6f4; position:relative; padding:1.5rem; }
  .btn-input:hover { border-color:#94e2d5; background:rgba(148,226,213,0.05); }
  
  .placeholder { display:flex; flex-direction:column; align-items:center; gap:0.5rem; width:100%; }
  .placeholder i { font-size:2.5rem; color:#6c7086; }
  .placeholder span { font-size:1.1rem; font-weight:bold; }
  
  .file-grid { display:flex; flex-wrap:wrap; gap:0.5rem; justify-content:center; }
  .f-badge { background:rgba(148,226,213,0.1); border:1px solid rgba(148,226,213,0.3); padding:0.4rem 0.8rem; border-radius:8px; display:flex; align-items:center; gap:0.5rem; font-size:0.9rem; color:#94e2d5; max-width:150px; }
  .f-badge span { overflow:hidden; text-overflow:ellipsis; white-space:nowrap; }
  .add-more { margin-top:1rem; color:#94e2d5; font-weight:bold; display:flex; align-items:center; gap:0.5rem; justify-content:center; width:100%; }
  
  .btn-clear { margin-top:0.5rem; background:none; border:none; color:#f38ba8; cursor:pointer; font-size:0.9rem; text-decoration:underline; width:100%; }
  
  .controls { margin-top:2rem; display:flex; flex-direction:column; gap:2rem; animation:fadeIn 0.3s ease-out; }
  .row { display:flex; flex-direction:column; gap:0.5rem; }
  label { color:#a6adc8; font-weight:600; font-size:0.9rem; text-transform:uppercase; letter-spacing:1px; }
  
  .fmt-grid { display:grid; grid-template-columns:repeat(3, 1fr); gap:0.75rem; }
  .fmt-grid button { padding:0.75rem; background:#11111b; border:2px solid #313244; color:#cdd6f4; border-radius:10px; cursor:pointer; font-weight:bold; transition:0.3s; }
  .fmt-grid button.active { border-color:#94e2d5; background:rgba(148,226,213,0.1); color:#94e2d5; }
  
  .resize-row { display:flex; gap:0.75rem; align-items:center; }
  input { flex:1; padding:0.75rem; background:#11111b; border:2px solid #313244; border-radius:10px; color:#cdd6f4; text-align:center; }
  input:focus { border-color:#94e2d5; outline:none; }
  .toggle { padding:0.75rem; background:#11111b; border:2px solid #313244; color:#6c7086; border-radius:10px; cursor:pointer; display:flex; align-items:center; gap:0.5rem; }
  .toggle.active { color:#94e2d5; border-color:#94e2d5; background:rgba(148,226,213,0.1); }
  
  .btn-primary { width:100%; padding:1.25rem; background:linear-gradient(135deg, #94e2d5 0%, #89dceb 100%); color:#1e1e2e; border:none; border-radius:12px; cursor:pointer; font-weight:bold; font-size:1.2rem; display:flex; align-items:center; justify-content:center; gap:0.75rem; transition:0.3s; }
  .btn-primary:hover { transform:translateY(-2px); box-shadow:0 8px 24px rgba(148,226,213,0.3); }
  
  @keyframes float { 0%, 100% { transform:translateY(0); } 50% { transform:translateY(-10px); } }
  @keyframes slideUp { from { opacity:0; transform:translateY(20px); } to { opacity:1; transform:translateY(0); } }
  @keyframes fadeIn { from { opacity:0; } to { opacity:1; } }
</style>
