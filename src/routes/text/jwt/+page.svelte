<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  let token = "";
  let result: any = null;
  let error = "";

  async function decode() {
    if (!token) { result=null; return; }
    try {
      result = await invoke("process_jwt_decode", { token });
      error = "";
    } catch (e) { error = e as string; result = null; }
  }
</script>

<main class="container">
  <div class="content">
    <div class="header">
      <div class="icon-wrapper"><i class="nf-md-key_variant"></i></div>
      <h1>JWT Decoder</h1>
      <p class="subtitle">Decode JSON Web Tokens (no secret required)</p>
    </div>

    <div class="input-section">
      <textarea bind:value={token} on:input={decode} placeholder="Paste your JWT token here (eyJ...)" style="height:120px"></textarea>
      {#if error}<p class="error"><i class="nf-md-alert_circle"></i> {error}</p>{/if}
    </div>

    {#if result}
      <div class="result-grid">
        <div class="card">
          <label>Header</label>
          <pre>{result.header}</pre>
        </div>
        <div class="card">
          <label>Payload</label>
          <pre>{result.payload}</pre>
        </div>
      </div>
    {/if}
  </div>
</main>

<style>
  .container { min-height:100vh; padding:3rem 2rem; background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%); }
  .content { max-width:1000px; margin:0 auto; }
  .header { text-align:center; margin-bottom:3rem; }
  .icon-wrapper { display:inline-flex; width:80px; height:80px; background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%); border-radius:18px; align-items:center; justify-content:center; margin-bottom:1.5rem; box-shadow:0 8px 24px rgba(203,166,247,0.4); animation:float 3s ease-in-out infinite; font-size:3rem; color:#1e1e2e; }
  h1 { background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%); -webkit-background-clip:text; -webkit-text-fill-color:transparent; font-size:2.5rem; margin-bottom:0.5rem; font-weight:bold; }
  .subtitle { color:#a6adc8; font-size:1.1rem; }
  
  .input-section { background:rgba(24,24,37,0.7); backdrop-filter:blur(10px); padding:2rem; border-radius:20px; border:1px solid rgba(203,166,247,0.15); box-shadow:0 8px 32px rgba(0,0,0,0.3); margin-bottom:2rem; animation:slideUp 0.5s ease-out; }
  textarea { width:100%; padding:1rem; background:#11111b; border:2px solid #313244; border-radius:12px; color:#cdd6f4; font-family:'Consolas',monospace; transition:all 0.3s; }
  textarea:focus { border-color:#cba6f7; outline:none; }
  .error { color:#f38ba8; margin-top:0.5rem; font-size:0.9rem; }
  
  .result-grid { display:grid; grid-template-columns:1fr 1fr; gap:1.5rem; }
  .card { background:rgba(24,24,37,0.7); padding:1.5rem; border-radius:16px; border:1px solid rgba(203,166,247,0.15); animation:slideUp 0.5s ease-out; }
  label { color:#cba6f7; font-weight:bold; display:block; margin-bottom:1rem; font-size:1.1rem; }
  pre { background:#11111b; padding:1rem; border-radius:10px; color:#a6e3a1; overflow:auto; font-family:'Consolas',monospace; font-size:0.9rem; max-height:400px; }
  
  @keyframes float { 0%, 100% { transform:translateY(0); } 50% { transform:translateY(-10px); } }
  @keyframes slideUp { from { opacity:0; transform:translateY(20px); } to { opacity:1; transform:translateY(0); } }
</style>
