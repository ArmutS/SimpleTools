<script lang="ts">
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
        { id: "soon", name: "Coming Soon", icon: "nf-fae-comet" },
        { id: "soon", name: "Coming Soon", icon: "nf-fae-comet" },
        { id: "soon", name: "Coming Soon", icon: "nf-fae-comet" },
        { id: "soon", name: "Coming Soon", icon: "nf-fae-comet" },
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
