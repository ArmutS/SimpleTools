<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  let directory = "";
  let entries: any[] = [];
  let isAnalyzing = false;
  async function selectDir() {
    const selected = await open({ directory: true });
    if (selected) { directory = selected as string; analyze(); }
  }
  async function analyze() {
    isAnalyzing = true;
    try {
      entries = await invoke("analyze_disk_usage", { directory });
      entries.sort((a, b) => b.size - a.size);
    } catch (e) { alert(`Error: ${e}`); }
    finally { isAnalyzing = false; }
  }
  function formatBytes(b: number) {
    if (b === 0) return "0 B";
    const k = 1024, s = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(b) / Math.log(k));
    return (b / Math.pow(k, i)).toFixed(2) + " " + s[i];
  }
  $: maxSize = entries.length > 0 ? entries[0].size : 1;
</script>
<main style="min-height:100vh;padding:2rem;background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%)">
  <div style="max-width:1100px;margin:0 auto">
    <div style="text-align:center;margin-bottom:2rem">
      <div style="display:inline-flex;width:80px;height:80px;background:linear-gradient(135deg, #a6e3a1 0%, #94e2d5 100%);border-radius:18px;align-items:center;justify-content:center;margin-bottom:1.5rem;box-shadow:0 8px 24px rgba(166,227,161,0.4);animation:float 3s ease-in-out infinite">
        <i class="nf-md-chart_pie" style="font-size:3rem;color:#1e1e2e"></i>
      </div>
      <h1 style="background:linear-gradient(135deg, #a6e3a1 0%, #94e2d5 100%);-webkit-background-clip:text;-webkit-text-fill-color:transparent;font-size:2.5rem;margin-bottom:0.5rem;font-weight:bold">Disk Usage Analyzer</h1>
      <p style="color:#a6adc8;font-size:1.1rem">Analyze directory sizes and find space hogs</p>
    </div>
    <div style="background:rgba(24,24,37,0.7);backdrop-filter:blur(10px);padding:2.5rem;border-radius:20px;border:1px solid rgba(166,227,161,0.15);box-shadow:0 8px 32px rgba(0,0,0,0.3);margin-bottom:2rem">
      <button on:click={selectDir} disabled={isAnalyzing} style="width:100%;padding:1.5rem;background:linear-gradient(135deg, #a6e3a1 0%, #94e2d5 100%);color:#1e1e2e;border:none;border-radius:14px;cursor:pointer;font-size:1.1rem;font-weight:600;display:flex;align-items:center;justify-content:center;gap:0.75rem;transition:all 0.3s;box-shadow:0 4px 16px rgba(166,227,161,0.3)">
        <i class="nf-md-chart_bar" style="font-size:1.5rem"></i><span>{isAnalyzing ? "Analyzing..." : "Analyze Directory"}</span>
      </button>
      {#if directory}<p style="color:#cdd6f4;margin-top:1rem;font-size:0.95rem">{directory}</p>{/if}
    </div>
    {#if entries.length > 0}
      <div style="background:rgba(24,24,37,0.7);backdrop-filter:blur(10px);padding:2.5rem;border-radius:20px;border:1px solid rgba(166,227,161,0.15);box-shadow:0 8px 32px rgba(0,0,0,0.3);animation:slideUp 0.5s ease-out">
        <h2 style="color:#a6e3a1;margin-bottom:2rem;display:flex;align-items:center;gap:0.75rem;font-size:1.5rem"><i class="nf-md-folder_open"></i>{entries.length} items analyzed</h2>
        <div style="max-height:550px;overflow:auto">
          {#each entries as entry, i}
            <div style="margin-bottom:1.75rem;animation:fadeIn 0.4s ease-out {i * 40}ms backwards">
              <div style="display:flex;justify-content:space-between;margin-bottom:0.75rem">
                <span style="color:#cdd6f4;font-weight:600;font-size:0.95rem">{entry.name}</span>
                <span style="color:#a6e3a1;font-weight:bold;font-size:1rem">{formatBytes(entry.size)}</span>
              </div>
              <div style="background:#11111b;height:14px;border-radius:8px;overflow:hidden;box-shadow:inset 0 2px 4px rgba(0,0,0,0.3)">
                <div style="background:linear-gradient(90deg, #a6e3a1 0%, #94e2d5 100%);height:100%;width:{(entry.size / maxSize) * 100}%;transition:width 0.6s ease;border-radius:8px"></div>
              </div>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
</main>
<style>
  @keyframes float { 0%, 100% { transform: translateY(0px); } 50% { transform: translateY(-10px); } }
  @keyframes slideUp { from { opacity: 0; transform: translateY(20px); } to { opacity: 1; transform: translateY(0); } }
  @keyframes fadeIn { from { opacity: 0; } to { opacity: 1; } }
  button:hover:not(:disabled) { transform: translateY(-3px); box-shadow: 0 8px 24px rgba(166, 227, 161, 0.5); }
</style>
