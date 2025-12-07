<script lang="ts">
    import { onMount } from "svelte";
    import "../themes/catpuccin.css";
    import "../themes/nordic.css";
    import "../themes/dracula.css";
    import "../themes/gruvbox.css";
    import "../themes/carbonfox.css";

    let currentTheme = "";
    function setTheme(themeName: string) {
        document.documentElement.setAttribute("data-theme", themeName);
        localStorage.setItem("selected-theme", themeName);
    }
    $: if (currentTheme) setTheme(currentTheme);

    onMount(() => {
        const savedTheme = localStorage.getItem("selected-theme") || "nordic";
        setTheme(savedTheme);
    });
</script>

<main class="main">
    <div class="main-box">
        <div class="search-box">
            <input class="search-input" type="search" placeholder="Search..." />
        </div>
        <div class="suggestion-box"></div>
        <div class="options-box">
            <label for="themes">Themes:</label>
            <select name="themes" id="themes" bind:value={currentTheme}>
                <option value="catppuccin-dark">Catpuccin Dark</option>
                <option value="nordic-dark">Nordic Dark</option>
                <option value="dracula-dark">Dracula Dark</option>
                <option value="carbonfox-dark">Carbonfox Dark</option>
                <option value="gruvbox-dark">Gruvbox Dark</option>
                <option value="catppuccin-light">Catpuccin Light</option>
                <option value="nordic-light">Nordic Light</option>
                <option value="dracula-light">Dracula Light</option>
                <option value="carbonfox-light">Carbonfox Light</option>
                <option value="gruvbox-light">Gruvbox Light</option>
            </select>
        </div>
    </div>
</main>

<style>
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
    }

    .main {
        box-sizing: border-box;
        width: 100%;
        height: 100%;
        display: flex;
    }

    .main-box {
        box-sizing: border-box;
        width: 100%;
        height: 100%;

        display: flex;
        flex-direction: column;

        background-color: var(--bg-app);
        color: var(--text-main);

        border-radius: 12px;
        overflow: hidden;
        border: 1px solid var(--border-color);

        box-shadow:
            0 0 0 1px rgba(255, 255, 255, 0.05),
            0 20px 50px rgba(0, 0, 0, 0.6);
    }

    .search-box {
        box-sizing: border-box;
        width: 100%;
        height: 18%;

        background-color: var(--bg-input);
        border-bottom: 1px solid var(--border-color);

        display: flex;
        align-items: center;
        padding: 0 15px;
    }

    .search-input {
        width: 100%;
        height: 100%;

        background: transparent;
        border: none;
        outline: none;

        font-size: 1.5rem;
        color: var(--text-main);
    }

    .search-input::placeholder {
        color: var(--text-muted);
    }

    .suggestion-box {
        box-sizing: border-box;
        width: 100%;
        height: 70%;

        background-color: var(--bg-suggestion);
    }

    .options-box {
        box-sizing: border-box;
        width: 100%;
        height: 12%;

        background-color: var(--bg-options);
        color: var(--options-text);
    }
</style>
