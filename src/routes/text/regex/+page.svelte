<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    interface MatchResult {
        text: string;
        start: number;
        end: number;
    }

    let regexText = "";
    let regexFlags = "";
    let regexInput = "";
    let results: MatchResult[] = [];

    async function runRegex() {
        if (!regexInput) {
            results = [];
            return;
        }

        try {
            results = await invoke("process_text_reg", {
                current_text: regexText,
                current_regex: regexInput,
                current_flags: regexFlags,
            });
        } catch (error) {
            results = [];
            console.error("Regex Hatası:", error);
        }
    }
    $: if (regexFlags || regexInput || regexText) {
        runRegex();
    }
</script>

<main class="regex-container">
    <div class="control-panel">
        <div class="input-label">Regular Expression</div>
        <div class="regex-bar">
            <span class="slash">/</span>
            <input
                type="text"
                class="pattern-input"
                placeholder="expression..."
                bind:value={regexInput}
                spellcheck="false"
            />
            <span class="slash">/</span>
            <input
                type="text"
                class="flags-input"
                placeholder="sim"
                bind:value={regexFlags}
                spellcheck="false"
            />
        </div>
    </div>

    <div class="editor-section">
        <div class="input-label">Test String</div>
        <textarea
            class="test-area"
            placeholder="Metnini buraya yapıştır..."
            bind:value={regexText}
            spellcheck="false"
        ></textarea>
    </div>

    <div class="results-section">
        <div class="section-header">
            <span>MATCHES</span>
            <span class="match-count">{results.length} matches</span>
        </div>

        <div class="matches-container">
            {#each results as match, index}
                <div class="match-card">
                    <div class="match-header">
                        <span class="match-index">#{index + 1}</span>
                        <span class="match-range"
                            >Indices: [{match.start}-{match.end}]</span
                        >
                    </div>
                    <div class="match-content">
                        <span class="highlight">{match.text}</span>
                    </div>
                </div>
            {/each}

            {#if results.length === 0 && regexInput}
                <div
                    style="padding:10px; color:var(--text-muted); font-style:italic;"
                >
                    Eşleşme bulunamadı...
                </div>
            {/if}
        </div>
    </div>
</main>

<style>
    /* --- LAYOUT --- */
    .regex-container {
        height: 100vh;
        display: flex;
        flex-direction: column;
        padding: 20px;
        gap: 20px;
        background-color: var(--bg-app);
        color: var(--text-main);
        font-family: "Consolas", monospace;
    }

    .input-label {
        font-size: 0.75rem;
        color: var(--text-muted);
        margin-bottom: 8px;
        font-weight: bold;
        text-transform: uppercase;
    }

    /* --- REGEX BAR --- */
    .regex-bar {
        display: flex;
        align-items: center;
        background-color: var(--bg-input);
        border: 1px solid var(--border-color);
        border-radius: 8px;
        padding: 0 10px;
        height: 45px;
        font-size: 1.1rem;
    }

    .slash {
        color: var(--text-muted);
        font-weight: bold;
        user-select: none;
    }

    .pattern-input {
        flex: 1; /* Kalan alanı kapla */
        background: transparent;
        border: none;
        color: var(--accent); /* Regex deseni renkli olsun */
        padding: 0 10px;
        font-family: inherit;
        font-size: inherit;
        outline: none;
    }

    .flags-input {
        width: 60px;
        background: transparent;
        border: none;
        color: var(--diff-add-text); /* Flagler yeşilimsi olsun */
        padding: 0 5px;
        font-family: inherit;
        font-size: inherit;
        outline: none;
        font-style: italic;
    }

    /* --- TEST AREA --- */
    .editor-section {
        flex: 1; /* Esnek alan */
        display: flex;
        flex-direction: column;
        min-height: 150px;
    }

    .test-area {
        flex: 1;
        background-color: var(--bg-input);
        border: 1px solid var(--border-color);
        border-radius: 8px;
        padding: 15px;
        color: var(--text-main);
        font-family: inherit;
        font-size: 0.9rem;
        resize: none;
        outline: none;
        line-height: 1.6;
    }

    .test-area:focus {
        border-color: var(--accent);
    }

    /* --- RESULTS SECTION --- */
    .results-section {
        flex: 1;
        display: flex;
        flex-direction: column;
        background-color: var(--bg-header); /* Biraz daha koyu zemin */
        border: 1px solid var(--border-color);
        border-radius: 8px;
        overflow: hidden;
    }

    .section-header {
        padding: 10px 15px;
        background-color: var(--bg-input);
        border-bottom: 1px solid var(--border-color);
        display: flex;
        justify-content: space-between;
        font-size: 0.8rem;
        font-weight: bold;
        color: var(--text-muted);
    }

    .match-count {
        color: var(--accent);
    }

    .matches-container {
        flex: 1;
        overflow-y: auto;
        padding: 10px;
        display: flex;
        flex-direction: column;
        gap: 10px;
    }

    /* --- MATCH CARDS --- */
    .match-card {
        background-color: var(--bg-app);
        border: 1px solid var(--border-color);
        border-radius: 6px;
        padding: 10px;
        display: flex;
        flex-direction: column;
        gap: 5px;
    }

    .match-header {
        display: flex;
        justify-content: space-between;
        font-size: 0.75rem;
        color: var(--text-muted);
        border-bottom: 1px solid var(--border-color);
        padding-bottom: 5px;
        margin-bottom: 2px;
    }

    .match-index {
        color: var(--diff-add-text); /* Yeşilimsi */
        font-weight: bold;
    }

    .match-content {
        font-size: 0.95rem;
        color: var(--text-main);
        white-space: pre-wrap;
        word-break: break-all;
    }

    .highlight {
        background-color: var(--diff-add-bg);
        color: var(--diff-add-text);
        padding: 2px 4px;
        border-radius: 4px;
        font-weight: bold;
    }
</style>
