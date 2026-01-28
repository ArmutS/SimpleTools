<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open, save } from "@tauri-apps/plugin-dialog";
  let filePath = "";
  let rotation = 90;
  let isProcessing = false;

  async function rotate() {
    if (!filePath) return;
    const output = await save({ filters: [{ name: 'PDF', extensions: ['pdf'] }], defaultPath: 'rotated.pdf' });
    if (!output) return;

    isProcessing = true;
    try {
      const msg = await invoke("pdf_rotate", { request: { file_path: filePath, output_path: output, pages: [], rotation } });
      alert(msg);
    } catch(e) { alert(e); }
    finally { isProcessing = false; }
  }
</script>

<main class="container">
  <div class="content">
    <div class="header">
      <div class="icon-wrapper"><i class="nf-md-rotate_right"></i></div>
      <h1>Rotate PDF</h1>
      <p class="subtitle">Rotate pages by 90/180/270 degrees</p>
    </div>

    <div class="input-section">
      <button on:click={async () => { const s = await open({filters:[{name:'PDF',extensions:['pdf']}]}); if(s) filePath=s as string; }} class="btn-secondary" style="width:100%; margin-bottom:1.5rem">
        {filePath ? filePath : 'Select PDF File'}
      </button>

      {#if filePath}
        <div style="display:flex; gap:1rem; justify-content:center; margin-bottom:2rem">
          <button class:active={rotation===90} on:click={()=>rotation=90}>90° CW</button>
          <button class:active={rotation===180} on:click={()=>rotation=180}>180°</button>
          <button class:active={rotation===270} on:click={()=>rotation=270}>90° CCW</button>
        </div>

        <button on:click={rotate} disabled={isProcessing} class="btn-primary" style="width:100%">
          {isProcessing ? 'Rotating...' : 'Rotate Pages'}
        </button>
      {/if}
    </div>
  </div>
</main>

<style>
  /* Standard styles */
  .container { min-height:100vh; padding:3rem 2rem; background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%); }
  .content { max-width:600px; margin:0 auto; }
  .header { text-align:center; margin-bottom:3rem; }
  .icon-wrapper { display:inline-flex; width:80px; height:80px; background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%); border-radius:18px; align-items:center; justify-content:center; margin-bottom:1.5rem; box-shadow:0 8px 24px rgba(243,139,168,0.4); animation:float 3s ease-in-out infinite; font-size:3rem; color:#1e1e2e; }
  h1 { background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%); -webkit-background-clip:text; -webkit-text-fill-color:transparent; font-size:2.5rem; margin-bottom:0.5rem; font-weight:bold; }
  .subtitle { color:#a6adc8; font-size:1.1rem; }
  .input-section { background:rgba(24,24,37,0.7); backdrop-filter:blur(10px); padding:2rem; border-radius:20px; border:1px solid rgba(243,139,168,0.15); box-shadow:0 8px 32px rgba(0,0,0,0.3); animation:slideUp 0.5s ease-out; }
  
  button { padding:1rem; background:#11111b; border:2px solid #313244; color:#cdd6f4; border-radius:10px; cursor:pointer; transition:0.3s; flex:1; }
  button.active { border-color:#f38ba8; background:rgba(243,139,168,0.1); color:#f38ba8; }
  .btn-primary { padding:1.5rem; background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%); color:#1e1e2e; border:none; font-weight:bold; font-size:1.2rem; }
  .btn-secondary { background:#313244; color:#cdd6f4; border:none; }
  
  @keyframes float { 0%, 100% { transform:translateY(0); } 50% { transform:translateY(-10px); } }
  @keyframes slideUp { from { opacity:0; transform:translateY(20px); } to { opacity:1; transform:translateY(0); } }
</style>
