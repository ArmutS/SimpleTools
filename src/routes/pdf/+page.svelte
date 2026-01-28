<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  let tools = [{ id: "pdf/merger", name: "Merger", icon: "nf-md-file_multiple", width: 1200, height: 800, key: "M" },
    { id: "pdf/splitter", name: "Splitter", icon: "nf-md-content_cut", width: 900, height: 700, key: "S" },
    { id: "pdf/images-to-pdf", name: "Images→PDF", icon: "nf-md-image_multiple", width: 800, height: 700, key: "I" },
    { id: "pdf/pdf-to-images", name: "PDF→Images", icon: "nf-md-image_outline", width: 800, height: 700, key: "G" },
    { id: "pdf/compress", name: "Compress", icon: "nf-md-compress", width: 800, height: 600, key: "C" },
    { id: "pdf/rotate", name: "Rotate Pages", icon: "nf-md-rotate_right", width: 900, height: 700, key: "R" },
    { id: "pdf/delete", name: "Delete Pages", icon: "nf-md-delete", width: 800, height: 700, key: "D" },
    { id: "pdf/extract-text", name: "Extract Text", icon: "nf-md-text_box", width: 900, height: 800, key: "T" },
    { id: "pdf/remove-password", name: "Remove Pass", icon: "nf-md-lock_open", width: 700, height: 500, key: "U" },
    { id: "pdf/protect", name: "Protect PDF", icon: "nf-md-lock", width: 800, height: 600, key: "P" },
    { id: "pdf/watermark", name: "Watermark", icon: "nf-md-watermark", width: 900, height: 700, key: "W" },
    { id: "pdf/metadata", name: "Metadata", icon: "nf-md-information", width: 800, height: 700, key: "E" }];
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
      <div style="width:45px;height:45px;background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%);border-radius:11px;display:flex;align-items:center;justify-content:center;box-shadow:0 5px 15px rgba(137,180,250,0.3);animation:float 3s ease-in-out infinite">
        <i class="nf-fa-file_pdf" style="font-size:1.8rem;color:#1e1e2e"></i>
      </div>
      <h1 style="background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%);-webkit-background-clip:text;-webkit-text-fill-color:transparent;font-size:2rem;margin:0;font-weight:bold">PDF</h1>
    </div>
  </div>
  
  <div style="display:grid;grid-template-columns:repeat(4,1fr);grid-template-rows:repeat(3,1fr);gap:1rem;flex:1">
    {#each tools as tool, i}
      <button on:click={() => create_window(tool.id, tool.name, tool.width, tool.height)} style="position:relative;background:rgba(24,24,37,0.7);backdrop-filter:blur(10px);border:2px solid rgba(137,180,250,0.12);border-radius:14px;cursor:pointer;transition:all 0.3s;animation:fadeIn 0.5s ease-out {i * 0.04}s backwards;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:0.75rem;padding:1.5rem">
        {#if tool.key}
          <div style="position:absolute;top:10px;right:10px;background:linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%);color:#1e1e2e;font-size:0.85rem;font-weight:bold;width:26px;height:26px;display:flex;align-items:center;justify-content:center;border-radius:6px;font-family:Consolas,monospace">{tool.key}</div>
        {/if}
        <i class="{tool.icon}" style="font-size:2.5rem;color:#f38ba8"></i>
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
