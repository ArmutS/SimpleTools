<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let tools = [
    {
      id: "text/diff",
      name: "Diff Viewer",
      icon: "nf-seti-text",
      width: 1000,
      height: 800,
    },
    {
      id: "text/regex",
      name: "Regex Tester",
      icon: "nf-seti-pdf",
      width: 900,
      height: 700,
    },
    {
      id: "text/extractor",
      name: "Extractor",
      icon: "nf-md-file_arrow_left_right_outline",
      width: 800,
      height: 900,
    },
    {
      id: "text/strip",
      name: "Strip HTML Tags",
      icon: "nf-oct-file",
      width: 800,
      height: 800,
    },
    {
      id: "text/string",
      name: "String Escaper / Unescaper",
      icon: "nf-fa-image",
      width: 800,
      height: 600,
    },
    {
      id: "text/slug",
      name: "Slug Generator",
      icon: "nf-md-console_network",
      width: 600,
      height: 400,
    },
    {
      id: "text/jwt",
      name: "JWT Decoder",
      icon: "nf-dev-devicon",
      width: 900,
      height: 700,
    },
    {
      id: "text/cron",
      name: "Cron Expression Explainer",
      icon: "nf-oct-command_palette",
      width: 700,
      height: 500,
    },
    {
      id: "text/markdown",
      name: "Markdown Preview",
      icon: "nf-fae-comet",
      width: 1000,
      height: 800,
    },
    {
      id: "text/lorem",
      name: "Lorem Ipsum Generator",
      icon: "nf-fae-comet",
      width: 700,
      height: 600,
    },
    {
      id: "text/obfuscator",
      name: "Text Obfuscator / Rot13",
      icon: "nf-fae-comet",
      width: 700,
      height: 600,
    },
    {
      id: "text/character",
      name: "Character Inspector",
      icon: "nf-fae-comet",
      width: 600,
      height: 600,
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
</script>

<main class="main">
  <div class="tools-box">
    {#each tools as tool}
      <button
        class="tools-buttons"
        on:click={() =>
          create_window(tool.id, tool.name, tool.width, tool.height)}
      >
        <i class="{tool.icon} tool-icon"></i>

        <span class="tool-name">{tool.name}</span>
      </button>
    {/each}
  </div>
</main>

<style>
</style>
