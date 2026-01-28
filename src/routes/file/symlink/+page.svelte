<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  let target = "";
  let linkName = "";
  async function selectTarget() {
    const selected = await open({ multiple: false });
    if (selected) target = selected as string;
  }
  async function create() {
    if (!target || !linkName) return alert("Select target and enter link name");
    try {
      await invoke("create_symlink", { target, linkName });
      alert("Symlink created successfully!");
    } catch (e) { alert(`Error: ${e}`); }
  }
</script>
<main style="min-height:100vh;padding:2rem;background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%)">
  <div style="max-width:900px;margin:0 auto">
    <div style="text-align:center;margin-bottom:2rem">
      <div style="display:inline-flex;width:80px;height:80px;background:linear-gradient(135deg, #94e2d5 0%, #89b4fa 100%);border-radius:18px;align-items:center;justify-content:center;margin-bottom:1.5rem;box-shadow:0 8px 24px rgba(148,226,213,0.4);animation:float 3s ease-in-out infinite">
        <i class="nf-md-link_variant" style="font-size:3rem;color:#1e1e2e"></i>
      </div>
      <h1 style="background:linear-gradient(135deg, #94e2d5 0%, #89b4fa 100%);-webkit-background-clip:text;-webkit-text-fill-color:transparent;font-size:2.5rem;margin-bottom:0.5rem;font-weight:bold">Symlink Manager</h1>
      <p style="color:#a6adc8;font-size:1.1rem">Create symbolic links easily</p>
    </div>
    <div style="background:rgba(24,24,37,0.7);backdrop-filter:blur(10px);padding:3rem;border-radius:20px;border:1px solid rgba(148,226,213,0.15);box-shadow:0 8px 32px rgba(0,0,0,0.3)">
      <div style="margin-bottom:2.5rem">
        <label style="color:#94e2d5;margin-bottom:0.75rem;display:block;font-weight:600">Target File/Directory</label>
        <button on:click={selectTarget} style="width:100%;padding:1.25rem;background:rgba(148,226,213,0.1);color:#94e2d5;border:2px dashed #94e2d5;border-radius:14px;cursor:pointer;font-family:Consolas,monospace;text-align:left;display:flex;align-items:center;gap:0.75rem;font-size:1rem;transition:all 0.3s">
          <i class="nf-md-bullseye_arrow"></i> {target || 'Click to select target...'}
        </button>
      </div>
      
      <div style="margin-bottom:2.5rem">
        <label style="color:#94e2d5;margin-bottom:0.75rem;display:block;font-weight:600">Link Name (Full Path)</label>
        <div style="position:relative">
          <i class="nf-md-rename_box" style="position:absolute;left:1.2rem;top:50%;transform:translateY(-50%);color:#a6adc8;font-size:1.2rem"></i>
          <input type="text" bind:value={linkName} placeholder="/path/to/symlink" style="width:100%;padding:1.25rem 1.25rem 1.25rem 3.5rem;background:#11111b;border:2px solid #313244;border-radius:14px;color:#cdd6f4;font-size:1rem"/>
        </div>
      </div>

      <button on:click={create} style="width:100%;padding:1.5rem;background:linear-gradient(135deg, #94e2d5 0%, #89b4fa 100%);color:#1e1e2e;border:none;border-radius:14px;cursor:pointer;font-weight:bold;font-size:1.3rem;box-shadow:0 6px 20px rgba(148,226,213,0.4);transition:all 0.3s;display:flex;align-items:center;justify-content:center;gap:0.75rem">
        <i class="nf-md-plus_circle"></i> Create Symlink
      </button>
    </div>
  </div>
</main>
<style>
  @keyframes float { 0%, 100% { transform: translateY(0); } 50% { transform: translateY(-10px); } }
  button:hover { transform: translateY(-3px); }
  input:focus { border-color: #94e2d5; outline: none; }
</style>
