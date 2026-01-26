<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let tools = [
    {
      id: "convert/office",
      name: "Office Converter",
      icon: "nf-md-file_document_multiple",
      width: 900,
      height: 700,
      key: "O",
    },
    {
      id: "convert/image",
      name: "Image Converter",
      icon: "nf-md-image",
      width: 900,
      height: 700,
      key: "I",
    },
    {
      id: "convert/video",
      name: "Video Converter",
      icon: "nf-md-video",
      width: 900,
      height: 700,
      key: "V",
    },
    {
      id: "convert/audio",
      name: "Audio Converter",
      icon: "nf-md-music_note",
      width: 800,
      height: 600,
      key: "A",
    },
    {
      id: "convert/archive",
      name: "Archive Converter",
      icon: "nf-md-zip_box",
      width: 800,
      height: 600,
      key: "Z",
    },
    {
      id: "convert/ebook",
      name: "E-Book Converter",
      icon: "nf-md-book_open_variant",
      width: 800,
      height: 600,
      key: "E",
    },
    {
      id: "convert/font",
      name: "Font Converter",
      icon: "nf-md-format_font",
      width: 800,
      height: 600,
      key: "F",
    },
    {
      id: "convert/data",
      name: "Data Converter",
      icon: "nf-md-database",
      width: 1000,
      height: 800,
      key: "D",
    },
    {
      id: "convert/vector",
      name: "Vector Converter",
      icon: "nf-md-vector_curve",
      width: 900,
      height: 700,
      key: "S",
    },
    {
      id: "convert/cad",
      name: "3D/CAD Converter",
      icon: "nf-md-cube_outline",
      width: 900,
      height: 700,
      key: "M",
    },
    {
      id: "convert/icon",
      name: "Icon Converter",
      icon: "nf-md-emoticon_outline",
      width: 800,
      height: 600,
      key: "C",
    },
    {
      id: "convert/subtitle",
      name: "Subtitle Converter",
      icon: "nf-md-subtitles",
      width: 800,
      height: 600,
      key: "T",
    },
  ];

  async function create_window(
    id: String,
    title: String,
    width: number,
    height: number,
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
