<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  let input = "";
  let slug = "";

  async function generate() {
    if (!input) { slug=""; return; }
    try {
      slug = await invoke("process_slug_gen", { currentText: input });
    } catch (e) { alert(e); }
  }
</script>

<main class="container">
  <div class="content">
    <div class="header">
      <div class="icon-wrapper"><i class="nf-md-link_variant"></i></div>
      <h1>Slug Generator</h1>
      <p class="subtitle">Convert text into URL-friendly slugs</p>
    </div>

    <div class="input-section">
      <input type="text" bind:value={input} on:input={generate} placeholder="Enter article title or text..." />
    </div>

    {#if slug}
      <div class="result-section">
        <label>URL Slug</label>
        <div class="slug-box" on:click={() => navigator.clipboard.writeText(slug)}>
          {slug}
          <i class="nf-md-content_copy"></i>
        </div>
      </div>
    {/if}
  </div>
</main>

<style>
  /* Styling */
  .container { min-height:100vh; padding:3rem 2rem; background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%); }
  .content { max-width:700px; margin:0 auto; }
  .header { text-align:center; margin-bottom:3rem; }
  .icon-wrapper { display:inline-flex; width:80px; height:80px; background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%); border-radius:18px; align-items:center; justify-content:center; margin-bottom:1.5rem; box-shadow:0 8px 24px rgba(203,166,247,0.4); animation:float 3s ease-in-out infinite; font-size:3rem; color:#1e1e2e; }
  h1 { background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%); -webkit-background-clip:text; -webkit-text-fill-color:transparent; font-size:2.5rem; margin-bottom:0.5rem; font-weight:bold; }
  .subtitle { color:#a6adc8; font-size:1.1rem; }
  .input-section, .result-section { background:rgba(24,24,37,0.7); backdrop-filter:blur(10px); padding:2rem; border-radius:20px; border:1px solid rgba(203,166,247,0.15); box-shadow:0 8px 32px rgba(0,0,0,0.3); margin-bottom:2rem; animation:slideUp 0.5s ease-out; }
  
  input { width:100%; padding:1.25rem; background:#11111b; border:2px solid #313244; border-radius:12px; color:#cdd6f4; font-size:1.2rem; transition:all 0.3s; text-align:center; }
  input:focus { border-color:#cba6f7; outline:none; }
  
  .slug-box { background:#11111b; padding:1.5rem; border-radius:12px; border:2px dashed #cba6f7; color:#cba6f7; font-size:1.5rem; font-weight:600; text-align:center; cursor:pointer; display:flex; align-items:center; justify-content:center; gap:1rem; transition:all 0.2s; word-break:break-all; }
  .slug-box:hover { background:rgba(203,166,247,0.1); transform:translateY(-2px); }
  label { display:block; text-align:center; color:#a6adc8; margin-bottom:1rem; font-weight:600; }
  
  @keyframes float { 0%, 100% { transform:translateY(0); } 50% { transform:translateY(-10px); } }
  @keyframes slideUp { from { opacity:0; transform:translateY(20px); } to { opacity:1; transform:translateY(0); } }
</style>
