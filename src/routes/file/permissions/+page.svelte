<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  let filePath = "";
  let perms: any = null;
  async function selectFile() {
    const selected = await open({ multiple: false });
    if (selected) { filePath = selected as string; getPermissions(); }
  }
  async function getPermissions() {
    try {
      perms = await invoke("get_file_permissions", { filePath });
    } catch (e) { alert(`Error: ${e}`); }
  }
</script>
<main style="min-height:100vh;padding:2rem;background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%)">
  <div style="max-width:900px;margin:0 auto">
    <div style="text-align:center;margin-bottom:2rem">
      <div style="display:inline-flex;width:80px;height:80px;background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%);border-radius:18px;align-items:center;justify-content:center;margin-bottom:1.5rem;box-shadow:0 8px 24px rgba(203,166,247,0.4);animation:float 3s ease-in-out infinite">
        <i class="nf-md-shield_key" style="font-size:3rem;color:#1e1e2e"></i>
      </div>
      <h1 style="background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%);-webkit-background-clip:text;-webkit-text-fill-color:transparent;font-size:2.5rem;margin-bottom:0.5rem;font-weight:bold">File Permissions</h1>
      <p style="color:#a6adc8;font-size:1.1rem">View file permissions and attributes</p>
    </div>
    <div style="background:rgba(24,24,37,0.7);backdrop-filter:blur(10px);padding:2.5rem;border-radius:20px;border:1px solid rgba(203,166,247,0.15);box-shadow:0 8px 32px rgba(0,0,0,0.3)">
      <button on:click={selectFile} style="width:100%;padding:1.5rem;background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%);color:#1e1e2e;border:none;border-radius:14px;cursor:pointer;font-weight:600;font-size:1.1rem;box-shadow:0 4px 16px rgba(203,166,247,0.3)"><i class="nf-md-file"></i> Select File</button>
      {#if filePath}<p style="color:#cdd6f4;margin-top:1.5rem;font-size:0.95rem;font-family:Consolas,monospace;text-align:center;background:#11111b;padding:1rem;border-radius:10px">{filePath}</p>{/if}
    </div>
    {#if perms}
      <div style="background:rgba(24,24,37,0.7);backdrop-filter:blur(10px);padding:2.5rem;border-radius:20px;margin-top:2rem;border:1px solid rgba(203,166,247,0.15);animation:slideUp 0.5s ease-out;box-shadow:0 8px 32px rgba(0,0,0,0.3)">
        <div style="display:grid;grid-template-columns:1fr 1fr;gap:1.5rem;margin-bottom:1.5rem">
          <div style="background:#11111b;padding:1.5rem;border-radius:16px;text-align:center;border:2px solid #313244">
            <p style="color:#cba6f7;font-weight:bold;margin-bottom:0.5rem;font-size:1.1rem">Octal Mode</p>
            <p style="color:#cdd6f4;font-family:Consolas,monospace;font-size:2rem;font-weight:bold">{perms.octal}</p>
          </div>
          <div style="background:#11111b;padding:1.5rem;border-radius:16px;text-align:center;border:2px solid #313244">
            <p style="color:#cba6f7;font-weight:bold;margin-bottom:0.5rem;font-size:1.1rem">Symbolic</p>
            <p style="color:#cdd6f4;font-family:Consolas,monospace;font-size:2rem;font-weight:bold">{perms.symbolic}</p>
          </div>
        </div>
        <div style="background:#11111b;padding:1.5rem;border-radius:16px;border:2px solid #313244">
          <p style="color:#cba6f7;font-weight:bold;margin-bottom:1rem;font-size:1.1rem">Access Capabilities</p>
          <div style="display:grid;grid-template-columns:1fr 1fr 1fr;gap:1rem;text-align:center">
            <div style="padding:1rem;background:rgba(166,227,161,0.1);border-radius:12px;border:1px solid {perms.readable?'#a6e3a1':'#313244'}">
              <i class="nf-md-eye" style="font-size:2rem;color:{perms.readable?'#a6e3a1':'#45475a'};margin-bottom:0.5rem;display:block"></i>
              <span style="color:#cdd6f4">Read</span>
            </div>
            <div style="padding:1rem;background:rgba(249,226,175,0.1);border-radius:12px;border:1px solid {perms.writable?'#f9e2af':'#313244'}">
              <i class="nf-md-pencil" style="font-size:2rem;color:{perms.writable?'#f9e2af':'#45475a'};margin-bottom:0.5rem;display:block"></i>
              <span style="color:#cdd6f4">Write</span>
            </div>
            <div style="padding:1rem;background:rgba(243,139,168,0.1);border-radius:12px;border:1px solid {perms.executable?'#f38ba8':'#313244'}">
              <i class="nf-md-run" style="font-size:2rem;color:{perms.executable?'#f38ba8':'#45475a'};margin-bottom:0.5rem;display:block"></i>
              <span style="color:#cdd6f4">Execute</span>
            </div>
          </div>
        </div>
      </div>
    {/if}
  </div>
</main>
<style>
  @keyframes float { 0%, 100% { transform: translateY(0); } 50% { transform: translateY(-10px); } }
  @keyframes slideUp { from { opacity: 0; transform: translateY(20px); } to { opacity: 1; transform: translateY(0); } }
  button:hover { transform: translateY(-3px); box-shadow: 0 8px 24px rgba(203, 166, 247, 0.5); }
</style>
