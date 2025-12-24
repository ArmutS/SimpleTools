<script lang="ts"></script>
<main class="stripper-container">
    
    <div class="pane input-pane">
        <div class="pane-header">HTML INPUT</div>
        <textarea 
            class="editor source-editor" 
            placeholder="HTML kodunu buraya yapıştır..." 
            spellcheck="false"
            
            ><div class="content">
    <h1>Merhaba Dünya &amp; Mars!</h1>
    <p>Bu bir <b>HTML</b> temizleme testidir.</p>
    <a href="#">Link</a> ve <span>Span</span> etiketleri silinecek.
</div></textarea>
    </div>

    <div class="divider"></div>

    <div class="pane output-pane">
        
        <div class="controls">
            <div class="filter-label">OPTIONS:</div>
            
            <label class="toggle-chip active">
                <input type="checkbox" checked /> 
                Decode Entities (&amp; &rarr; &)
            </label>
        </div>

        <div class="pane-header result-header">
            <span>PLAIN TEXT</span>
            
            <button class="action-btn">
                Copy Result
            </button>
        </div>

        <textarea 
            class="editor result-editor" 
            placeholder="Temizlenen metin burada görünecek..." 
            readonly
            
            >Merhaba Dünya & Mars!
Bu bir HTML temizleme testidir.
Link ve Span etiketleri silinecek.</textarea>

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
        min-width: 0; /* Flexbox taşma koruması */
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
        flex-shrink: 0; /* Header asla büzüşmesin */
    }

    .controls {
        padding: 12px;
        background-color: var(--bg-input);
        border-bottom: 1px solid var(--border-color);
        display: flex;
        align-items: center;
        gap: 8px;
        flex-wrap: wrap; /* Sığmazsa aşağı insin */
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

    /* Sol taraf biraz daha silik (Source Code Havası) */
    .source-editor {
        background-color: rgba(0, 0, 0, 0.1); 
    }

    /* Sağ taraf (Sonuç) daha net */
    .result-editor {
        color: var(--accent); /* Vurgu rengi ile sonuç */
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

    .toggle-chip.active {
        background-color: var(--accent);
        border-color: var(--accent);
        color: var(--bg-app);
        font-weight: 600;
    }

    /* --- ACTION BUTTON --- */
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
