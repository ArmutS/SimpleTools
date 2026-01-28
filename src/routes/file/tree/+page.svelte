<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  let directory = "";
  let tree = "";
  let maxDepth = 3;
  async function selectDir() {
    const selected = await open({ directory: true });
    if (selected) { directory = selected as string; generateTree(); }
  }
  async function generateTree() {
    try {
      tree = await invoke("generate_directory_tree", { directory, maxDepth, filterExtension: null });
    } catch (e) { alert(`Error: ${e}`); }
  }
</script>
<main style="min-height:100vh;padding:2rem;background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%)">
  <div style="max-width:1100px;margin:0 auto">
    <div style="text-align:center;margin-bottom:2rem">
      <div style="display:inline-flex;width:80px;height:80px;background:linear-gradient(135deg, #fab387 0%, #f9e2af 100%);border-radius:18px;align-items:center;justify-content:center;margin-bottom:1.5rem;box-shadow:0 8px 24px rgba(250,179,135,0.4);animation:float 3s ease-in-out infinite">
        <i class="nf-md-file_tree" style="font-size:3rem;color:#1e1e2e"></i>
      </div>
      <h1 style="background:linear-gradient(135deg, #fab387 0%, #f9e2af 100%);-webkit-background-clip:text;-webkit-text-fill-color:transparent;font-size:2.5rem;margin-bottom:0.5rem;font-weight:bold">Directory Tree</h1>
      <p style="color:#a6adc8;font-size:1.1rem">Visualize directory structure</p>
    </div>
    <div style="background:rgba(24,24,37,0.7);backdrop-filter:blur(10px);padding:2.5rem;border-radius:20px;border:1px solid rgba(250,179,135,0.15);box-shadow:0 8px 32px rgba(0,0,0,0.3);display:grid;grid-template-columns:1fr 200px;gap:1.5rem;align-items:end">
      <div>
        <button on:click={selectDir} style="width:100%;padding:1.5rem;background:linear-gradient(135deg, #fab387 0%, #f9e2af 100%);color:#1e1e2e;border:none;border-radius:14px;cursor:pointer;font-weight:600;font-size:1.1rem;box-shadow:0 4px 16px rgba(250,179,135,0.3)"><i class="nf-md-folder_open"></i> Select Directory</button>
      </div>
      <div>
        <label style="color:#fab387;margin-bottom:0.75rem;display:block;font-weight:600">Max Depth</label>
        <input type="number" bind:value={maxDepth} min="1" max="10" style="width:100%;padding:1.25rem;background:#11111b;border:2px solid #313244;border-radius:12px;color:#cdd6f4;font-size:1.1rem;text-align:center"/>
      </div>
    </div>
    {#if directory}<p style="color:#cdd6f4;margin:1.5rem 0;font-family:Consolas,monospace;background:rgba(24,24,37,0.7);padding:1rem;border-radius:12px;text-align:center">{directory}</p>{/if}
    
    {#if tree}
      <div style="background:rgba(24,24,37,0.7);backdrop-filter:blur(10px);padding:2rem;border-radius:20px;margin-top:1.5rem;border:1px solid rgba(250,179,135,0.15);animation:slideUp 0.5s ease-out;box-shadow:0 8px 32px rgba(0,0,0,0.3)">
        <pre style="color:#cdd6f4;background:#11111b;padding:2rem;border-radius:14px;overflow:auto;max-height:600px;font-size:0.95rem;line-height:1.5;font-family:'FiraCode Nerd Font Mono', monospace;border:2px solid #313244">{tree}</pre>
        <button on:click={() => navigator.clipboard.writeText(tree)} style="margin-top:1.5rem;padding:1rem 2rem;background:#313244;color:#cdd6f4;border:none;border-radius:10px;cursor:pointer;font-weight:600;display:flex;align-items:center;gap:0.5rem"><i class="nf-md-content_copy"></i> Copy Tree</button>
      </div>
    {/if}
  </div>
</main>
<style>
  @keyframes float { 0%, 100% { transform: translateY(0); } 50% { transform: translateY(-10px); } }
  @keyframes slideUp { from { opacity: 0; transform: translateY(20px); } to { opacity: 1; transform: translateY(0); } }
  button:hover { transform: translateY(-3px); }
  input:focus { border-color: #fab387; outline: none; }
</style>
