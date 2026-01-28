<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  let filePath = "";
  let format = "png";
  let dpi = 150;
  let isProcessing = false;

  async function selectFile() {
    const selected = await open({ filters: [{ name: 'PDF', extensions: ['pdf'] }] });
    if (selected) filePath = selected as string;
  }

  async function convert() {
    if (!filePath) return;
    const dir = await open({ directory: true });
    if (!dir) return;

    isProcessing = true;
    try {
      const msg = await invoke("pdf_to_images", { request: { file_path: filePath, output_dir: dir, format, dpi } });
      alert(msg);
    } catch(e) { alert(e); }
    finally { isProcessing = false; }
  }
</script>

<main class="container">
  <div class="content">
    <div class="header">
      <div class="icon-wrapper"><i class="nf-md-image_outline"></i></div>
      <h1>PDF to Images</h1>
      <p class="subtitle">Convert PDF pages to PNG or JPG images</p>
    </div>

    <div class="input-section">
      <button on:click={selectFile} class="btn-secondary" style="width:100%; margin-bottom:1.5rem">
        {filePath ? filePath : 'Select PDF File'}
      </button>

      {#if filePath}
        <div class="controls">
           <div class="grid">
             <div>
               <label>Format</label>
               <select bind:value={format}>
                 <option value="png">PNG</option>
                 <option value="jpg">JPG</option>
               </select>
             </div>
             <div>
               <label>Quality (DPI)</label>
               <select bind:value={dpi}>
                 <option value={72}>72 (Screen)</option>
                 <option value={150}>150 (Medium)</option>
                 <option value={300}>300 (Print)</option>
               </select>
             </div>
           </div>

          <button on:click={convert} disabled={isProcessing} class="btn-primary" style="width:100%; margin-top:2rem">
            {isProcessing ? 'Converting...' : 'Convert to Images'}
          </button>
        </div>
      {/if}
    </div>
  </div>
</main>

<style>
  /* Reuse standard styles */
  .container { min-height:100vh; padding:3rem 2rem; background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%); }
  .content { max-width:700px; margin:0 auto; }
  .header { text-align:center; margin-bottom:3rem; }
  .icon-wrapper { display:inline-flex; width:80px; height:80px; background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%); border-radius:18px; align-items:center; justify-content:center; margin-bottom:1.5rem; box-shadow:0 8px 24px rgba(243,139,168,0.4); animation:float 3s ease-in-out infinite; font-size:3rem; color:#1e1e2e; }
  h1 { background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%); -webkit-background-clip:text; -webkit-text-fill-color:transparent; font-size:2.5rem; margin-bottom:0.5rem; font-weight:bold; }
  .subtitle { color:#a6adc8; font-size:1.1rem; }
  
  .input-section { background:rgba(24,24,37,0.7); backdrop-filter:blur(10px); padding:2rem; border-radius:20px; border:1px solid rgba(243,139,168,0.15); box-shadow:0 8px 32px rgba(0,0,0,0.3); animation:slideUp 0.5s ease-out; }
  .btn-primary { padding:1.5rem; background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%); color:#1e1e2e; border:none; border-radius:12px; cursor:pointer; font-weight:bold; font-size:1.2rem; transition:0.3s; }
  .btn-secondary { padding:1rem; background:#313244; color:#cdd6f4; border:none; border-radius:12px; cursor:pointer; font-weight:600; width:100%; word-break:break-all; }
  
  .grid { display:grid; grid-template-columns:1fr 1fr; gap:1.5rem; }
  select { width:100%; padding:1rem; background:#11111b; border:2px solid #313244; border-radius:10px; color:#cdd6f4; }
  label { display:block; color:#a6adc8; margin-bottom:0.5rem; }
  
  @keyframes float { 0%, 100% { transform:translateY(0); } 50% { transform:translateY(-10px); } }
  @keyframes slideUp { from { opacity:0; transform:translateY(20px); } to { opacity:1; transform:translateY(0); } }
</style>
