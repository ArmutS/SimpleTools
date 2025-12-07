<script lang="ts">
    import "../themes/catpuccin.css";
    import "../themes/nordic.css";
    import "../themes/dracula.css";
    import "../themes/gruvbox.css";
    import "../themes/carbonfox.css";
    import { onMount } from "svelte";

    // let currentTheme = localStorage.getItem("selected-theme") || "nordic-dark";
    function setTheme(themeName: string) {
        document.documentElement.setAttribute("data-theme", themeName);
        // localStorage.setItem("selected-theme", themeName);
    }
    // $: if (currentTheme) setTheme(currentTheme);
    onMount (() => {
        setTheme("carbonfox-dark");
        })
</script>

<main class="main">

<div class="main-box">

<div class="pdf"></div>
<div class="a"></div>
<div class="b"></div>

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

        display: grid;
        gap: 2%;

        background-color: var(--bg-app);
        color: var(--text-main);

        overflow: hidden;
        border: 1px solid var(--border-color);
    }
    div{
        box-sizing: border-box;
        width: auto;
        height: auto;
        background-color: red;
        border-color: black;
        }

    .a{background-color: blue;}
    .b{background-color: aqua;}
</style>
