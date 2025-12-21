<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    interface Options {
        email: boolean;
        url: boolean;
        ip: boolean;
        hashtag: boolean;
        log_error: boolean;
    }

    let current_text: string;
    let result: [] = [];

    let options: Options = {
        email: false,
        url: false,
        ip: false,
        hashtag: false,
        log_error: false,
    };

    async function runExtractor() {
        if (!current_text) {
            return (result = []);
        }

        try {
            result = await invoke("process_extractor", {
                current_text: current_text,
                options: options,
            });
        } catch (error) {
            console.error("Extractor Hatasi:", error);
        }
    }

    $: if (options) {
        runExtractor();
    }
</script>

<main class="extractor-container">
    <div class="pane input-pane">
        <div class="pane-header">SOURCE TEXT</div>
        <textarea
            class="editor"
            placeholder="Karmaşık metni buraya yapıştır..."
            spellcheck="false"
            bind:value={current_text}
        ></textarea>
    </div>

    <div class="divider"></div>

    <div class="pane output-pane">
        <div class="controls">
            <div class="filter-label">EXTRACT:</div>

            <label class="toggle-chip" class:active={options.email}>
                <input type="checkbox" bind:checked={options.email} />
                Emails
            </label>

            <label class="toggle-chip" class:active={options.url}>
                <input type="checkbox" bind:checked={options.url} />
                URLs
            </label>

            <label class="toggle-chip" class:active={options.ip}>
                <input type="checkbox" bind:checked={options.ip} />
                IPs
            </label>

            <label class="toggle-chip" class:active={options.hashtag}>
                <input type="checkbox" bind:checked={options.hashtag} />
                #Tags
            </label>
            <label class="toggle-chip" class:active={options.log_error}>
                <input type="checkbox" bind:checked={options.log_error} />
                Errors (Log)
            </label>
        </div>

        <div class="pane-header result-header">
            <span>RESULTS</span>

            <button class="action-btn"> Copy All </button>
        </div>

        <div class="results-list">
            {#each result as item, index}
                <div class="result-card">
                    <div class="card-header">
                        <span class="index-badge">#{index + 1}</span>

                        <button
                            class="copy-icon-btn"
                            title="Copy"
                            on:click={() => navigator.clipboard.writeText(item)}
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="14"
                                height="14"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                                <rect
                                    x="9"
                                    y="9"
                                    width="13"
                                    height="13"
                                    rx="2"
                                    ry="2"
                                ></rect>
                                <path
                                    d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"
                                ></path>
                            </svg>
                        </button>
                    </div>

                    <div class="card-content">{item}</div>
                </div>
            {/each}
            {#if result.length === 0 && current_text}
                <div class="empty-state">No matches found.</div>
            {/if}
        </div>
    </div>
</main>

<style>
    /* --- GENEL DÜZEN --- */
    .extractor-container {
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

    /* --- BAŞLIKLAR --- */
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
        flex-shrink: 0; /* Başlık asla küçülmesin */
    }

    /* --- INPUT ALANI --- */
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

    /* --- KONTROLLER (CHIPS) - ESKİ HALİNE DÖNDÜ --- */
    .controls {
        padding: 12px;
        background-color: var(--bg-input);
        border-bottom: 1px solid var(--border-color);
        
        display: flex;
        align-items: center;
        gap: 8px;
        
        /* 1. Scroll yerine aşağı kayma (Wrap) geri geldi */
        flex-wrap: wrap; 
        
        /* Scroll kapalı */
        overflow: visible; 
        
        /* Alan daralırsa butonlar sığmazsa aşağı insin */
        flex-shrink: 0; 
    }

    .filter-label {
        font-size: 0.7rem;
        color: var(--text-muted);
        font-weight: bold;
        margin-right: 5px;
    }

    .toggle-chip {
        display: inline-flex;
        align-items: center;
        justify-content: center;
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

    .toggle-chip.active {
        background-color: var(--accent);
        border-color: var(--accent);
        color: var(--bg-app);
        font-weight: 600;
    }

    /* --- SONUÇ LİSTESİ --- */
    .results-list {
        flex: 1; /* Kalan boşluğu doldur */
        
        /* Sadece dikey scroll olsun */
        overflow-y: auto; 
        padding: 15px;
        background-color: var(--bg-app);
        display: flex;
        flex-direction: column;
        gap: 10px;
    }

    /* --- RESULT CARD (EN ÖNEMLİ KISIM) --- */
    .result-card {
        background-color: var(--bg-input);
        border: 1px solid var(--border-color);
        border-radius: 6px;
        display: flex;
        flex-direction: column;
        
        /* BU SATIR SORUNU ÇÖZER: */
        /* Flex container içinde sıkışmayı (squish) engeller */
        flex-shrink: 0; 
        
        /* İçerik taşarsa kartın içinde kalsın */
        overflow: hidden; 
    }

    .card-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 5px 10px;
        background-color: rgba(0, 0, 0, 0.1);
        border-bottom: 1px solid var(--border-color);
    }

    .index-badge {
        font-size: 0.7rem;
        font-weight: bold;
        color: var(--text-muted);
    }

    .card-content {
        padding: 10px;
        font-size: 0.9rem;
        color: var(--text-main);
        word-break: break-all;
        font-family: "Consolas", monospace;
    }

    /* --- BUTONLAR --- */
    .copy-icon-btn {
        background: transparent;
        border: none;
        color: var(--text-muted);
        cursor: pointer;
        padding: 4px;
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: 4px;
        transition: all 0.2s;
    }

    .copy-icon-btn:hover {
        color: var(--accent);
        background-color: var(--bg-app);
    }

    .action-btn {
        background: transparent;
        border: 1px solid var(--border-color);
        color: var(--text-main);
        padding: 3px 8px;
        border-radius: 4px;
        cursor: pointer;
        font-size: 0.7rem;
        transition: background 0.2s;
    }

    .action-btn:hover {
        background: var(--bg-input);
        border-color: var(--text-muted);
    }

    .empty-state {
        text-align: center;
        color: var(--text-muted);
        font-style: italic;
        margin-top: 20px;
    }
</style>
