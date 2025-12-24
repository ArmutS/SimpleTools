<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    let pure_text: boolean = true;
    let current_text: string = "";
    let result = "";

    async function runStrip(text: string, is_pure: boolean) {
        try {
            result = await invoke("process_strip", {
                current_text: text,
                pure_text: is_pure,
            });
        } catch (error) {
            console.error("Strip hatasi:", error);
        }
    }

    $: runStrip(current_text, pure_text);
</script>

<main class="stripper-container">
    <div class="pane input-pane">
        <div class="pane-header">HTML INPUT</div>
        <textarea
            class="editor source-editor"
            placeholder="HTML kodunu buraya yapıştır..."
            spellcheck="false"
            bind:value={current_text}
        ></textarea>
    </div>

    <div class="divider"></div>

    <div class="pane output-pane">
        <div class="controls">
            <div class="filter-label">OPTIONS:</div>

            <label class="toggle-chip" class:active={pure_text}>
                <input type="checkbox" bind:checked={pure_text} />
                Decode Entities (&amp; &rarr; &)
            </label>
        </div>

        <div class="pane-header result-header">
            <span>PLAIN TEXT</span>
            <button class="action-btn"> Copy Result </button>
        </div>

        <textarea
            class="editor result-editor"
            placeholder="Temizlenen metin burada görünecek..."
            readonly
            bind:value={result}
        ></textarea>
    </div>
</main>

<style>
    /* --- GENEL LAYOUT --- */
    .stripper-container {
        height: 100vh;
        display: flex;
        background-color: var(--bg-app);
        color: var(--text-main);
        font-family: "Consolas", monospace;
        overflow: hidden;
    }

    .pane {
        flex: 1;
        display: flex;
        flex-direction: column;
        min-width: 0;
    }

    .divider {
        width: 1px;
        background: var(--border-color);
        opacity: 0.5;
    }

    /* --- HEADER & TOOLBAR --- */
    .pane-header {
        padding: 8px 12px;
        background: var(--bg-header);
        border-bottom: 1px solid var(--border-color);
        font-size: 0.75rem;
        font-weight: bold;
        color: var(--text-muted);
        display: flex;
        justify-content: space-between;
        align-items: center;
        user-select: none;
        flex-shrink: 0;
    }

    .controls {
        padding: 12px;
        background-color: var(--bg-input);
        border-bottom: 1px solid var(--border-color);
        display: flex;
        align-items: center;
        gap: 8px;
        flex-wrap: wrap;
        flex-shrink: 0;
    }

    /* --- EDİTÖRLER (TEXTAREA) --- */
    .editor {
        flex: 1;
        background: transparent;
        border: none;
        color: var(--text-main);
        padding: 15px;
        resize: none;
        outline: none;
        font-family: inherit;
        font-size: 0.9rem;
        line-height: 1.6;
    }

    .source-editor {
        background-color: rgba(0, 0, 0, 0.1);
    }

    .result-editor {
        color: var(--accent);
    }

    /* --- CHIP BUTTONS --- */
    .filter-label {
        font-size: 0.7rem;
        color: var(--text-muted);
        font-weight: bold;
        margin-right: 5px;
    }

    .toggle-chip {
        display: inline-flex;
        align-items: center;
        padding: 4px 10px;
        border-radius: 12px;
        border: 1px solid var(--border-color);
        background: var(--bg-app);
        color: var(--text-muted);
        font-size: 0.8rem;
        cursor: pointer;
        transition: all 0.2s ease;
        user-select: none;
    }

    .toggle-chip:hover {
        border-color: var(--text-muted);
        color: var(--text-main);
    }

    .toggle-chip input {
        display: none;
    }

    /* Svelte dinamik class:active kullanımı ile burası tetiklenir */
    .toggle-chip.active {
        background-color: var(--accent);
        border-color: var(--accent);
        color: var(--bg-app);
        font-weight: 600;
    }

    .action-btn {
        background: transparent;
        border: 1px solid var(--border-color);
        color: var(--text-main);
        padding: 4px 10px;
        border-radius: 4px;
        cursor: pointer;
        font-size: 0.75rem;
        font-family: inherit;
        transition: background 0.2s;
    }

    .action-btn:hover {
        background: var(--bg-input);
        border-color: var(--text-muted);
        color: var(--accent);
    }

    .action-btn:active {
        transform: translateY(1px);
    }
</style>
