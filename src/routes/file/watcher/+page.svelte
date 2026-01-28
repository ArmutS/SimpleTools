<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  let directory = "";
  let isWatching = false;
  let events: any[] = [];
  async function selectDir() {
    const selected = await open({ directory: true });
    if (selected) directory = selected as string;
  }
  async function toggleWatch() {
    try {
      if (isWatching) {
        await invoke("stop_file_watcher");
        isWatching = false;
      } else {
        await invoke("start_file_watcher", { directory });
        isWatching = true;
      }
    } catch (e) { alert(`Error: ${e}`); }
  }
  onMount(() => {
    listen("file-event", (event: any) => {
      events = [event.payload, ...events].slice(0, 50);
    });
  });
</script>
<main style="min-height:100vh;padding:2rem;background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%)">
  <div style="max-width:1100px;margin:0 auto">
    <div style="text-align:center;margin-bottom:2rem">
      <div style="display:inline-flex;width:80px;height:80px;background:linear-gradient(135deg, #f5c2e7 0%, #eba0ac 100%);border-radius:18px;align-items:center;justify-content:center;margin-bottom:1.5rem;box-shadow:0 8px 24px rgba(245,194,231,0.4);animation:float 3s ease-in-out infinite">
        <i class="nf-md-eye" style="font-size:3rem;color:#1e1e2e"></i>
      </div>
      <h1 style="background:linear-gradient(135deg, #f5c2e7 0%, #eba0ac 100%);-webkit-background-clip:text;-webkit-text-fill-color:transparent;font-size:2.5rem;margin-bottom:0.5rem;font-weight:bold">File Watcher</h1>
      <p style="color:#a6adc8;font-size:1.1rem">Monitor directory changes in real-time</p>
    </div>
    <div style="background:rgba(24,24,37,0.7);backdrop-filter:blur(10px);padding:2.5rem;border-radius:20px;border:1px solid rgba(245,194,231,0.15);box-shadow:0 8px 32px rgba(0,0,0,0.3);margin-bottom:2rem">
      <div style="display:grid;grid-template-columns:1fr 200px;gap:1.5rem">
        <button on:click={selectDir} disabled={isWatching} style="width:100%;padding:1.5rem;background:rgba(245,194,231,0.1);color:#f5c2e7;border:2px dashed #f5c2e7;border-radius:14px;cursor:{isWatching?'not-allowed':'pointer'};font-weight:600;font-size:1.1rem;transition:all 0.3s">
          {directory ? directory : 'Select Directory to Watch'}
        </button>
        <button on:click={toggleWatch} disabled={!directory} style="width:100%;padding:1.5rem;background:{isWatching?'#f38ba8':'linear-gradient(135deg, #a6e3a1 0%, #94e2d5 100%)'};color:#1e1e2e;border:none;border-radius:14px;cursor:{!directory?'not-allowed':'pointer'};font-weight:600;font-size:1.1rem;box-shadow:0 4px 16px rgba(0,0,0,0.2)">
          {isWatching ? 'Stop' : 'Start'}
        </button>
      </div>
    </div>
    <div style="background:rgba(24,24,37,0.7);backdrop-filter:blur(10px);padding:2.5rem;border-radius:20px;border:1px solid rgba(245,194,231,0.15);box-shadow:0 8px 32px rgba(0,0,0,0.3);min-height:400px">
      <h2 style="color:#f5c2e7;margin-bottom:1.5rem;display:flex;align-items:center;gap:0.75rem;font-size:1.5rem"><i class="nf-md-history"></i> Event Log</h2>
      <div style="max-height:500px;overflow:auto;display:flex;flex-direction:column;gap:0.75rem">
        {#each events as event, i}
          <div style="background:#11111b;padding:1rem;border-radius:12px;border-left:4px solid #f5c2e7;display:flex;justify-content:space-between;align-items:center;animation:fadeIn 0.3s ease-out">
            <span style="color:#cdd6f4;font-family:Consolas,monospace">{event.path || event}</span>
            <span style="color:#f5c2e7;font-weight:bold;font-size:0.9rem;background:rgba(245,194,231,0.1);padding:0.25rem 0.5rem;border-radius:4px">{event.kind || 'Change'}</span>
          </div>
        {/each}
        {#if events.length === 0}
          <div style="text-align:center;color:#6c7086;padding:3rem">Waiting for events...</div>
        {/if}
      </div>
    </div>
  </div>
</main>
<style>
  @keyframes float { 0%, 100% { transform: translateY(0); } 50% { transform: translateY(-10px); } }
  @keyframes fadeIn { from { opacity: 0; transform: translateX(-10px); } to { opacity: 1; transform: translateX(0); } }
  button:hover:not(:disabled) { transform: translateY(-3px); }
</style>
