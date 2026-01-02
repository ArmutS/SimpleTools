<script lang="ts">
  import "../themes/catpuccin.css";
  import "../themes/nordic.css";
  import "../themes/dracula.css";
  import "../themes/gruvbox.css";
  import "../themes/carbonfox.css";
  import "../app.css";
  import { onMount } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  function setTheme(themeName: string) {
    document.documentElement.setAttribute("data-theme", themeName);
  }

  async function closeWindow() {
    try {
      const currentwindow = getCurrentWindow();
      await currentwindow.close();
      console.log("closeWindow un try icine girildi");
    } catch (error) {
      console.error("closeWindow catch icine girildi ", error);
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Escape") {
      closeWindow();
    }
  }

  onMount(() => {
    setTheme("catppuccin-dark");
  });
</script>

<svelte:window onkeydown={handleKeydown} />

<link
  rel="stylesheet"
  href="https://www.nerdfonts.com/assets/css/webfont.css"
/>

<slot />
