<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  let regexPattern = "";
  let testString = "";
  let flags = "gm";
  let matches: any[] = [];
  let error = "";

  async function testRegex() {
    if (!regexPattern) return;
    error = "";
    try {
      matches = await invoke("process_text_reg", { currentRegex: regexPattern, currentText: testString, currentFlags: flags });
    } catch (e) {
      matches = [];
      error = e as string;
    }
  }
</script>

<main class="container">
  <div class="content">
    <div class="header">
      <div class="icon-wrapper"><i class="nf-fa-code"></i></div>
      <h1>Regex Tester</h1>
      <p class="subtitle">Test regular expressions in real-time</p>
    </div>

    <div class="input-section">
      <div style="margin-bottom:1.5rem">
        <label><i class="nf-md-regex"></i> Regular Expression</label>
        <div style="display:flex; gap:1rem">
          <input type="text" bind:value={regexPattern} on:input={testRegex} placeholder="e.g. /^[a-z]+$/" style="flex:1" />
          <input type="text" bind:value={flags} on:input={testRegex} placeholder="flags (gmi)" style="width:100px" />
        </div>
        {#if error}<p style="color:#f38ba8; margin-top:0.5rem; font-size:0.9rem"><i class="nf-md-alert_circle"></i> {error}</p>{/if}
      </div>

      <div>
        <label><i class="nf-md-text"></i> Test String</label>
        <textarea bind:value={testString} on:input={testRegex} placeholder="Paste text here to test against regex..." style="height:150px"></textarea>
      </div>
    </div>

    {#if matches.length > 0}
      <div class="result-section">
        <label><i class="nf-md-check_circle"></i> Found {matches.length} matches</label>
        <div class="matches-list">
          {#each matches as match, i}
            <div class="match-item" style="animation-delay:{i*0.05}s">
              <span class="match-index">#{i+1}</span>
              <span class="match-text">{match.text}</span>
              <span class="match-range">[{match.start}-{match.end}]</span>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
</main>

<style>
  /* Reuse common styles */
  .container { min-height:100vh; padding:3rem 2rem; background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%); }
  .content { max-width:900px; margin:0 auto; }
  .header { text-align:center; margin-bottom:3rem; }
  .icon-wrapper { display:inline-flex; width:80px; height:80px; background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%); border-radius:18px; align-items:center; justify-content:center; margin-bottom:1.5rem; box-shadow:0 8px 24px rgba(203,166,247,0.4); animation:float 3s ease-in-out infinite; font-size:3rem; color:#1e1e2e; }
  h1 { background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%); -webkit-background-clip:text; -webkit-text-fill-color:transparent; font-size:2.5rem; margin-bottom:0.5rem; font-weight:bold; }
  .subtitle { color:#a6adc8; font-size:1.1rem; }
  .input-section, .result-section { background:rgba(24,24,37,0.7); backdrop-filter:blur(10px); padding:2.5rem; border-radius:20px; border:1px solid rgba(203,166,247,0.15); box-shadow:0 8px 32px rgba(0,0,0,0.3); margin-bottom:2rem; animation:slideUp 0.5s ease-out; }
  
  input, textarea { width:100%; padding:1rem; background:#11111b; border:2px solid #313244; border-radius:12px; color:#cdd6f4; font-family:'Consolas',monospace; transition:all 0.3s; }
  input:focus, textarea:focus { border-color:#cba6f7; outline:none; }
  label { color:#cba6f7; font-weight:600; margin-bottom:0.75rem; display:flex; align-items:center; gap:0.5rem; }
  
  .matches-list { display:flex; flex-direction:column; gap:0.75rem; max-height:400px; overflow-y:auto; }
  .match-item { background:#11111b; padding:1rem; border-radius:10px; display:flex; align-items:center; gap:1rem; border-left:4px solid #cba6f7; animation:fadeIn 0.3s ease-out backwards; }
  .match-index { color:#6c7086; font-size:0.9rem; font-weight:bold; }
  .match-text { color:#cdd6f4; font-family:'Consolas',monospace; flex:1; word-break:break-all; }
  .match-range { color:#6c7086; font-size:0.8rem; font-family:'Consolas',monospace; }
  
  @keyframes float { 0%, 100% { transform:translateY(0); } 50% { transform:translateY(-10px); } }
  @keyframes slideUp { from { opacity:0; transform:translateY(20px); } to { opacity:1; transform:translateY(0); } }
  @keyframes fadeIn { from { opacity:0; transform:translateX(-10px); } to { opacity:1; transform:translateX(0); } }
</style>
