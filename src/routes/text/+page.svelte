<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  let tools = [{ id: "text/diff", name: "Diff Viewer", icon: "nf-oct-diff", width: 1000, height: 800, key: "D" },
    { id: "text/regex", name: "Regex Tester", icon: "nf-fa-code", width: 900, height: 700, key: "R" },
    { id: "text/extractor", name: "Extractor", icon: "nf-fa-filter", width: 800, height: 900, key: "E" },
    { id: "text/strip", name: "Strip HTML", icon: "nf-fa-eraser", width: 800, height: 800, key: "S" },
    { id: "text/string", name: "String Escaper", icon: "nf-md-format_text", width: 800, height: 600, key: "N" },
    { id: "text/slug", name: "Slug Generator", icon: "nf-md-link_variant", width: 600, height: 400, key: "U" },
    { id: "text/jwt", name: "JWT Decoder", icon: "nf-md-key_variant", width: 900, height: 700, key: "J" },
    { id: "text/cron", name: "Cron Explainer", icon: "nf-md-clock_outline", width: 700, height: 500, key: "C" },
    { id: "text/markdown", name: "Markdown", icon: "nf-md-language_markdown", width: 1000, height: 800, key: "M" },
    { id: "text/lorem", name: "Lorem Generator", icon: "nf-fa-paragraph", width: 700, height: 600, key: "L" },
    { id: "text/obfuscator", name: "Obfuscator", icon: "nf-fa-user_secret", width: 700, height: 600, key: "O" },
    { id: "text/character", name: "Character", icon: "nf-fa-info_circle", width: 600, height: 600, key: "H" }];
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
      <div style="width:45px;height:45px;background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%);border-radius:11px;display:flex;align-items:center;justify-content:center;box-shadow:0 5px 15px rgba(137,180,250,0.3);animation:float 3s ease-in-out infinite">
        <i class="nf-fa-font" style="font-size:1.8rem;color:#1e1e2e"></i>
      </div>
      <h1 style="background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%);-webkit-background-clip:text;-webkit-text-fill-color:transparent;font-size:2rem;margin:0;font-weight:bold">Text</h1>
    </div>
  </div>
  
  <div style="display:grid;grid-template-columns:repeat(4,1fr);grid-template-rows:repeat(3,1fr);gap:1rem;flex:1">
    {#each tools as tool, i}
      <button on:click={() => create_window(tool.id, tool.name, tool.width, tool.height)} style="position:relative;background:rgba(24,24,37,0.7);backdrop-filter:blur(10px);border:2px solid rgba(137,180,250,0.12);border-radius:14px;cursor:pointer;transition:all 0.3s;animation:fadeIn 0.5s ease-out {i * 0.04}s backwards;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:0.75rem;padding:1.5rem">
        {#if tool.key}
          <div style="position:absolute;top:10px;right:10px;background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%);color:#1e1e2e;font-size:0.85rem;font-weight:bold;width:26px;height:26px;display:flex;align-items:center;justify-content:center;border-radius:6px;font-family:Consolas,monospace">{tool.key}</div>
        {/if}
        <i class="{tool.icon}" style="font-size:2.5rem;color:#cba6f7"></i>
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
