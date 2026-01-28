<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  let directory = "";
  let duplicates: any[] = [];
  let isScanning = false;
  async function selectDir() {
    const selected = await open({ directory: true });
    if (selected) { directory = selected as string; findDuplicates(); }
  }
  async function findDuplicates() {
    isScanning = true;
    try {
      duplicates = await invoke("find_duplicate_files", { directory });
    } catch (e) { alert(`Error: ${e}`); }
    finally { isScanning = false; }
  }
  function formatBytes(b: number) {
    if (b === 0) return "0 B";
    const k = 1024, s = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(b) / Math.log(k));
    return (b / Math.pow(k, i)).toFixed(2) + " " + s[i];
  }
</script>
<main style="min-height:100vh;padding:2rem;background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%)">
  <div style="max-width:1100px;margin:0 auto">
    <div style="text-align:center;margin-bottom:2rem">
      <div style="display:inline-flex;width:80px;height:80px;background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%);border-radius:18px;align-items:center;justify-content:center;margin-bottom:1.5rem;box-shadow:0 8px 24px rgba(243,139,168,0.4);animation:float 3s ease-in-out infinite">
        <i class="nf-md-content_copy" style="font-size:3rem;color:#1e1e2e"></i>
      </div>
      <h1 style="background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%);-webkit-background-clip:text;-webkit-text-fill-color:transparent;font-size:2.5rem;margin-bottom:0.5rem;font-weight:bold">Duplicate Finder</h1>
      <p style="color:#a6adc8;font-size:1.1rem">Find duplicate files by content hash</p>
    </div>
    <div style="background:rgba(24,24,37,0.7);backdrop-filter:blur(10px);padding:2.5rem;border-radius:20px;border:1px solid rgba(243,139,168,0.15);box-shadow:0 8px 32px rgba(0,0,0,0.3);margin-bottom:2rem">
      <button on:click={selectDir} disabled={isScanning} style="width:100%;padding:1.5rem;background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%);color:#1e1e2e;border:none;border-radius:14px;cursor:pointer;font-size:1.1rem;font-weight:600;display:flex;align-items:center;justify-content:center;gap:0.75rem;transition:all 0.3s;box-shadow:0 4px 16px rgba(243,139,168,0.3)">
        <i class="nf-md-folder_search" style="font-size:1.5rem"></i><span>{isScanning ? "Scanning..." : "Scan Directory"}</span>
      </button>
      {#if directory}<p style="color:#cdd6f4;margin-top:1rem;font-size:0.95rem">{directory}</p>{/if}
    </div>
    {#if duplicates.length > 0}
      <div style="background:rgba(24,24,37,0.7);backdrop-filter:blur(10px);padding:2.5rem;border-radius:20px;border:1px solid rgba(243,139,168,0.15);box-shadow:0 8px 32px rgba(0,0,0,0.3);animation:slideUp 0.5s ease-out">
        <h2 style="color:#f38ba8;margin-bottom:2rem;display:flex;align-items:center;gap:0.75rem;font-size:1.5rem"><i class="nf-md-file_multiple"></i>Found {duplicates.length} duplicate groups</h2>
        <div style="max-height:550px;overflow:auto">
          {#each duplicates as group, i}
            <div style="background:#11111b;padding:1.75rem;border-radius:14px;margin-bottom:1.25rem;border:2px solid #313244;animation:fadeIn 0.4s ease-out {i * 60}ms backwards">
              <p style="color:#f38ba8;font-weight:bold;margin-bottom:1rem;font-size:1.05rem">Group {i + 1} - {formatBytes(group.file_size)} each</p>
              {#each group.files as file}
                <p style="color:#a6adc8;font-size:0.9rem;padding:0.75rem;background:rgba(17,17,27,0.6);border-radius:8px;margin-bottom:0.5rem;font-family:Consolas,monospace">{file}</p>
              {/each}
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
  button:hover:not(:disabled) { transform: translateY(-3px); box-shadow: 0 8px 24px rgba(243, 139, 168, 0.5); }
</style>
