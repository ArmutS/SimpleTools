<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  let left = "";
  let right = "";
  let diffs: any[] = [];
  let mode = 1; // 0=char, 1=word, 2=line

  async function compare() {
    if (!left && !right) return;
    try {
      diffs = await invoke("process_text_diff", { leftIn: left, rightIn: right, select: mode });
    } catch (e) { alert(e); }
  }
</script>

<main class="container">
  <div class="content" style="max-width:1200px">
    <div class="header">
      <div class="icon-wrapper">
        <i class="nf-oct-diff"></i>
      </div>
      <h1>Text Diff Viewer</h1>
      <p class="subtitle">Compare two texts and find differences</p>
    </div>

    <div class="input-section" style="padding:1.5rem">
      <div class="controls">
        <button class:active={mode===0} on:click={()=>{mode=0;compare()}}>Chars</button>
        <button class:active={mode===1} on:click={()=>{mode=1;compare()}}>Words</button>
        <button class:active={mode===2} on:click={()=>{mode=2;compare()}}>Lines</button>
      </div>
      
      <div class="grid">
        <div class="pane">
          <label><i class="nf-md-text_box_outline"></i> Original Text</label>
          <textarea bind:value={left} on:input={compare} placeholder="Paste original text here..."></textarea>
        </div>
        <div class="pane">
          <label><i class="nf-md-text_box_check_outline"></i> Modified Text</label>
          <textarea bind:value={right} on:input={compare} placeholder="Paste modified text here..."></textarea>
        </div>
      </div>
    </div>

    {#if diffs.length > 0}
      <div class="result-section">
        <label><i class="nf-md-compare"></i> Comparison Result</label>
        <div class="diff-output">
          {#each diffs as part}
            <span class={part.tag}>{part.text}</span>
          {/each}
        </div>
      </div>
    {/if}
  </div>
</main>

<style>
  .container { min-height:100vh; padding:3rem 2rem; background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%); }
  .content { margin:0 auto; }
  .header { text-align:center; margin-bottom:3rem; }
  .icon-wrapper { display:inline-flex; width:80px; height:80px; background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%); border-radius:18px; align-items:center; justify-content:center; margin-bottom:1.5rem; box-shadow:0 8px 24px rgba(203,166,247,0.4); animation:float 3s ease-in-out infinite; }
  .icon-wrapper i { font-size:3rem; color:#1e1e2e; }
  h1 { background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%); -webkit-background-clip:text; -webkit-text-fill-color:transparent; font-size:2.5rem; margin-bottom:0.5rem; }
  .subtitle { color:#a6adc8; font-size:1.1rem; }
  .input-section, .result-section { background:rgba(24,24,37,0.7); backdrop-filter:blur(10px); padding:2rem; border-radius:20px; border:1px solid rgba(203,166,247,0.15); box-shadow:0 8px 32px rgba(0,0,0,0.3); margin-bottom:2rem; animation:slideUp 0.5s ease-out; }
  
  .controls { display:flex; gap:1rem; margin-bottom:1.5rem; justify-content:center; }
  .controls button { padding:0.75rem 1.5rem; background:#11111b; border:2px solid #313244; color:#cdd6f4; border-radius:10px; cursor:pointer; font-weight:600; transition:all 0.3s; }
  .controls button.active { border-color:#cba6f7; background:rgba(203,166,247,0.1); color:#cba6f7; }
  
  .grid { display:grid; grid-template-columns:1fr 1fr; gap:1.5rem; }
  .pane { display:flex; flex-direction:column; gap:0.5rem; }
  textarea { width:100%; height:300px; background:#11111b; border:2px solid #313244; color:#cdd6f4; border-radius:12px; padding:1rem; font-family:'Consolas',monospace; resize:none; transition:all 0.3s; }
  textarea:focus { border-color:#cba6f7; outline:none; }
  
  .diff-output { background:#11111b; padding:1.5rem; border-radius:12px; font-family:'Consolas',monospace; line-height:1.6; white-space:pre-wrap; border:2px solid #313244; min-height:100px; }
  .diff-output span.left { background:rgba(243,139,168,0.2); color:#f38ba8; text-decoration:line-through; }
  .diff-output span.right { background:rgba(166,227,161,0.2); color:#a6e3a1; }
  .diff-output span.not { color:#cdd6f4; }
  
  @keyframes float { 0%, 100% { transform:translateY(0); } 50% { transform:translateY(-10px); } }
  @keyframes slideUp { from { opacity:0; transform:translateY(20px); } to { opacity:1; transform:translateY(0); } }
  label { color:#cba6f7; font-weight:600; display:flex; align-items:center; gap:0.5rem; margin-bottom:0.5rem; }
</style>
