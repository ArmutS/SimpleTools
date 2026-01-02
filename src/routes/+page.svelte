<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let tools = [
    { id: "text", name: "Text Tools", icon: "nf-fa-font", key: "T" },
    { id: "pdf", name: "PDF Tools", icon: "nf-fa-file_pdf", key: "P" },
    {
      id: "convert",
      name: "Converters",
      icon: "nf-fa-exchange",
      key: "C",
    },
    { id: "file", name: "File & System", icon: "nf-fa-laptop", key: "F" },
    { id: "image", name: "Image Tools", icon: "nf-fa-image", key: "I" },
    { id: "network", name: "Network", icon: "nf-fa-wifi", key: "N" },
    {
      id: "quickcmd",
      name: "Quick Cmds",
      icon: "nf-fa-terminal",
      key: "Q",
    },
    { id: "dev", name: "Dev Tools", icon: "nf-fa-code", key: "D" },
    { id: "soon", name: "Coming Soon", icon: "nf-fa-question_circle", key: "" },
    { id: "soon", name: "Coming Soon", icon: "nf-fa-question_circle", key: "" },
    { id: "soon", name: "Coming Soon", icon: "nf-fa-question_circle", key: "" },
    { id: "soon", name: "Coming Soon", icon: "nf-fa-question_circle", key: "" },
  ];

  async function create_window(id: String, title: String) {
    await invoke("create_new_window", { id: id, title: title });
  }

  function handleKeydown(event: KeyboardEvent) {
    const pressedKey = event.key.toUpperCase();
    const tool = tools.find((t) => t.key === pressedKey);
    if (tool) {
      create_window(tool.id, tool.name);
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<main class="main">
  <div class="tools-box">
    {#each tools as tool}
      <button
        class="tools-buttons"
        on:click={() => create_window(tool.id, tool.name)}
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
    position: relative; /* Badge positioning */
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
