<script lang="ts">
    import "../themes/catpuccin.css";
    import "../themes/nordic.css";
    import "../themes/dracula.css";
    import "../themes/gruvbox.css";
    import "../themes/carbonfox.css";
    import "../themes/icons.css";
    import { onMount } from "svelte";

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

<main class="main-box">
    <div class="text-tools"><i class="nf-seti-text"></i> Text Tools</div>

</main>

<style>
    @font-face {
        font-family: "NerdFonts";
        src: url("../assets/fonts/HurmitNerdFont-Regular.otf");
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

    .main-box {
        box-sizing: border-box;
        width: 100%;
        height: 100%;
        display: flex;
        flex-wrap: wrap;
        gap: 2%;
        overflow: hidden;
        background-color: transparent;
    }
    div {
        box-sizing: border-box;
        width: calc((100% - 6%) / 4);
        height: auto;
        aspect-ratio: 1/1;
        background-color: var(--bg-app);
        color: var(--text-main);
        text-align: center;
        font-size: 1.5rem;
    }
    i[class^="nf-"], i[class*=" nf-"] {
    font-style: normal !important;
}

</style>
