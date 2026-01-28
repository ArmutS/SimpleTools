<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open, save } from "@tauri-apps/plugin-dialog";
  let filePath = "";
  let quality = "medium";
  let isProcessing = false;

  async function compress() {
    if (!filePath) return;
    const output = await save({ filters: [{ name: 'PDF', extensions: ['pdf'] }], defaultPath: 'compressed.pdf' });
    if (!output) return;

    isProcessing = true;
    try {
      const msg = await invoke("pdf_compress", { request: { file_path: filePath, output_path: output, quality } });
      alert(msg);
    } catch(e) { alert(e); }
    finally { isProcessing = false; }
  }
</script>

<main class="container">
  <div class="content">
    <div class="header">
      <div class="icon-wrapper"><i class="nf-md-compress"></i></div>
      <h1>PDF Compressor</h1>
      <p class="subtitle">Reduce PDF file size</p>
    </div>

    <div class="input-section">
      <button on:click={async () => { const s = await open({filters:[{name:'PDF',extensions:['pdf']}]}); if(s) filePath=s as string; }} class="btn-secondary" style="width:100%; margin-bottom:1.5rem">
        {filePath ? filePath : 'Select PDF File'}
      </button>

      {#if filePath}
        <button on:click={compress} disabled={isProcessing} class="btn-primary" style="width:100%; margin-top:1rem">
          {isProcessing ? 'Compressing...' : 'Compress PDF'}
        </button>
      {/if}
    </div>
  </div>
</main>

<style>
  /* Simplified styles */
  .container { min-height:100vh; padding:3rem 2rem; background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%); }
  .content { max-width:600px; margin:0 auto; }
  .header { text-align:center; margin-bottom:3rem; }
  .icon-wrapper { display:inline-flex; width:80px; height:80px; background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%); border-radius:18px; align-items:center; justify-content:center; margin-bottom:1.5rem; box-shadow:0 8px 24px rgba(243,139,168,0.4); animation:float 3s ease-in-out infinite; font-size:3rem; color:#1e1e2e; }
  h1 { background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%); -webkit-background-clip:text; -webkit-text-fill-color:transparent; font-size:2.5rem; margin-bottom:0.5rem; font-weight:bold; }
  .subtitle { color:#a6adc8; font-size:1.1rem; }
  .input-section { background:rgba(24,24,37,0.7); backdrop-filter:blur(10px); padding:2rem; border-radius:20px; border:1px solid rgba(243,139,168,0.15); box-shadow:0 8px 32px rgba(0,0,0,0.3); animation:slideUp 0.5s ease-out; }
  .btn-primary { padding:1.5rem; background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%); color:#1e1e2e; border:none; border-radius:12px; cursor:pointer; font-weight:bold; font-size:1.2rem; }
  .btn-secondary { padding:1rem; background:#313244; color:#cdd6f4; border:none; border-radius:12px; cursor:pointer; font-weight:600; }
  @keyframes float { 0%, 100% { transform:translateY(0); } 50% { transform:translateY(-10px); } }
  @keyframes slideUp { from { opacity:0; transform:translateY(20px); } to { opacity:1; transform:translateY(0); } }
</style>
