<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  let result: any = null;
  let isCleaning = false;
  async function clean() {
    isCleaning = true;
    try {
      result = await invoke("clean_temp_files");
    } catch (e) { alert(`Error: ${e}`); }
    finally { isCleaning = false; }
  }
  function formatBytes(b: number) {
    if (b === 0) return "0 B";
    const k = 1024, s = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(b) / Math.log(k));
    return (b / Math.pow(k, i)).toFixed(2) + " " + s[i];
  }
</script>
<main style="min-height:100vh;padding:2rem;background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%)">
  <div style="max-width:900px;margin:0 auto">
    <div style="text-align:center;margin-bottom:2rem">
      <div style="display:inline-flex;width:80px;height:80px;background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%);border-radius:18px;align-items:center;justify-content:center;margin-bottom:1.5rem;box-shadow:0 8px 24px rgba(243,139,168,0.4);animation:float 3s ease-in-out infinite">
        <i class="nf-md-broom" style="font-size:3rem;color:#1e1e2e"></i>
      </div>
      <h1 style="background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%);-webkit-background-clip:text;-webkit-text-fill-color:transparent;font-size:2.5rem;margin-bottom:0.5rem;font-weight:bold">System Cleaner</h1>
      <p style="color:#a6adc8;font-size:1.1rem">Clean temporary files and free up space</p>
    </div>
    
    <div style="background:rgba(24,24,37,0.7);backdrop-filter:blur(10px);padding:3rem;border-radius:20px;border:1px solid rgba(243,139,168,0.15);box-shadow:0 8px 32px rgba(0,0,0,0.3);text-align:center">
      <p style="color:#cdd6f4;margin-bottom:2rem;font-size:1.1rem;line-height:1.6">
        This tool will safely remove temporary files from your system's temp directories.<br>
        <span style="color:#f38ba8;font-size:0.9rem"><i class="nf-md-alert_circle"></i> Action cannot be undone.</span>
      </p>
      
      <button on:click={clean} disabled={isCleaning} style="padding:1.5rem 3rem;background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%);color:#1e1e2e;border:none;border-radius:14px;cursor:{isCleaning?'not-allowed':'pointer'};font-weight:bold;font-size:1.4rem;box-shadow:0 6px 20px rgba(243,139,168,0.4);transition:all 0.3s;display:inline-flex;align-items:center;gap:1rem">
        <i class="nf-md-delete_sweep" style="font-size:1.8rem"></i> {isCleaning ? 'Cleaning...' : 'Clean Now'}
      </button>

      {#if result}
        <div style="margin-top:2.5rem;background:#11111b;padding:2rem;border-radius:16px;border:2px solid #a6e3a1;animation:slideUp 0.5s ease-out">
          <h2 style="color:#a6e3a1;margin-bottom:1rem;font-size:1.8rem"><i class="nf-md-check_circle"></i> Cleanup Complete!</h2>
          <div style="display:grid;grid-template-columns:1fr 1fr;gap:1.5rem;margin-top:1.5rem">
            <div style="background:rgba(166,227,161,0.1);padding:1.5rem;border-radius:12px">
              <p style="color:#a6adc8;font-size:1rem;margin-bottom:0.5rem">Space Freed</p>
              <p style="color:#a6e3a1;font-size:2rem;font-weight:bold">{formatBytes(result.freed_space)}</p>
            </div>
            <div style="background:rgba(166,227,161,0.1);padding:1.5rem;border-radius:12px">
              <p style="color:#a6adc8;font-size:1rem;margin-bottom:0.5rem">Files Removed</p>
              <p style="color:#a6e3a1;font-size:2rem;font-weight:bold">{result.files_removed}</p>
            </div>
          </div>
          {#each result.errors as error}
            <p style="color:#f38ba8;margin-top:0.5rem;font-size:0.9rem;text-align:left"><i class="nf-md-alert"></i> {error}</p>
          {/each}
        </div>
      {/if}
    </div>
  </div>
</main>
<style>
  @keyframes float { 0%, 100% { transform: translateY(0); } 50% { transform: translateY(-10px); } }
  @keyframes slideUp { from { opacity: 0; transform: translateY(20px); } to { opacity: 1; transform: translateY(0); } }
  button:hover:not(:disabled) { transform: translateY(-4px); box-shadow: 0 10px 25px rgba(243, 139, 168, 0.6); }
</style>
