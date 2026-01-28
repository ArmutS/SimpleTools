<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  let input = "";
  let output = "";
  let decodeEntities = true;

  async function strip() {
    if (!input) { output = ""; return; }
    try {
      output = await invoke("process_strip", { currentText: input, pureText: decodeEntities });
    } catch (e) { alert(e); }
  }
</script>

<main class="container">
  <div class="content">
    <div class="header">
      <div class="icon-wrapper"><i class="nf-fa-eraser"></i></div>
      <h1>HTML Strip</h1>
      <p class="subtitle">Remove HTML tags and clean text</p>
    </div>

    <div class="input-section">
      <div style="margin-bottom:1.5rem">
        <textarea bind:value={input} on:input={strip} placeholder="Paste HTML content here..." style="height:200px"></textarea>
      </div>
      <label class="checkbox">
        <input type="checkbox" bind:checked={decodeEntities} on:change={strip}> 
        Decode HTML entities (&amp;nbsp;, &amp;gt; etc.)
      </label>
    </div>

    {#if output}
      <div class="result-section">
        <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:1rem">
          <label><i class="nf-md-text_long"></i> Cleaned Text</label>
          <button class="btn-copy" on:click={() => navigator.clipboard.writeText(output)}>Copy Result</button>
        </div>
        <textarea readonly value={output} style="height:200px; background:#181825; border-color:#cba6f7"></textarea>
      </div>
    {/if}
  </div>
</main>

<style>
  /* Same styles again */
  .container { min-height:100vh; padding:3rem 2rem; background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%); }
  .content { max-width:900px; margin:0 auto; }
  .header { text-align:center; margin-bottom:3rem; }
  .icon-wrapper { display:inline-flex; width:80px; height:80px; background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%); border-radius:18px; align-items:center; justify-content:center; margin-bottom:1.5rem; box-shadow:0 8px 24px rgba(203,166,247,0.4); animation:float 3s ease-in-out infinite; font-size:3rem; color:#1e1e2e; }
  h1 { background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%); -webkit-background-clip:text; -webkit-text-fill-color:transparent; font-size:2.5rem; margin-bottom:0.5rem; font-weight:bold; }
  .subtitle { color:#a6adc8; font-size:1.1rem; }
  .input-section, .result-section { background:rgba(24,24,37,0.7); backdrop-filter:blur(10px); padding:2.5rem; border-radius:20px; border:1px solid rgba(203,166,247,0.15); box-shadow:0 8px 32px rgba(0,0,0,0.3); margin-bottom:2rem; animation:slideUp 0.5s ease-out; }
  
  textarea { width:100%; padding:1rem; background:#11111b; border:2px solid #313244; border-radius:12px; color:#cdd6f4; font-family:'Consolas',monospace; transition:all 0.3s; resize:vertical; }
  textarea:focus { border-color:#cba6f7; outline:none; }
  
  label { color:#cba6f7; font-weight:600; margin-bottom:0.75rem; display:flex; align-items:center; gap:0.5rem; }
  .checkbox { display:inline-flex; align-items:center; gap:0.5rem; color:#cdd6f4; cursor:pointer; user-select:none; }
  .checkbox input { width:auto; }
  
  .btn-copy { background:#313244; color:#cdd6f4; border:none; padding:0.5rem 1rem; border-radius:8px; cursor:pointer; font-weight:600; }
  .btn-copy:hover { background:#45475a; }
  
  @keyframes float { 0%, 100% { transform:translateY(0); } 50% { transform:translateY(-10px); } }
  @keyframes slideUp { from { opacity:0; transform:translateY(20px); } to { opacity:1; transform:translateY(0); } }
</style>
