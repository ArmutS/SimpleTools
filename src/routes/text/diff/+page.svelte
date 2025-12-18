<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  interface DiffItem {
    text: string;
    tag: "left" | "right" | "not";
  }

  // SADECE SATIR MODU
  const select = 2;

  let leftRender: DiffItem[] = [];
  let rightRender: DiffItem[] = [];
  let firstText = "";
  let secondText = "";
  let statusMessage = "Hazır";

  let leftBackdrop: HTMLDivElement;
  let rightBackdrop: HTMLDivElement;

  $: if (firstText || secondText) {
    runDiff();
  }

  function handleScroll(e: Event, side: "left" | "right") {
    const target = e.target as HTMLTextAreaElement;
    if (side === "left" && leftBackdrop) {
      leftBackdrop.scrollTop = target.scrollTop;
      leftBackdrop.scrollLeft = target.scrollLeft;
    } else if (side === "right" && rightBackdrop) {
      rightBackdrop.scrollTop = target.scrollTop;
      rightBackdrop.scrollLeft = target.scrollLeft;
    }
  }

  async function runDiff() {
    if (!firstText && !secondText) {
      leftRender = [];
      rightRender = [];
      statusMessage = "Veri bekleniyor...";
      return;
    }

    try {
      statusMessage = "Hesaplanıyor...";
      const result: DiffItem[] = await invoke("process_text_diff", {
        select: select,
        left_in: firstText,
        right_in: secondText,
      });

      const l_temp: DiffItem[] = [];
      const r_temp: DiffItem[] = [];

      result.forEach((item) => {
        if (item.tag === "not") {
          l_temp.push(item);
          r_temp.push(item);
        } else if (item.tag === "left") {
          l_temp.push(item);
        } else if (item.tag === "right") {
          r_temp.push(item);
        }
      });

      leftRender = l_temp;
      rightRender = r_temp;
      statusMessage = "Senkronize";
    } catch (error) {
      console.error(error);
      statusMessage = "Hata oluştu";
    }
  }
</script>

<main class="main">
  <div class="diff-container">
    <div class="pane">
      <div class="pane-header">
        <span class="dot red"></span>
        <span>ESKİ METİN</span>
      </div>
      <div class="stack-wrapper">
        <div class="backdrop" bind:this={leftBackdrop}>
          {#each leftRender as block}
            <span class={block.tag}>{block.text}</span>
          {/each}
          <span class="spacer">&nbsp;</span>
        </div>
        <textarea
          class="editor-front"
          bind:value={firstText}
          on:scroll={(e) => handleScroll(e, "left")}
          placeholder="Eski metni yapıştır..."
          spellcheck="false"
        ></textarea>
      </div>
    </div>

    <div class="divider"></div>

    <div class="pane">
      <div class="pane-header">
        <span class="dot green"></span>
        <span>YENİ METİN</span>
      </div>
      <div class="stack-wrapper">
        <div class="backdrop" bind:this={rightBackdrop}>
          {#each rightRender as block}
            <span class={block.tag}>{block.text}</span>
          {/each}
          <span class="spacer">&nbsp;</span>
        </div>
        <textarea
          class="editor-front"
          bind:value={secondText}
          on:scroll={(e) => handleScroll(e, "right")}
          placeholder="Yeni metni yapıştır..."
          spellcheck="false"
        ></textarea>
      </div>
    </div>
  </div>

  <div class="status-bar">
    <span class="msg">{statusMessage}</span>
    <span class="info">MOD: SATIR</span>
  </div>
</main>

<style>
  /* CSS Değişkenlerini (Variables) burada tanımlamıyoruz.
     Senin 'catppuccin.css' dosyanın yüklü olduğunu varsayarak
     doğrudan var(--degisken-adi) kullanıyoruz.
  */

  .main {
    height: 100vh;
    display: flex;
    flex-direction: column;

    /* Global Temadan Gelen Renkler */
    background-color: var(--bg-app);
    color: var(--text-main);

    /* Hizalama Fontu */
    font-family: "Consolas", monospace;
  }

  /* --- LAYOUT --- */
  .diff-container {
    flex: 1;
    display: flex;
    gap: 1px;
    background-color: var(--border-color);
    min-height: 0;
  }

  .pane {
    flex: 1;
    display: flex;
    flex-direction: column;
    background-color: var(--bg-app);
    min-width: 0;
  }

  /* --- HEADER --- */
  .pane-header {
    height: 32px;
    padding: 0 12px;

    /* Tema Renkleri */
    background: var(--bg-header);
    color: var(--text-muted);
    border-bottom: 1px solid var(--border-color);

    font-size: 0.75rem;
    font-weight: bold;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
  }
  /* Noktalar için de temadaki text renklerini kullanıyoruz */
  .dot.red {
    background-color: var(--diff-del-text);
  }
  .dot.green {
    background-color: var(--diff-add-text);
  }

  /* --- STACK (Katmanlama) --- */
  .stack-wrapper {
    position: relative;
    flex: 1;
    overflow: hidden;
  }

  .backdrop,
  .editor-front {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    margin: 0;
    padding: 10px;
    border: none;
    box-sizing: border-box;
    font-family: "HurmitNerd", monospace;
    font-size: 14px;
    line-height: 20px;
    white-space: pre-wrap;
    word-wrap: break-word;
  }

  /* --- BACKDROP (HIGHLIGHT ALANI) --- */
  .backdrop {
    z-index: 1;
    background: transparent;
    color: transparent; /* Metin görünmez */
    pointer-events: none;
    overflow: hidden;
  }

  /* HIGHLIGHT RENKLERİ:
     Burada sadece senin tanımladığın değişkenleri çağırdım.
     !important ekledim ki başka stiller ezmesin.
  */
  .backdrop span.left {
    background-color: var(--diff-del-bg) !important;
  }

  .backdrop span.right {
    background-color: var(--diff-add-bg) !important;
  }

  .spacer {
    display: block;
    height: 50px;
  }

  /* --- EDITOR (YAZI ALANI) --- */
  .editor-front {
    z-index: 2;
    background: transparent;
    color: var(--text-main);
    caret-color: var(--text-main);
    resize: none;
    outline: none;
    overflow: auto;
  }
  .editor-front::-webkit-scrollbar {
    width: 0px;
  }

  /* --- STATUS BAR --- */
  .status-bar {
    height: 28px;
    background-color: var(--bg-header);
    border-top: 1px solid var(--border-color);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 12px;
    font-size: 0.75rem;
    color: var(--text-muted);
    user-select: none;
  }

  .status-bar .msg {
    color: var(--accent);
    font-weight: 600;
  }
</style>
