<script lang="ts">
    import "../themes/catpuccin.css";
    import "../themes/nordic.css";
    import "../themes/dracula.css";
    import "../themes/gruvbox.css";
    import "../themes/carbonfox.css";
    import "../themes/icons.css";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    let tools = [
        { id: "text", name: "Text Tools", icon: "nf-seti-text" },
        { id: "pdf", name: "PDF Tools", icon: "nf-seti-pdf" },
        {
            id: "convert",
            name: "Converters",
            icon: "nf-md-file_arrow_left_right_outline",
        },
        { id: "file", name: "File & System", icon: "nf-oct-file" },
        { id: "image", name: "Image Tools", icon: "nf-fa-image" },
        { id: "network", name: "Network", icon: "nf-md-console_network" },
        { id: "dev", name: "Dev Tools", icon: "nf-dev-devicon" },
        { id: "quickcmd", name: "Quick Cmds", icon: "nf-oct-command_palette" },
        { id: "9", name: "Coming Soon", icon: "nf-fae-comet" },
        { id: "10", name: "Coming Soon", icon: "nf-fae-comet" },
        { id: "11", name: "Coming Soon", icon: "nf-fae-comet" },
        { id: "12", name: "Coming Soon", icon: "nf-fae-comet" },
    ];

    async function create_window(id: String, title: String) {
        await invoke("create_new_window", { id: id, title: title});
    }

    // let currentTheme = localStorage.getItem("selected-theme") || "nordic-dark";
    function setTheme(themeName: string) {
        document.documentElement.setAttribute("data-theme", themeName);
        // localStorage.setItem("selected-theme", themeName);
    }
    // $: if (currentTheme) setTheme(currentTheme);
    onMount(() => {
        setTheme("carbonfox-dark");
    });
</script>

<main class="main">
    <div class="tools-box">
        {#each tools as tool}
            <button
                class="tools-buttons"
                on:click={() => create_window(tool.id,tool.name)}
            >
                <i class="{tool.icon} tool-icon"></i>

                <span class="tool-name">{tool.name}</span>
            </button>
        {/each}
    </div>
</main>

<style>
    @font-face {
        font-family: "NerdFonts";
        src: url("/fonts/HurmitNerdFont-Regular.otf");
        font-weight: normal;
        font-style: normal;
    }

    :global(html),
    :global(body) {
        margin: 0;
        padding: 0;
        width: 100%;
        height: 100%;
        background-color: transparent !important;
        display: flex;
        justify-content: center;
        align-items: center;
        overflow: hidden;
        font-family: "NerdFonts";
    }

    .main {
        box-sizing: border-box;
        width: 100%;
        height: 100%;
        display: flex;
        justify-content: center;
        overflow: hidden;
        background-color: transparent;
    }

    .tools-box {
        display: flex;
        flex-wrap: wrap;
        gap: 2%;
        box-sizing: border-box;
        width: 100%;
        height: 100%;
        background-color: transparent;
    }
    i[class^="nf-"],
    i[class*=" nf-"] {
        font-style: normal !important;
    }

    .tools-buttons {
        width: calc((100% - 6%) / 4);
        height: auto;
        background-color: var(--bg-input, #333);
        color: var(--text-main, #fff);
        border: 1px solid var(--border-color, #555);
        border-radius: 8px;
        cursor: pointer;
        transition: all 0.2s;

        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        gap: 15px;
    }

    .tool-icon {
        font-size: 2.5rem;
        color: var(--bg-options);
        margin-bottom: 5px;
    }

    .tool-name {
        font-family: "NerdFonts";
        font-size: 1.1rem;
        font-weight: bold;
    }

    .tools-buttons:hover {
        background-color: var(--bg-options);
        color: var(--options-text);
        transform: translateY(-3px);
    }

    .tools-buttons:hover .tool-icon {
        color: var(--options-text);
    }
</style>
