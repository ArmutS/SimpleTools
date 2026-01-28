<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open, save } from "@tauri-apps/plugin-dialog";
  let images: string[] = [];
  let isProcessing = false;

  async function addImages() {
    const selected = await open({ multiple: true, filters: [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg'] }] });
    if (selected) {
      if (Array.isArray(selected)) images = [...images, ...selected];
      else images = [...images, selected as string];
    }
  }

  async function convert() {
    if (images.length < 1) return;
    const output = await save({ filters: [{ name: 'PDF', extensions: ['pdf'] }], defaultPath: 'images.pdf' });
    if (!output) return;

    isProcessing = true;
    try {
      const msg = await invoke("images_to_pdf", { request: { image_paths: images, output_path: output, _page_size: "A4" } });
      alert(msg);
      images = [];
    } catch(e) { alert(e); }
    finally { isProcessing = false; }
  }
</script>

<main class="container">
  <div class="content">
    <div class="header">
      <div class="icon-wrapper"><i class="nf-md-image_multiple"></i></div>
      <h1>Images to PDF</h1>
      <p class="subtitle">Convert images to a single PDF file</p>
    </div>

    <div class="input-section">
      <button on:click={addImages} class="btn-secondary" style="width:100%; margin-bottom:1.5rem"><i class="nf-md-plus"></i> Add Images</button>
      
      {#if images.length > 0}
        <div class="list">
          {#each images as img, i}
             <div class="item">
               <span>{i+1}.</span>
               <span class="path">{img}</span>
               <button class="btn-del" on:click={()=>images = images.filter((_, idx) => idx !== i)}><i class="nf-md-close"></i></button>
             </div>
          {/each}
        </div>
        
        <button on:click={convert} disabled={isProcessing} class="btn-primary" style="width:100%; margin-top:2rem">
          {isProcessing ? 'Converting...' : 'Create PDF'}
        </button>
      {:else}
        <div class="empty-state">No images selected</div>
      {/if}
    </div>
  </div>
</main>

<style>
  /* Same style set */
  .container { min-height:100vh; padding:3rem 2rem; background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%); }
  .content { max-width:800px; margin:0 auto; }
  .header { text-align:center; margin-bottom:3rem; }
  .icon-wrapper { display:inline-flex; width:80px; height:80px; background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%); border-radius:18px; align-items:center; justify-content:center; margin-bottom:1.5rem; box-shadow:0 8px 24px rgba(243,139,168,0.4); animation:float 3s ease-in-out infinite; font-size:3rem; color:#1e1e2e; }
  h1 { background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%); -webkit-background-clip:text; -webkit-text-fill-color:transparent; font-size:2.5rem; margin-bottom:0.5rem; font-weight:bold; }
  .subtitle { color:#a6adc8; font-size:1.1rem; }
  
  .input-section { background:rgba(24,24,37,0.7); backdrop-filter:blur(10px); padding:2rem; border-radius:20px; border:1px solid rgba(243,139,168,0.15); box-shadow:0 8px 32px rgba(0,0,0,0.3); animation:slideUp 0.5s ease-out; }
  
  .btn-primary { padding:1.5rem; background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%); color:#1e1e2e; border:none; border-radius:12px; cursor:pointer; font-weight:bold; font-size:1.2rem; transition:0.3s; }
  .btn-secondary { padding:1rem; background:#313244; color:#cdd6f4; border:none; border-radius:12px; cursor:pointer; font-weight:600; }
  
  .list { display:flex; flex-direction:column; gap:0.5rem; max-height:400px; overflow-y:auto; }
  .item { background:#11111b; padding:1rem; border-radius:10px; display:flex; align-items:center; gap:1rem; color:#cdd6f4; font-family:'Consolas',monospace; font-size:0.9rem; }
  .path { flex:1; overflow:hidden; text-overflow:ellipsis; white-space:nowrap; direction:rtl; text-align:left; }
  .btn-del { background:none; border:none; color:#f38ba8; cursor:pointer; font-size:1.2rem; }
  .empty-state { text-align:center; color:#6c7086; padding:2rem; border:2px dashed #313244; border-radius:12px; }
  
  @keyframes float { 0%, 100% { transform:translateY(0); } 50% { transform:translateY(-10px); } }
  @keyframes slideUp { from { opacity:0; transform:translateY(20px); } to { opacity:1; transform:translateY(0); } }
</style>
