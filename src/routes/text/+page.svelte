<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    let tools = [
        { id: "text/diff", name: "Diff Viewer", icon: "nf-seti-text" },
        { id: "text/regex", name: "Regex Tester", icon: "nf-seti-pdf" },
        {
            id: "text/extractor",
            name: "Extractor",
            icon: "nf-md-file_arrow_left_right_outline",
        },
        { id: "text/strip", name: "Strip HTML Tags", icon: "nf-oct-file" },
        {
            id: "text/string",
            name: "String Escaper / Unescaper",
            icon: "nf-fa-image",
        },
        {
            id: "text/slug",
            name: "Slug Generator",
            icon: "nf-md-console_network",
        },
        { id: "text/jwt", name: "JWT Decoder", icon: "nf-dev-devicon" },
        {
            id: "text/cron",
            name: "Cron Expression Explainer",
            icon: "nf-oct-command_palette",
        },
        { id: "text/markdown", name: "Markdown Preview", icon: "nf-fae-comet" },
        {
            id: "text/lorem",
            name: "Lorem Ipsum Generator",
            icon: "nf-fae-comet",
        },
        {
            id: "text/obfuscator",
            name: "Text Obfuscator / Rot13",
            icon: "nf-fae-comet",
        },
        {
            id: "text/character",
            name: "Character Inspector",
            icon: "nf-fae-comet",
        },
    ];

    async function create_window(id: String, title: String) {
        await invoke("create_new_window", { id: id, title: title });
    }
</script>

<main class="main">
    <div class="tools-box">
        {#each tools as tool}
            <button
                class="tools-buttons"
                on:click={() => create_window(tool.id, tool.name)}
            >
                <i class="{tool.icon} tool-icon"></i>

                <span class="tool-name">{tool.name}</span>
            </button>
        {/each}
    </div>
</main>

<style>
</style>
