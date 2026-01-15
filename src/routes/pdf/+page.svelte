<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let tools = [
    {
      id: "pdf/merger",
      name: "PDF Merger",
      icon: "nf-md-file_multiple",
      width: 900,
      height: 700,
      key: "M",
    },
    {
      id: "pdf/splitter",
      name: "PDF Splitter",
      icon: "nf-md-content_cut",
      width: 900,
      height: 700,
      key: "S",
    },
    {
      id: "pdf/images-to-pdf",
      name: "Images to PDF",
      icon: "nf-md-image_multiple",
      width: 800,
      height: 700,
      key: "I",
    },
    {
      id: "pdf/pdf-to-images",
      name: "PDF to Images",
      icon: "nf-md-image_outline",
      width: 800,
      height: 700,
      key: "G",
    },
    {
      id: "pdf/compress",
      name: "Compress PDF",
      icon: "nf-md-compress",
      width: 800,
      height: 600,
      key: "C",
    },
    {
      id: "pdf/rotate",
      name: "Rotate Pages",
      icon: "nf-md-rotate_right",
      width: 900,
      height: 700,
      key: "R",
    },
    {
      id: "pdf/delete",
      name: "Delete Pages",
      icon: "nf-md-delete",
      width: 800,
      height: 700,
      key: "D",
    },
    {
      id: "pdf/extract-text",
      name: "Extract Text",
      icon: "nf-md-text_box",
      width: 900,
      height: 800,
      key: "T",
    },
    {
      id: "pdf/remove-password",
      name: "Remove Password",
      icon: "nf-md-lock_open",
      width: 700,
      height: 500,
      key: "U",
    },
    {
      id: "pdf/protect",
      name: "Protect PDF",
      icon: "nf-md-lock",
      width: 800,
      height: 600,
      key: "P",
    },
    {
      id: "pdf/watermark",
      name: "Watermark",
      icon: "nf-md-watermark",
      width: 900,
      height: 700,
      key: "W",
    },
    {
      id: "pdf/metadata",
      name: "Metadata Editor",
      icon: "nf-md-information",
      width: 800,
      height: 700,
      key: "E",
    },
  ];

  async function create_window(
    id: String,
    title: String,
    width: number,
    height: number
  ) {
    await invoke("create_new_window", {
      id: id,
      title: title,
      width: width,
      height: height,
    });
  }

  function handleKeydown(event: KeyboardEvent) {
    const pressedKey = event.key.toUpperCase();
    const tool = tools.find((t) => t.key === pressedKey);
    if (tool) {
      create_window(tool.id, tool.name, tool.width, tool.height);
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<main class="main">
  <div class="tools-box">
    {#each tools as tool}
      <button
        class="tools-buttons"
        on:click={() =>
          create_window(tool.id, tool.name, tool.width, tool.height)}
      >
        {#if tool.key}
          <div class="key-badge">{tool.key}</div>
        {/if}
        <i class="{tool.icon} tool-icon"></i>

        <span class="tool-name">{tool.name}</span>
      </button>
    {/each}
  </div>
</main>

<style>
  .tools-buttons {
    position: relative;
  }

  .key-badge {
    position: absolute;
    top: 8px;
    right: 8px;
    background-color: var(--accent);
    color: var(--bg-app);
    font-size: 1.4rem;
    font-weight: bold;
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 8px;
    font-family: "Consolas", monospace;
    opacity: 0.8;
  }
</style>
