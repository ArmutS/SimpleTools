<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  let input = "";
  let output = "";

  function transform(type: string) {
    if (!input) return;
    switch(type) {
      case 'upper': output = input.toUpperCase(); break;
      case 'lower': output = input.toLowerCase(); break;
      case 'title': output = input.replace(/\w\S*/g, (txt) => txt.charAt(0).toUpperCase() + txt.substr(1).toLowerCase()); break;
      case 'camel': output = input.replace(/(?:^\w|[A-Z]|\b\w)/g, (w, i) => i === 0 ? w.toLowerCase() : w.toUpperCase()).replace(/\s+/g, ''); break;
      case 'kebab': output = input.match(/[A-Z]{2,}(?=[A-Z][a-z]+[0-9]*|\b)|[A-Z]?[a-z]+[0-9]*|[A-Z]|[0-9]+/g)?.map(x => x.toLowerCase()).join('-') || ""; break;
      case 'snake': output = input.match(/[A-Z]{2,}(?=[A-Z][a-z]+[0-9]*|\b)|[A-Z]?[a-z]+[0-9]*|[A-Z]|[0-9]+/g)?.map(x => x.toLowerCase()).join('_') || ""; break;
      case 'reverse': output = input.split('').reverse().join(''); break;
    }
  }

  async function escape(mode: string) {
    if (!input) return;
    try {
      output = await invoke("process_string_escape", { currentText: input, mode });
    } catch (e) { alert(e); }
  }
</script>

<main class="container">
  <div class="content">
    <div class="header">
      <div class="icon-wrapper"><i class="nf-md-format_text"></i></div>
      <h1>String Tools</h1>
      <p class="subtitle">Convert, escape, and manipulate text strings</p>
    </div>

    <div class="input-section">
      <textarea bind:value={input} placeholder="Enter text to transform..." style="height:150px; margin-bottom:1.5rem"></textarea>
      
      <div class="controls-grid">
        <div class="group">
          <label>Case Converters</label>
          <div class="btn-group">
            <button on:click={() => transform('upper')}>UPPERCASE</button>
            <button on:click={() => transform('lower')}>lowercase</button>
            <button on:click={() => transform('title')}>Title Case</button>
            <button on:click={() => transform('camel')}>camelCase</button>
            <button on:click={() => transform('snake')}>snake_case</button>
            <button on:click={() => transform('kebab')}>kebab-case</button>
          </div>
        </div>
        <div class="group">
          <label>Escaping & Utils</label>
          <div class="btn-group">
            <button on:click={() => escape('escape')}>JSON Escape</button>
            <button on:click={() => escape('unescape')}>JSON Unescape</button>
            <button on:click={() => transform('reverse')}>Reverse</button>
          </div>
        </div>
      </div>
    </div>

    {#if output}
      <div class="result-section">
        <label>Result</label>
        <textarea readonly value={output} style="height:150px; background:#181825; border-color:#cba6f7"></textarea>
        <button class="btn-copy" on:click={() => navigator.clipboard.writeText(output)} style="margin-top:1rem; width:100%">Copy Result</button>
      </div>
    {/if}
  </div>
</main>

<style>
  /* Common styles reused */
  .container { min-height:100vh; padding:3rem 2rem; background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%); }
  .content { max-width:900px; margin:0 auto; }
  .header { text-align:center; margin-bottom:3rem; }
  .icon-wrapper { display:inline-flex; width:80px; height:80px; background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%); border-radius:18px; align-items:center; justify-content:center; margin-bottom:1.5rem; box-shadow:0 8px 24px rgba(203,166,247,0.4); animation:float 3s ease-in-out infinite; font-size:3rem; color:#1e1e2e; }
  h1 { background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%); -webkit-background-clip:text; -webkit-text-fill-color:transparent; font-size:2.5rem; margin-bottom:0.5rem; font-weight:bold; }
  .subtitle { color:#a6adc8; font-size:1.1rem; }
  .input-section, .result-section { background:rgba(24,24,37,0.7); backdrop-filter:blur(10px); padding:2.5rem; border-radius:20px; border:1px solid rgba(203,166,247,0.15); box-shadow:0 8px 32px rgba(0,0,0,0.3); margin-bottom:2rem; animation:slideUp 0.5s ease-out; }
  
  textarea { width:100%; padding:1rem; background:#11111b; border:2px solid #313244; border-radius:12px; color:#cdd6f4; font-family:'Consolas',monospace; transition:all 0.3s; }
  textarea:focus { border-color:#cba6f7; outline:none; }
  label { color:#cba6f7; font-weight:600; margin-bottom:0.75rem; display:block; }
  
  .controls-grid { display:grid; grid-template-columns:1fr 1fr; gap:2rem; }
  .btn-group { display:grid; grid-template-columns:1fr 1fr; gap:0.75rem; }
  button { padding:0.75rem; background:#313244; color:#cdd6f4; border:none; border-radius:8px; cursor:pointer; font-size:0.9rem; transition:0.2s; }
  button:hover { background:#cba6f7; color:#1e1e2e; }
  .btn-copy { background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%); color:#1e1e2e; font-weight:bold; }
  
  @keyframes float { 0%, 100% { transform:translateY(0); } 50% { transform:translateY(-10px); } }
  @keyframes slideUp { from { opacity:0; transform:translateY(20px); } to { opacity:1; transform:translateY(0); } }
</style>
