<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let tools = [
    {
      id: "text/diff",
      name: "Diff Viewer",
      icon: "nf-oct-diff",
      width: 1000,
      height: 800,
      key: "D",
    },
    {
      id: "text/regex",
      name: "Regex Tester",
      icon: "nf-fa-code",
      width: 900,
      height: 700,
      key: "R",
    },
    {
      id: "text/extractor",
      name: "Extractor",
      icon: "nf-fa-filter",
      width: 800,
      height: 900,
      key: "E",
    },
    {
      id: "text/strip",
      name: "Strip HTML Tags",
      icon: "nf-fa-eraser",
      width: 800,
      height: 800,
      key: "S",
    },
    {
      id: "text/string",
      name: "String Escaper / Unescaper",
      icon: "nf-md-format_text",
      width: 800,
      height: 600,
      // 'String' -> N (avoid S collision, or maybe G? User said N in my head? No wait user map: S=Strip. String=? User map didn't include String Escaper specifically in the prompt example list? Ah, wait, "S Strip HTML". String Escaper user didn't specify. I'll use 'A' or 'G' or 'T'. User list: D, R, E, S, U, J, C, M. String Escaper is missing from user list. I will pick 'G' (StrinG))
      // User list: D, R, E, S(Strip), U(Slug), J, C, M.
      // String Escaper is NOT in user's explicit list. I will assume 'G' for now.
      key: "N",
    },
    {
      id: "text/slug",
      name: "Slug Generator",
      icon: "nf-md-link_variant",
      width: 600,
      height: 400,
      key: "U",
    },
    {
      id: "text/jwt",
      name: "JWT Decoder",
      icon: "nf-md-key_variant",
      width: 900,
      height: 700,
      key: "J",
    },
    {
      id: "text/cron",
      name: "Cron Expression Explainer",
      icon: "nf-md-clock_outline",
      width: 700,
      height: 500,
      key: "C",
    },
    {
      id: "text/markdown",
      name: "Markdown Preview",
      icon: "nf-md-language_markdown",
      width: 1000,
      height: 800,
      key: "M",
    },
    {
      id: "text/lorem",
      name: "Lorem Ipsum Generator",
      icon: "nf-fa-paragraph",
      width: 700,
      height: 600,
      key: "L",
    },
    {
      id: "text/obfuscator",
      name: "Text Obfuscator / Rot13",
      icon: "nf-fa-user_secret",
      width: 700,
      height: 600,
      key: "O",
    },
    {
      id: "text/character",
      name: "Character Inspector",
      icon: "nf-fa-info_circle",
      width: 600,
      height: 600,
      key: "H",
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
