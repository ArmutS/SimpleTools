<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  let filePath = "";
  let metadata: any = null;
  async function selectFile() {
    const selected = await open({ multiple: false });
    if (selected) { filePath = selected as string; getMetadata(); }
  }
  async function getMetadata() {
    try {
      metadata = await invoke("get_file_metadata", { filePath });
    } catch (e) { alert(`Error: ${e}`); }
  }
</script>
<main style="min-height:100vh;padding:2rem;background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%)">
  <div style="max-width:900px;margin:0 auto">
    <div style="text-align:center;margin-bottom:2rem">
      <div style="display:inline-flex;width:80px;height:80px;background:linear-gradient(135deg, #89b4fa 0%, #74c7ec 100%);border-radius:18px;align-items:center;justify-content:center;margin-bottom:1.5rem;box-shadow:0 8px 24px rgba(137,180,250,0.4);animation:float 3s ease-in-out infinite">
        <i class="nf-md-information_variant" style="font-size:3rem;color:#1e1e2e"></i>
      </div>
      <h1 style="background:linear-gradient(135deg, #89b4fa 0%, #74c7ec 100%);-webkit-background-clip:text;-webkit-text-fill-color:transparent;font-size:2.5rem;margin-bottom:0.5rem;font-weight:bold">Metadata Viewer</h1>
      <p style="color:#a6adc8;font-size:1.1rem">View detailed file information (EXIF, ID3)</p>
    </div>
    <div style="background:rgba(24,24,37,0.7);backdrop-filter:blur(10px);padding:2.5rem;border-radius:20px;border:1px solid rgba(137,180,250,0.15);box-shadow:0 8px 32px rgba(0,0,0,0.3)">
      <button on:click={selectFile} style="width:100%;padding:1.5rem;background:linear-gradient(135deg, #89b4fa 0%, #74c7ec 100%);color:#1e1e2e;border:none;border-radius:14px;cursor:pointer;font-weight:600;font-size:1.1rem;box-shadow:0 4px 16px rgba(137,180,250,0.3)"><i class="nf-md-file_search"></i> Select Image or Audio File</button>
      {#if filePath}<p style="color:#cdd6f4;margin-top:1.5rem;font-size:0.95rem;font-family:Consolas,monospace;text-align:center;background:#11111b;padding:1rem;border-radius:10px">{filePath}</p>{/if}
    </div>
    {#if metadata}
      <div style="background:rgba(24,24,37,0.7);backdrop-filter:blur(10px);padding:2.5rem;border-radius:20px;margin-top:2rem;border:1px solid rgba(137,180,250,0.15);animation:slideUp 0.5s ease-out;box-shadow:0 8px 32px rgba(0,0,0,0.3)">
        <h2 style="color:#89b4fa;margin-bottom:1.5rem;font-size:1.5rem;border-bottom:2px solid #313244;padding-bottom:0.5rem">File Properties</h2>
        <div style="display:grid;grid-template-columns:1fr 1fr;gap:2rem">
          <div>
            {#each Object.entries(metadata).slice(0, Math.ceil(Object.keys(metadata).length / 2)) as [key, value]}
              <div style="margin-bottom:1rem;background:#11111b;padding:0.75rem;border-radius:8px">
                <span style="color:#a6adc8;font-weight:600;display:block;font-size:0.9rem;margin-bottom:0.25rem">{key.replace(/_/g, ' ').toUpperCase()}</span>
                <span style="color:#cdd6f4;font-family:Consolas,monospace;display:block;word-break:break-all">{value}</span>
              </div>
            {/each}
          </div>
          <div>
            {#each Object.entries(metadata).slice(Math.ceil(Object.keys(metadata).length / 2)) as [key, value]}
              <div style="margin-bottom:1rem;background:#11111b;padding:0.75rem;border-radius:8px">
                <span style="color:#a6adc8;font-weight:600;display:block;font-size:0.9rem;margin-bottom:0.25rem">{key.replace(/_/g, ' ').toUpperCase()}</span>
                <span style="color:#cdd6f4;font-family:Consolas,monospace;display:block;word-break:break-all">{value}</span>
              </div>
            {/each}
          </div>
        </div>
      </div>
    {/if}
  </div>
</main>
<style>
  @keyframes float { 0%, 100% { transform: translateY(0); } 50% { transform: translateY(-10px); } }
  @keyframes slideUp { from { opacity: 0; transform: translateY(20px); } to { opacity: 1; transform: translateY(0); } }
  button:hover { transform: translateY(-3px); box-shadow: 0 8px 24px rgba(137, 180, 250, 0.5); }
</style>
