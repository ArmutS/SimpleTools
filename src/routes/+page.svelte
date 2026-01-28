<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let tools = [
    { id: "text", name: "Text Tools", icon: "nf-fa-font", key: "T", color: "#cba6f7", gradient: "linear-gradient(135deg, #cba6f7 0%, #b4befe 100%)" },
    { id: "pdf", name: "PDF Tools", icon: "nf-fa-file_pdf", key: "P", color: "#f38ba8", gradient: "linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%)" },
    { id: "convert", name: "Converters", icon: "nf-fa-exchange", key: "C", color: "#94e2d5", gradient: "linear-gradient(135deg, #94e2d5 0%, #74c7ec 100%)" },
    { id: "file", name: "File & System", icon: "nf-fa-laptop", key: "F", color: "#89b4fa", gradient: "linear-gradient(135deg, #89b4fa 0%, #74c7ec 100%)" },
    { id: "image", name: "Image Tools", icon: "nf-fa-image", key: "I", color: "#f9e2af", gradient: "linear-gradient(135deg, #f9e2af 0%, #fab387 100%)" },
    { id: "network", name: "Network", icon: "nf-fa-wifi", key: "N", color: "#a6e3a1", gradient: "linear-gradient(135deg, #a6e3a1 0%, #94e2d5 100%)" },
    { id: "quickcmd", name: "Quick Cmds", icon: "nf-fa-terminal", key: "Q", color: "#f5c2e7", gradient: "linear-gradient(135deg, #f5c2e7 0%, #eba0ac 100%)" },
    { id: "dev", name: "Dev Tools", icon: "nf-fa-code", key: "D", color: "#74c7ec", gradient: "linear-gradient(135deg, #74c7ec 0%, #89b4fa 100%)" },
    { id: "soon", name: "Coming Soon", icon: "nf-fa-question_circle", key: "", color: "#6c7086" },
    { id: "soon", name: "Coming Soon", icon: "nf-fa-question_circle", key: "", color: "#6c7086" },
    { id: "soon", name: "Coming Soon", icon: "nf-fa-question_circle", key: "", color: "#6c7086" },
    { id: "soon", name: "Coming Soon", icon: "nf-fa-question_circle", key: "", color: "#6c7086" },
  ];

  async function create_window(id: String, title: String) {
    if (id === "soon") return;
    await invoke("create_new_window", { id, title });
  }

  function handleKeydown(event: KeyboardEvent) {
    const pressedKey = event.key.toUpperCase();
    const tool = tools.find((t) => t.key === pressedKey);
    if (tool && tool.id !== "soon") create_window(tool.id, tool.name);
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<main style="min-height:100vh;padding:1.5rem;background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%);display:flex;flex-direction:column">
  <div style="text-align:center;margin-bottom:1.5rem">
    <div style="display:flex;align-items:center;justify-content:center;gap:0.75rem;margin-bottom:0.5rem">
      <div style="width:50px;height:50px;background:linear-gradient(135deg, #89b4fa 0%, #cba6f7 100%);border-radius:12px;display:flex;align-items:center;justify-content:center;box-shadow:0 6px 18px rgba(137,180,250,0.4);animation:float 3s ease-in-out infinite">
        <i class="nf-fa-cube" style="font-size:1.8rem;color:#1e1e2e"></i>
      </div>
      <h1 style="background:linear-gradient(135deg, #89b4fa 0%, #cba6f7 100%);-webkit-background-clip:text;-webkit-text-fill-color:transparent;font-size:2.2rem;margin:0;font-weight:bold">SimpleTools</h1>
    </div>
  </div>
  
  <div style="display:grid;grid-template-columns:repeat(4,1fr);grid-template-rows:repeat(3,1fr);gap:1rem;flex:1">
    {#each tools as tool, i}
      <button on:click={() => create_window(tool.id, tool.name)} disabled={tool.id === 'soon'} style="position:relative;background:rgba(24,24,37,0.7);backdrop-filter:blur(10px);border:2px solid rgba(137,180,250,0.15);border-radius:14px;cursor:{tool.id==='soon'?'not-allowed':'pointer'};transition:all 0.3s;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:0.75rem;animation:fadeIn 0.5s ease-out {i * 0.04}s backwards;opacity:{tool.id==='soon'?0.4:1};padding:1.5rem">
        {#if tool.key}
          <div style="position:absolute;top:10px;right:10px;background:{tool.gradient||tool.color};color:#1e1e2e;font-size:0.9rem;font-weight:bold;width:28px;height:28px;display:flex;align-items:center;justify-content:center;border-radius:7px;font-family:Consolas,monospace">{tool.key}</div>
        {/if}
        <div style="width:60px;height:60px;border-radius:14px;display:flex;align-items:center;justify-content:center;background:{tool.gradient||`rgba(${tool.color},0.15)`}">
          <i class="{tool.icon}" style="font-size:2.5rem;color:{tool.color}"></i>
        </div>
        <span style="color:#cdd6f4;font-size:1.05rem;font-weight:600;text-align:center">{tool.name}</span>
        {#if tool.id === 'soon'}
          <span style="position:absolute;bottom:10px;right:10px;background:rgba(108,112,134,0.3);color:#6c7086;padding:0.3rem 0.6rem;border-radius:6px;font-size:0.75rem;font-weight:600">Soon</span>
        {/if}
      </button>
    {/each}
  </div>
</main>

<style>
  @keyframes float { 0%, 100% { transform: translateY(0); } 50% { transform: translateY(-8px); } }
  @keyframes fadeIn { from { opacity: 0; transform: translateY(15px); } to { opacity: 1; transform: translateY(0); } }
  button:not(:disabled):hover { transform: translateY(-4px); box-shadow: 0 8px 20px rgba(137, 180, 250, 0.25); border-color: rgba(137, 180, 250, 0.4); }
</style>
