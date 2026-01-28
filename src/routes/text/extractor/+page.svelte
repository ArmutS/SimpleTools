<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  let input = "";
  let results: string[] = [];
  let options = {
    email: true,
    url: true,
    ip: false,
    hashtag: false,
    log_error: false
  };

  async function extract() {
    if (!input) return;
    try {
      results = await invoke("process_extractor", { currentText: input, options });
    } catch (e) { alert(e); }
  }
</script>

<main class="container">
  <div class="content">
    <div class="header">
      <div class="icon-wrapper"><i class="nf-fa-filter"></i></div>
      <h1>Text Extractor</h1>
      <p class="subtitle">Extract specific data types from unstructured text</p>
    </div>

    <div class="input-section">
      <div style="margin-bottom:1.5rem">
        <label><i class="nf-md-text"></i> Source Text</label>
        <textarea bind:value={input} on:input={extract} placeholder="Paste text here containing emails, urls, etc..." style="height:200px"></textarea>
      </div>

      <div style="display:flex; gap:1.5rem; flex-wrap:wrap">
        <label class="checkbox"><input type="checkbox" bind:checked={options.email} on:change={extract}> Emails</label>
        <label class="checkbox"><input type="checkbox" bind:checked={options.url} on:change={extract}> URLs</label>
        <label class="checkbox"><input type="checkbox" bind:checked={options.ip} on:change={extract}> IP Addresses</label>
        <label class="checkbox"><input type="checkbox" bind:checked={options.hashtag} on:change={extract}> Hashtags</label>
        <label class="checkbox"><input type="checkbox" bind:checked={options.log_error} on:change={extract}> Log Errors</label>
      </div>
    </div>

    {#if results.length > 0}
      <div class="result-section">
        <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:1rem">
          <label><i class="nf-md-check_all"></i> Extracted Items ({results.length})</label>
          <button class="btn-copy" on:click={() => navigator.clipboard.writeText(results.join('\n'))}>Copy All</button>
        </div>
        <div class="list">
          {#each results as items, i}
             <div class="item" style="animation-delay:{i*0.03}s">{items}</div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
</main>

<style>
  /* Reuse styles */
  .container { min-height:100vh; padding:3rem 2rem; background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%); }
  .content { max-width:900px; margin:0 auto; }
  .header { text-align:center; margin-bottom:3rem; }
  .icon-wrapper { display:inline-flex; width:80px; height:80px; background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%); border-radius:18px; align-items:center; justify-content:center; margin-bottom:1.5rem; box-shadow:0 8px 24px rgba(203,166,247,0.4); animation:float 3s ease-in-out infinite; font-size:3rem; color:#1e1e2e; }
  h1 { background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%); -webkit-background-clip:text; -webkit-text-fill-color:transparent; font-size:2.5rem; margin-bottom:0.5rem; font-weight:bold; }
  .subtitle { color:#a6adc8; font-size:1.1rem; }
  .input-section, .result-section { background:rgba(24,24,37,0.7); backdrop-filter:blur(10px); padding:2.5rem; border-radius:20px; border:1px solid rgba(203,166,247,0.15); box-shadow:0 8px 32px rgba(0,0,0,0.3); margin-bottom:2rem; animation:slideUp 0.5s ease-out; }
  
  textarea { width:100%; padding:1rem; background:#11111b; border:2px solid #313244; border-radius:12px; color:#cdd6f4; font-family:'Consolas',monospace; transition:all 0.3s; }
  textarea:focus { border-color:#cba6f7; outline:none; }
  
  label { color:#cba6f7; font-weight:600; margin-bottom:0.75rem; display:flex; align-items:center; gap:0.5rem; }
  .checkbox { background:#11111b; padding:0.75rem 1.25rem; border-radius:10px; border:2px solid #313244; cursor:pointer; color:#cdd6f4; display:flex; align-items:center; gap:0.5rem; transition:all 0.2s; }
  .checkbox:hover { border-color:#cba6f7; }
  .checkbox input { width:auto; margin:0; }
  
  .list { display:flex; flex-direction:column; gap:0.5rem; max-height:400px; overflow-y:auto; }
  .item { background:#11111b; padding:1rem; border-radius:8px; border-left:3px solid #cba6f7; font-family:'Consolas',monospace; animation:fadeIn 0.3s ease-out backwards; color:#cdd6f4; }
  
  .btn-copy { background:#313244; color:#cdd6f4; border:none; padding:0.5rem 1rem; border-radius:8px; cursor:pointer; font-weight:600; }
  .btn-copy:hover { background:#45475a; }
  
  @keyframes float { 0%, 100% { transform:translateY(0); } 50% { transform:translateY(-10px); } }
  @keyframes slideUp { from { opacity:0; transform:translateY(20px); } to { opacity:1; transform:translateY(0); } }
  @keyframes fadeIn { from { opacity:0; transform:translateX(-10px); } to { opacity:1; transform:translateX(0); } }
</style>
