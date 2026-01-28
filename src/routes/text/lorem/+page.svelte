<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  let count = 3;
  let mode = "paragraphs";
  let output = "";

  async function generate() {
    try {
      output = await invoke("process_lorem", { count, mode });
    } catch(e) { alert(e); }
  }
</script>

<main class="container">
  <div class="content">
    <div class="header">
      <div class="icon-wrapper"><i class="nf-fa-paragraph"></i></div>
      <h1>Lorem Ipsum</h1>
      <p class="subtitle">Generate placeholder text for your designs</p>
    </div>

    <div class="input-section" style="display:flex; gap:2rem; align-items:center; justify-content:space-between">
      <div class="control-group">
        <label>Count ({count})</label>
        <input type="range" min="1" max="50" bind:value={count} on:change={generate} />
      </div>
      
      <div class="controls">
        <button class:active={mode==='words'} on:click={()=>{mode='words';generate()}}>Words</button>
        <button class:active={mode==='sentences'} on:click={()=>{mode='sentences';generate()}}>Sentences</button>
        <button class:active={mode==='paragraphs'} on:click={()=>{mode='paragraphs';generate()}}>Paragraphs</button>
      </div>
      
      <button class="btn-primary" on:click={generate}>Generate</button>
    </div>

    {#if output}
      <div class="result-section">
        <textarea readonly value={output} style="height:400px; background:#181825; border-color:#cba6f7"></textarea>
        <button class="btn-copy" on:click={() => navigator.clipboard.writeText(output)} style="margin-top:1rem; width:100%">Copy Text</button>
      </div>
    {/if}
  </div>
</main>

<style>
  /* Standard styles */
  .container { min-height:100vh; padding:3rem 2rem; background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%); }
  .content { max-width:900px; margin:0 auto; }
  .header { text-align:center; margin-bottom:3rem; }
  .icon-wrapper { display:inline-flex; width:80px; height:80px; background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%); border-radius:18px; align-items:center; justify-content:center; margin-bottom:1.5rem; box-shadow:0 8px 24px rgba(203,166,247,0.4); animation:float 3s ease-in-out infinite; font-size:3rem; color:#1e1e2e; }
  h1 { background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%); -webkit-background-clip:text; -webkit-text-fill-color:transparent; font-size:2.5rem; margin-bottom:0.5rem; font-weight:bold; }
  .subtitle { color:#a6adc8; font-size:1.1rem; }
  
  .input-section, .result-section { background:rgba(24,24,37,0.7); backdrop-filter:blur(10px); padding:2rem; border-radius:20px; border:1px solid rgba(203,166,247,0.15); box-shadow:0 8px 32px rgba(0,0,0,0.3); margin-bottom:2rem; animation:slideUp 0.5s ease-out; }
  textarea { width:100%; padding:1rem; background:#11111b; border:2px solid #313244; border-radius:12px; color:#cdd6f4; font-family:'Consolas',monospace; transition:all 0.3s; resize:vertical; }
  textarea:focus { border-color:#cba6f7; outline:none; }
  
  .controls { display:flex; gap:0.5rem; }
  button { padding:0.75rem 1.5rem; background:#11111b; border:2px solid #313244; color:#cdd6f4; border-radius:10px; cursor:pointer; font-weight:600; transition:all 0.3s; }
  button.active { border-color:#cba6f7; background:rgba(203,166,247,0.1); color:#cba6f7; }
  .btn-primary { background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%); color:#1e1e2e; border:none; }
  .btn-copy { background:#313244; border:none; }
  
  input[type="range"] { accent-color:#cba6f7; width:200px; }
  label { color:#cba6f7; font-weight:600; display:block; margin-bottom:0.5rem; }
  
  @keyframes float { 0%, 100% { transform:translateY(0); } 50% { transform:translateY(-10px); } }
  @keyframes slideUp { from { opacity:0; transform:translateY(20px); } to { opacity:1; transform:translateY(0); } }
</style>
