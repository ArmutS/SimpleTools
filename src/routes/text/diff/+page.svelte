<script lang="ts">
    let firstText = "";
    let secondText = "";
</script>

<main class="main">
    <div class="diff-container">
        <div class="pane">
            <div class="pane-header">
                <span class="dot red"></span>
                <span class="title">FIRST TEXT</span>
            </div>
            <textarea
                class="editor"
                bind:value={firstText}
                placeholder="Buraya orijinal metni yapıştır..."
                spellcheck="false"
            ></textarea>
        </div>

        <div class="divider"></div>

        <div class="pane">
            <div class="pane-header">
                <span class="dot green"></span>
                <span class="title">SECOND TEXT</span>
            </div>
            <textarea
                class="editor"
                bind:value={secondText}
                placeholder="Buraya yeni metni yapıştır..."
                spellcheck="false"
            ></textarea>
        </div>
    </div>
</main>

<style>
    .diff-container {
        width: 100%;
        height: 100%;
        display: flex;
        gap: 2px;
        background-color: var(--border-color, #333); /* Çerçeve rengi */
    }

    .pane {
        flex: 1;
        display: flex;
        flex-direction: column;
        background-color: var(--bg-app);
        min-width: 0;
    }

    .pane-header {
        height: 40px;
        background-color: var(--bg-input);
        border-bottom: 1px solid var(--border-color);
        display: flex;
        align-items: center;
        padding: 0 15px;
        gap: 10px;
        user-select: none;
    }

    .title {
        font-family: "HurmitNerd", monospace;
        font-size: 0.85rem;
        font-weight: bold;
        color: var(--text-muted);
        letter-spacing: 1px;
    }

    .dot {
        width: 8px;
        height: 8px;
        border-radius: 50%;
    }
    .dot.red {
        background-color: #bf616a;
        box-shadow: 0 0 5px rgba(191, 97, 106, 0.4);
    }
    .dot.green {
        background-color: #a3be8c;
        box-shadow: 0 0 5px rgba(163, 190, 140, 0.4);
    }

    .editor {
        flex: 1;
        width: 100%;
        background-color: transparent;
        color: var(--text-main);
        border: none;
        resize: none;
        padding: 15px;
        font-family: "HurmitNerd", monospace;
        font-size: 0.95rem;
        line-height: 1.6;
        outline: none;
    }

    .editor::-webkit-scrollbar {
        width: 8px;
    }
    .editor::-webkit-scrollbar-track {
        background: transparent;
    }
    .editor::-webkit-scrollbar-thumb {
        background-color: var(--border-color);
        border-radius: 4px;
    }
    .editor::-webkit-scrollbar-thumb:hover {
        background-color: var(--text-muted);
    }

    .editor::placeholder {
        color: var(--border-color);
        opacity: 0.5;
    }
</style>
