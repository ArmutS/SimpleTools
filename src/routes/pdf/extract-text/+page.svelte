<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  let filePath = "";
  let text = "";
  let isProcessing = false;

  async function extract() {
    if (!filePath) return;
    isProcessing = true;
    try {
      const result: any = await invoke("pdf_extract_text", { request: { file_path: filePath, pages: null } });
      text = result.text;
    } catch(e) { alert(e); }
    finally { isProcessing = false; }
  }
</script>

<main class="container">
  <div class="content">
    <div class="header">
      <div class="icon-wrapper"><i class="nf-md-text_box"></i></div>
      <h1>Extract Text</h1>
      <p class="subtitle">Extract plain text content from PDF</p>
    </div>

    <div class="split-view">
      <div class="input-pane">
        <button on:click={async () => { const s = await open({filters:[{name:'PDF',extensions:['pdf']}]}); if(s) { filePath=s as string; extract(); } }} class="btn-secondary" style="width:100%">
          {filePath ? 'Change File' : 'Select PDF File'}
        </button>
        {#if filePath}<p class="path">{filePath}</p>{/if}
      </div>
      
      {#if text}
        <div class="result-pane">
           <textarea readonly value={text}></textarea>
           <button class="btn-copy" on:click={()=>navigator.clipboard.writeText(text)}>Copy Text</button>
        </div>
      {/if}
    </div>
  </div>
</main>

<style>
  .container { min-height:100vh; padding:2rem; background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%); display:flex; flex-direction:column; }
  .content { flex:1; display:flex; flex-direction:column; width:100%; max-width:1000px; margin:0 auto; }
  .header { text-align:center; margin-bottom:2rem; }
  .icon-wrapper { display:inline-flex; width:60px; height:60px; background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%); border-radius:14px; align-items:center; justify-content:center; margin-bottom:1rem; box-shadow:0 8px 24px rgba(243,139,168,0.4); animation:float 3s ease-in-out infinite; font-size:2.5rem; color:#1e1e2e; }
  h1 { font-size:2rem; font-weight:bold; color:#cdd6f4; margin:0; }
  .subtitle { color:#a6adc8; margin:0; }
  
  .split-view { flex:1; display:flex; flex-direction:column; gap:1.5rem; min-height:0; }
  .input-pane { text-align:center; }
  .path { margin-top:0.5rem; color:#6c7086; font-size:0.9rem; font-family:'Consolas',monospace; }
  
  .result-pane { flex:1; display:flex; flex-direction:column; gap:1rem; min-height:300px; }
  textarea { flex:1; padding:1.5rem; background:#11111b; border:2px solid #313244; border-radius:16px; color:#cdd6f4; font-family:'Consolas',monospace; font-size:0.95rem; resize:none; outline:none; }
  textarea:focus { border-color:#f38ba8; }
  
  .btn-secondary { padding:1rem; background:#313244; color:#cdd6f4; border:none; border-radius:12px; cursor:pointer; font-weight:600; }
  .btn-copy { padding:1rem; background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%); color:#1e1e2e; border:none; border-radius:12px; cursor:pointer; font-weight:600; }

  @keyframes float { 0%, 100% { transform:translateY(0); } 50% { transform:translateY(-10px); } }
</style>
