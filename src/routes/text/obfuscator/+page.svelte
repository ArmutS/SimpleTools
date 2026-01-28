<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  let input = "";
  let output = "";
  let mode = "rot13";

  async function obfuscate() {
    if (!input) { output=""; return; }
    try {
      output = await invoke("process_obfuscator", { currentText: input, mode });
    } catch(e) { alert(e); }
  }
</script>

<main class="container">
  <div class="content">
    <div class="header">
      <div class="icon-wrapper"><i class="nf-fa-user_secret"></i></div>
      <h1>Text Obfuscator</h1>
      <p class="subtitle">Scramble text using common reversible algorithms</p>
    </div>

    <div class="input-section">
      <textarea bind:value={input} on:input={obfuscate} placeholder="Enter text to obfuscate..." style="height:150px; margin-bottom:1.5rem"></textarea>
      
      <div class="controls">
        <button class:active={mode==='rot13'} on:click={()=>{mode='rot13';obfuscate()}}>ROT13</button>
        <button class:active={mode==='base64'} on:click={()=>{mode='base64';obfuscate()}}>Base64</button>
        <button class:active={mode==='reverse'} on:click={()=>{mode='reverse';obfuscate()}}>Reverse</button>
      </div>
    </div>

    {#if output}
      <div class="result-section">
        <label>Obfuscated Result</label>
        <textarea readonly value={output} style="height:150px; background:#181825; border-color:#cba6f7"></textarea>
        <button class="btn-copy" on:click={() => navigator.clipboard.writeText(output)} style="margin-top:1rem; width:100%">Copy Result</button>
      </div>
    {/if}
  </div>
</main>

<style>
  /* Same styles */
  .container { min-height:100vh; padding:3rem 2rem; background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%); }
  .content { max-width:800px; margin:0 auto; }
  .header { text-align:center; margin-bottom:3rem; }
  .icon-wrapper { display:inline-flex; width:80px; height:80px; background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%); border-radius:18px; align-items:center; justify-content:center; margin-bottom:1.5rem; box-shadow:0 8px 24px rgba(203,166,247,0.4); animation:float 3s ease-in-out infinite; font-size:3rem; color:#1e1e2e; }
  h1 { background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%); -webkit-background-clip:text; -webkit-text-fill-color:transparent; font-size:2.5rem; margin-bottom:0.5rem; font-weight:bold; }
  .subtitle { color:#a6adc8; font-size:1.1rem; }
  
  .input-section, .result-section { background:rgba(24,24,37,0.7); backdrop-filter:blur(10px); padding:2rem; border-radius:20px; border:1px solid rgba(203,166,247,0.15); box-shadow:0 8px 32px rgba(0,0,0,0.3); margin-bottom:2rem; animation:slideUp 0.5s ease-out; }
  textarea { width:100%; padding:1rem; background:#11111b; border:2px solid #313244; border-radius:12px; color:#cdd6f4; font-family:'Consolas',monospace; transition:all 0.3s; }
  textarea:focus { border-color:#cba6f7; outline:none; }
  label { color:#cba6f7; font-weight:600; margin-bottom:0.75rem; display:block; }
  
  .controls { display:flex; gap:1rem; justify-content:center; }
  button { padding:0.75rem 2rem; background:#11111b; border:2px solid #313244; color:#cdd6f4; border-radius:10px; cursor:pointer; font-weight:600; transition:all 0.3s; }
  button.active { border-color:#cba6f7; background:rgba(203,166,247,0.1); color:#cba6f7; }
  .btn-copy { background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%); color:#1e1e2e; font-weight:bold; border:none; }
  
  @keyframes float { 0%, 100% { transform:translateY(0); } 50% { transform:translateY(-10px); } }
  @keyframes slideUp { from { opacity:0; transform:translateY(20px); } to { opacity:1; transform:translateY(0); } }
</style>
