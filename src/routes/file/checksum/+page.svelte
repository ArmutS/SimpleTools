<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  let filePath = "";
  let algo = "sha256";
  let expectedHash = "";
  let result: any = null;
  async function selectFile() {
    const selected = await open({ multiple: false });
    if (selected) filePath = selected as string;
  }
  async function verify() {
    if (!filePath || !expectedHash) return alert("Select file and enter expected hash");
    try {
      result = await invoke("verify_checksum", { filePath, expectedHash, algorithm: algo });
    } catch (e) { alert(`Error: ${e}`); }
  }
</script>
<main style="min-height:100vh;padding:2rem;background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%)">
  <div style="max-width:900px;margin:0 auto">
    <div style="text-align:center;margin-bottom:2rem">
      <div style="display:inline-flex;width:80px;height:80px;background:linear-gradient(135deg, #94e2d5 0%, #74c7ec 100%);border-radius:18px;align-items:center;justify-content:center;margin-bottom:1.5rem;box-shadow:0 8px 24px rgba(148,226,213,0.4);animation:float 3s ease-in-out infinite">
        <i class="nf-md-checkbox_marked_circle" style="font-size:3rem;color:#1e1e2e"></i>
      </div>
      <h1 style="background:linear-gradient(135deg, #94e2d5 0%, #74c7ec 100%);-webkit-background-clip:text;-webkit-text-fill-color:transparent;font-size:2.5rem;margin-bottom:0.5rem;font-weight:bold">Checksum Verifier</h1>
      <p style="color:#a6adc8;font-size:1.1rem">Verify file integrity with checksums</p>
    </div>
    <div style="background:rgba(24,24,37,0.7);backdrop-filter:blur(10px);padding:2.5rem;border-radius:20px;border:1px solid rgba(148,226,213,0.15);box-shadow:0 8px 32px rgba(0,0,0,0.3)">
      <button on:click={selectFile} style="width:100%;padding:1.5rem;background:linear-gradient(135deg, #94e2d5 0%, #74c7ec 100%);color:#1e1e2e;border:none;border-radius:14px;cursor:pointer;font-size:1.1rem;font-weight:600;margin-bottom:2rem;box-shadow:0 4px 16px rgba(148,226,213,0.3)"><i class="nf-md-file"></i> Select File</button>
      {#if filePath}<p style="color:#cdd6f4;margin-bottom:2rem;font-family:Consolas,monospace;background:#11111b;padding:1rem;border-radius:10px">{filePath}</p>{/if}
      <div style="margin-bottom:2rem">
        <label style="color:#94e2d5;margin-bottom:0.75rem;display:block;font-weight:600">Algorithm</label>
        <select bind:value={algo} style="width:100%;padding:1rem;background:#11111b;border:2px solid #313244;border-radius:12px;color:#cdd6f4;font-size:1rem;cursor:pointer">
          <option value="md5">MD5</option>
          <option value="sha256">SHA256</option>
          <option value="sha512">SHA512</option>
        </select>
      </div>
      <div style="margin-bottom:2rem">
        <label style="color:#94e2d5;margin-bottom:0.75rem;display:block;font-weight:600">Expected Hash</label>
        <input type="text" bind:value={expectedHash} placeholder="Enter expected hash..." style="width:100%;padding:1rem;background:#11111b;border:2px solid #313244;border-radius:12px;color:#cdd6f4;font-family:Consolas,monospace;font-size:1rem"/>
      </div>
      <button on:click={verify} style="width:100%;padding:1.5rem;background:linear-gradient(135deg, #94e2d5 0%, #74c7ec 100%);color:#1e1e2e;border:none;border-radius:14px;cursor:pointer;font-weight:600;font-size:1.1rem;box-shadow:0 4px 16px rgba(148,226,213,0.3)"><i class="nf-md-check_decagram"></i> Verify Integrity</button>
    </div>
    {#if result}
      <div style="background:rgba(24,24,37,0.7);backdrop-filter:blur(10px);padding:2.5rem;border-radius:20px;margin-top:2rem;text-align:center;border:2px solid {result.match ? '#a6e3a1' : '#f38ba8'};animation:slideUp 0.5s ease-out;box-shadow:0 8px 32px rgba(0,0,0,0.3)">
        <i class="{result.match ? 'nf-md-check_circle' : 'nf-md-close_circle'}" style="font-size:5rem;color:{result.match ? '#a6e3a1' : '#f38ba8'};margin-bottom:1.5rem;display:block"></i>
        <h2 style="color:{result.match ? '#a6e3a1' : '#f38ba8'};font-size:2.5rem;margin-bottom:1rem;font-weight:bold">{result.match ? 'Integrity Verified!' : 'Hash Mismatch!'}</h2>
        <div style="text-align:left;background:#11111b;padding:1.5rem;border-radius:14px;margin-top:1.5rem">
          <p style="color:#a6adc8;margin-bottom:0.5rem">Computed Hash:</p>
          <code style="color:#cdd6f4;font-family:Consolas,monospace;word-break:break-all;font-size:1rem">{result.computed_hash}</code>
        </div>
      </div>
    {/if}
  </div>
</main>
<style>
  @keyframes float { 0%, 100% { transform: translateY(0px); } 50% { transform: translateY(-10px); } }
  @keyframes slideUp { from { opacity: 0; transform: translateY(20px); } to { opacity: 1; transform: translateY(0); } }
  button:hover { transform: translateY(-3px); box-shadow: 0 8px 24px rgba(148, 226, 213, 0.5); }
  input:focus, select:focus { border-color: #94e2d5; outline: none; }
</style>
