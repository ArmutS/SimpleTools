<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open, save } from "@tauri-apps/plugin-dialog";
  let filePath = "";
  let pagesInput = "";
  let isProcessing = false;

  async function deletePages() {
    if (!filePath || !pagesInput) return;
    const output = await save({ filters: [{ name: 'PDF', extensions: ['pdf'] }], defaultPath: 'clean.pdf' });
    if (!output) return;

    // Parse pages like "1,3,5"
    const pagesToDelete = pagesInput.split(',').map(s => parseInt(s.trim())).filter(n => !isNaN(n));

    isProcessing = true;
    try {
      const msg = await invoke("pdf_delete_pages", { request: { file_path: filePath, output_path: output, pages_to_delete: pagesToDelete } });
      alert(msg);
    } catch(e) { alert(e); }
    finally { isProcessing = false; }
  }
</script>

<main class="container">
  <div class="content">
    <div class="header">
      <div class="icon-wrapper"><i class="nf-md-delete"></i></div>
      <h1>Delete PDF Pages</h1>
      <p class="subtitle">Remove specific pages from a PDF file</p>
    </div>

    <div class="input-section">
      <button on:click={async () => { const s = await open({filters:[{name:'PDF',extensions:['pdf']}]}); if(s) filePath=s as string; }} class="btn-secondary" style="width:100%; margin-bottom:1.5rem">
        {filePath ? filePath : 'Select PDF File'}
      </button>

      {#if filePath}
        <div style="margin-bottom:2rem">
            <label>Pages to Delete (comma separated, e.g. 1, 5, 12)</label>
            <input type="text" bind:value={pagesInput} placeholder="1, 2, 3..." />
        </div>

        <button on:click={deletePages} disabled={isProcessing} class="btn-primary" style="width:100%">
          {isProcessing ? 'Deleting...' : 'Delete Pages'}
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
  
  input { width:100%; padding:1rem; background:#11111b; border:2px solid #313244; border-radius:10px; color:#cdd6f4; font-size:1.2rem; text-align:center; }
  input:focus { outline:none; border-color:#f38ba8; }
  label { display:block; color:#f38ba8; margin-bottom:0.75rem; font-weight:600; text-align:center; }
  
  .btn-primary { padding:1.5rem; background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%); color:#1e1e2e; border:none; border-radius:12px; cursor:pointer; font-weight:bold; font-size:1.2rem; }
  .btn-secondary { padding:1rem; background:#313244; color:#cdd6f4; border:none; border-radius:12px; cursor:pointer; font-weight:600; }
  
  @keyframes float { 0%, 100% { transform:translateY(0); } 50% { transform:translateY(-10px); } }
  @keyframes slideUp { from { opacity:0; transform:translateY(20px); } to { opacity:1; transform:translateY(0); } }
</style>
