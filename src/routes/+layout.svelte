<script lang="ts">
    import "../themes/catpuccin.css";
    import "../themes/nordic.css";
    import "../themes/dracula.css";
    import "../themes/gruvbox.css";
    import "../themes/carbonfox.css";
    import "../themes/icons.css";
    import "../app.css";
    import { onMount } from "svelte";
     

    function setTheme(themeName: string) {
        document.documentElement.setAttribute("data-theme", themeName);
    }


    onMount(() => {
        setTheme("carbonfox-dark");
    });
</script>



<slot />
