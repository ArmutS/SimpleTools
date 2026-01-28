<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  let input = "# Hello World\n\nType some markdown here...";
  let html = "";

  async function render() {
    try {
      html = await invoke("process_markdown_preview", { currentText: input });
    } catch(e) { /* ignore */ }
  }
  
  $: { input; render(); }
</script>

<main class="container">
  <div class="content">
    <div class="header">
      <div class="icon-wrapper"><i class="nf-md-language_markdown"></i></div>
      <h1>Markdown Editor</h1>
      <p class="subtitle">Live preview markdown rendering</p>
    </div>

    <div class="grid">
      <div class="pane">
        <label><i class="nf-md-pencil"></i> Editor</label>
        <textarea bind:value={input}></textarea>
      </div>
      <div class="pane">
        <label><i class="nf-md-eye"></i> Preview</label>
        <div class="preview">
          {@html html}
        </div>
      </div>
    </div>
  </div>
</main>

<style>
  .container { min-height:100vh; padding:2rem; background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%); display:flex; flex-direction:column; }
  .content { flex:1; display:flex; flex-direction:column; width:100%; max-width:1400px; margin:0 auto; }
  .header { text-align:center; margin-bottom:2rem; }
  .icon-wrapper { display:inline-flex; width:60px; height:60px; background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%); border-radius:14px; align-items:center; justify-content:center; margin-bottom:1rem; box-shadow:0 8px 24px rgba(203,166,247,0.4); animation:float 3s ease-in-out infinite; font-size:2.5rem; color:#1e1e2e; }
  h1 { font-size:2rem; font-weight:bold; color:#cdd6f4; margin:0; }
  .subtitle { color:#a6adc8; margin:0; }
  
  .grid { display:grid; grid-template-columns:1fr 1fr; gap:1.5rem; flex:1; min-height:0; }
  .pane { display:flex; flex-direction:column; gap:0.5rem; }
  
  textarea { flex:1; padding:1.5rem; background:#11111b; border:2px solid #313244; border-radius:16px; color:#cdd6f4; font-family:'Consolas',monospace; font-size:1rem; resize:none; outline:none; }
  textarea:focus { border-color:#cba6f7; }
  
  .preview { flex:1; padding:1.5rem; background:rgba(255,255,255,0.9); color:#1e1e2e; border-radius:16px; overflow-y:auto; font-family:sans-serif; line-height:1.6; }
  /* Simple prose styles for preview */
  :global(.preview h1) { font-size:2rem; margin-bottom:1rem; border-bottom:2px solid #ddd; padding-bottom:0.5rem; }
  :global(.preview h2) { font-size:1.5rem; margin-bottom:0.75rem; }
  :global(.preview code) { background:#eee; padding:0.2rem 0.4rem; border-radius:4px; font-family:monospace; }
  :global(.preview pre) { background:#222; color:#fff; padding:1rem; border-radius:8px; overflow-x:auto; }
  :global(.preview pre code) { background:none; padding:0; color:inherit; }
  :global(.preview blockquote) { border-left:4px solid #cba6f7; padding-left:1rem; margin:1rem 0; color:#555; }
  
  label { color:#cba6f7; font-weight:600; display:flex; align-items:center; gap:0.5rem; }
  @keyframes float { 0%, 100% { transform:translateY(0); } 50% { transform:translateY(-10px); } }
</style>
