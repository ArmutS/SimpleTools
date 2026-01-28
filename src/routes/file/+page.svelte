<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  let tools = [{ id: "file/hash", name: "Hash Generator", icon: "nf-md-pound", width: 800, height: 700, key: "H" },
    { id: "file/rename", name: "File Renamer", icon: "nf-md-rename_box", width: 1000, height: 800, key: "R" },
    { id: "file/duplicate", name: "Duplicate Finder", icon: "nf-md-content_copy", width: 1000, height: 800, key: "D" },
    { id: "file/diskusage", name: "Disk Usage", icon: "nf-md-chart_pie", width: 1000, height: 800, key: "U" },
    { id: "file/split", name: "File Splitter", icon: "nf-md-file_delimited", width: 800, height: 700, key: "S" },
    { id: "file/checksum", name: "Checksum", icon: "nf-md-checkbox_marked_circle", width: 800, height: 600, key: "C" },
    { id: "file/permissions", name: "Permissions", icon: "nf-md-shield_key", width: 800, height: 700, key: "P" },
    { id: "file/tree", name: "Directory Tree", icon: "nf-md-file_tree", width: 900, height: 800, key: "T" },
    { id: "file/watcher", name: "File Watcher", icon: "nf-md-eye", width: 900, height: 700, key: "W" },
    { id: "file/cleaner", name: "Temp Cleaner", icon: "nf-md-broom", width: 900, height: 800, key: "L" },
    { id: "file/metadata", name: "Metadata", icon: "nf-md-information_variant", width: 900, height: 800, key: "M" },
    { id: "file/symlink", name: "Symlink", icon: "nf-md-link_variant", width: 800, height: 700, key: "Y" }];
  async function create_window(id: String, title: String, width: number, height: number) {
    await invoke("create_new_window", { id, title, width, height });
  }
  function handleKeydown(event: KeyboardEvent) {
    const pressedKey = event.key.toUpperCase();
    const tool = tools.find((t) => t.key === pressedKey);
    if (tool) create_window(tool.id, tool.name, tool.width, tool.height);
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<main style="min-height:100vh;padding:1.5rem;background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%);display:flex;flex-direction:column">
  <div style="text-align:center;margin-bottom:1.5rem">
    <div style="display:flex;align-items:center;justify-content:center;gap:0.75rem">
      <div style="width:45px;height:45px;background:linear-gradient(135deg, #89b4fa 0%, #74c7ec 100%);border-radius:11px;display:flex;align-items:center;justify-content:center;box-shadow:0 5px 15px rgba(137,180,250,0.3);animation:float 3s ease-in-out infinite">
        <i class="nf-fa-laptop" style="font-size:1.8rem;color:#1e1e2e"></i>
      </div>
      <h1 style="background:linear-gradient(135deg, #89b4fa 0%, #74c7ec 100%);-webkit-background-clip:text;-webkit-text-fill-color:transparent;font-size:2rem;margin:0;font-weight:bold">File</h1>
    </div>
  </div>
  
  <div style="display:grid;grid-template-columns:repeat(4,1fr);grid-template-rows:repeat(3,1fr);gap:1rem;flex:1">
    {#each tools as tool, i}
      <button on:click={() => create_window(tool.id, tool.name, tool.width, tool.height)} style="position:relative;background:rgba(24,24,37,0.7);backdrop-filter:blur(10px);border:2px solid rgba(137,180,250,0.12);border-radius:14px;cursor:pointer;transition:all 0.3s;animation:fadeIn 0.5s ease-out {i * 0.04}s backwards;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:0.75rem;padding:1.5rem">
        {#if tool.key}
          <div style="position:absolute;top:10px;right:10px;background:linear-gradient(135deg, #89b4fa 0%, #74c7ec 100%);color:#1e1e2e;font-size:0.85rem;font-weight:bold;width:26px;height:26px;display:flex;align-items:center;justify-content:center;border-radius:6px;font-family:Consolas,monospace">{tool.key}</div>
        {/if}
        <i class="{tool.icon}" style="font-size:2.5rem;color:#89b4fa"></i>
        <span style="color:#cdd6f4;font-size:1rem;font-weight:600;text-align:center">{tool.name}</span>
      </button>
    {/each}
  </div>
</main>

<style>
  @keyframes float { 0%, 100% { transform: translateY(0); } 50% { transform: translateY(-7px); } }
  @keyframes fadeIn { from { opacity: 0; transform: translateY(15px); } to { opacity: 1; transform: translateY(0); } }
  button:hover { transform: translateY(-3px); box-shadow: 0 6px 18px rgba(137, 180, 250, 0.2); border-color: rgba(137, 180, 250, 0.3); }
</style>
