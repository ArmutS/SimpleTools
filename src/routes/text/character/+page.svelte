<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  let input = "";
  let chars: any[] = [];

  async function inspect() {
    if (!input) { chars=[]; return; }
    try {
      chars = await invoke("process_char_inspector", { currentText: input });
    } catch(e) { }
  }
</script>

<main class="container">
  <div class="content">
    <div class="header">
      <div class="icon-wrapper"><i class="nf-fa-info_circle"></i></div>
      <h1>Character Inspector</h1>
      <p class="subtitle">Analyze characters, unicode values and HTML entities</p>
    </div>

    <div class="input-section">
      <input type="text" bind:value={input} on:input={inspect} placeholder="Type or paste characters..." />
    </div>

    <div class="grid">
      {#each chars as char, i}
        <div class="char-card" style="animation-delay:{i*0.03}s">
          <div class="char-display">{char.char}</div>
          <div class="char-info">
            <div class="info-row"><span>U+</span> <strong>{char.unicode.replace('U+', '')}</strong></div>
            <div class="info-row"><span>Dec</span> <strong>{char.decimal}</strong></div>
            <div class="info-row"><span>HTML</span> <strong>{char.entity}</strong></div>
          </div>
        </div>
      {/each}
    </div>
  </div>
</main>

<style>
  .container { min-height:100vh; padding:3rem 2rem; background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%); }
  .content { max-width:1000px; margin:0 auto; }
  .header { text-align:center; margin-bottom:3rem; }
  .icon-wrapper { display:inline-flex; width:80px; height:80px; background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%); border-radius:18px; align-items:center; justify-content:center; margin-bottom:1.5rem; box-shadow:0 8px 24px rgba(203,166,247,0.4); animation:float 3s ease-in-out infinite; font-size:3rem; color:#1e1e2e; }
  h1 { background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%); -webkit-background-clip:text; -webkit-text-fill-color:transparent; font-size:2.5rem; margin-bottom:0.5rem; font-weight:bold; }
  .subtitle { color:#a6adc8; font-size:1.1rem; }
  
  .input-section { margin-bottom:3rem; }
  input { width:100%; padding:1.5rem; background:rgba(24,24,37,0.7); backdrop-filter:blur(10px); border:2px solid #313244; border-radius:20px; color:#cdd6f4; font-size:1.5rem; text-align:center; transition:all 0.3s; box-shadow:0 8px 32px rgba(0,0,0,0.3); }
  input:focus { border-color:#cba6f7; outline:none; transform:translateY(-2px); box-shadow:0 12px 40px rgba(203,166,247,0.2); }
  
  .grid { display:grid; grid-template-columns:repeat(auto-fill, minmax(180px, 1fr)); gap:1.5rem; }
  .char-card { background:rgba(24,24,37,0.7); backdrop-filter:blur(10px); padding:1.5rem; border-radius:16px; border:1px solid rgba(203,166,247,0.15); text-align:center; animation:fadeIn 0.4s ease-out backwards; transition:transform 0.2s; }
  .char-card:hover { transform:translateY(-5px); border-color:#cba6f7; background:rgba(203,166,247,0.05); }
  
  .char-display { font-size:3rem; margin-bottom:1rem; height:80px; display:flex; align-items:center; justify-content:center; color:#cdd6f4; }
  .char-info { display:flex; flex-direction:column; gap:0.5rem; font-size:0.9rem; }
  .info-row { display:flex; justify-content:space-between; color:#a6adc8; border-bottom:1px solid #313244; padding-bottom:0.25rem; }
  .info-row strong { color:#cba6f7; font-family:'Consolas',monospace; }
  
  @keyframes float { 0%, 100% { transform:translateY(0); } 50% { transform:translateY(-10px); } }
  @keyframes fadeIn { from { opacity:0; transform:translateY(20px); } to { opacity:1; transform:translateY(0); } }
</style>
