<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  let filePath = "";
  let info: any = null;
  let loading = false;

  async function selectFile() {
    const selected = await open({ filters: [{ name: 'PDF', extensions: ['pdf'] }] });
    if (selected) { filePath = selected as string; getInfo(); }
  }

  async function getInfo() {
    loading = true;
    try {
      info = await invoke("get_pdf_info", { filePath });
    } catch (e) { alert(e); }
    finally { loading = false; }
  }
</script>

<main class="container">
  <div class="content">
    <div class="header">
      <div class="icon-wrapper"><i class="nf-md-information"></i></div>
      <h1>PDF Info</h1>
      <p class="subtitle">View detailed metadata of PDF files</p>
    </div>

    <div class="input-section">
      <button on:click={selectFile} class="btn-primary" style="width:100%"><i class="nf-md-file_pdf_box"></i> Select PDF File</button>
      {#if filePath}<p class="path">{filePath}</p>{/if}
    </div>

    {#if loading}
       <div style="text-align:center;color:#eba0ac">Loading info...</div>
    {:else if info}
      <div class="result-section">
        {#if info.error}
           <p style="color:#f38ba8"><i class="nf-md-alert"></i> {info.error}</p>
        {:else}
           <div class="grid">
             <div class="card">
               <label>Page Count</label>
               <div class="val">{info.page_count}</div>
             </div>
             <div class="card">
               <label>File Size</label>
               <div class="val">{info.file_size_formatted}</div>
             </div>
             <div class="card">
               <label>Encrypted</label>
               <div class="val">{info.is_encrypted ? 'Yes' : 'No'}</div>
             </div>
           </div>
        {/if}
      </div>
    {/if}
  </div>
</main>

<style>
  .container { min-height:100vh; padding:3rem 2rem; background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%); }
  .content { max-width:800px; margin:0 auto; }
  .header { text-align:center; margin-bottom:3rem; }
  .icon-wrapper { display:inline-flex; width:80px; height:80px; background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%); border-radius:18px; align-items:center; justify-content:center; margin-bottom:1.5rem; box-shadow:0 8px 24px rgba(243,139,168,0.4); animation:float 3s ease-in-out infinite; font-size:3rem; color:#1e1e2e; }
  h1 { background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%); -webkit-background-clip:text; -webkit-text-fill-color:transparent; font-size:2.5rem; margin-bottom:0.5rem; font-weight:bold; }
  .subtitle { color:#a6adc8; font-size:1.1rem; }
  
  .input-section, .result-section { background:rgba(24,24,37,0.7); backdrop-filter:blur(10px); padding:2rem; border-radius:20px; border:1px solid rgba(243,139,168,0.15); box-shadow:0 8px 32px rgba(0,0,0,0.3); margin-bottom:2rem; animation:slideUp 0.5s ease-out; }
  
  .btn-primary { padding:1.5rem; background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%); color:#1e1e2e; border:none; border-radius:12px; cursor:pointer; font-weight:bold; font-size:1.2rem; display:flex; align-items:center; justify-content:center; gap:0.75rem; transition:0.3s; }
  .btn-primary:hover { transform:translateY(-3px); box-shadow:0 10px 25px rgba(243,139,168,0.4); }
  .path { color:#cdd6f4; background:#11111b; padding:1rem; border-radius:10px; margin-top:1rem; text-align:center; font-family:'Consolas',monospace; font-size:0.9rem; word-break:break-all; }
  
  .grid { display:grid; grid-template-columns:repeat(3, 1fr); gap:1rem; }
  .card { background:#11111b; padding:1.5rem; border-radius:12px; text-align:center; border:2px solid #313244; }
  .val { font-size:1.8rem; font-weight:bold; color:#f38ba8; margin-top:0.5rem; }
  label { color:#a6adc8; font-size:0.9rem; font-weight:600; text-transform:uppercase; }
  
  @keyframes float { 0%, 100% { transform:translateY(0); } 50% { transform:translateY(-10px); } }
  @keyframes slideUp { from { opacity:0; transform:translateY(20px); } to { opacity:1; transform:translateY(0); } }
</style>
