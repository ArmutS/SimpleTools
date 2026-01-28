<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { onMount } from "svelte";
  import * as mammoth from "@bagiit/mammoth";
  import html2pdf from "html2pdf.js";
  import { emit } from "@tauri-apps/api/event";

  let status = "Idle";

  onMount(async () => {
    // Broadcast ready state immediately in case we're late
    await emit("renderer-ready");

    // Also listen for pings from the main window (handshake)
    const unlistenPing = await WebviewWindow.getCurrent().listen(
      "ping-worker",
      async () => {
        console.log("Ping received, sending pong (renderer-ready)");
        await emit("renderer-ready");
      },
    );

    // Listen for conversion jobs
    const unlisten = await WebviewWindow.getCurrent().listen<{
      filePath: string;
      outputDir: string;
      callerLabel: string;
    }>("start-conversion", async (event) => {
      const { filePath, outputDir, callerLabel } = event.payload;
      await processDocxToPdf(filePath, outputDir, callerLabel);
    });

    /*
      Correctly handle cleanup.
      Wait for unlisten to be ready, but onMount must return sync or Promise<void>.
      See Svelte 5 / Next breaking changes or just return the function.
    */
  });

  async function processDocxToPdf(
    filePath: string,
    outputDir: string,
    callerLabel: string,
  ) {
    try {
      const fileName =
        filePath.split(/[/\\]/).pop()?.split(".")[0] || "converted";
      const targetPath = `${outputDir}/${fileName}.pdf`;

      status = `Reading ${fileName}...`;
      await WebviewWindow.getCurrent().emitTo(
        callerLabel,
        "conversion-status",
        {
          status: status,
        },
      );

      // 1. Rust: Read DOCX Binary
      const fileBytes = await invoke<number[]>("read_docx_binary", {
        path: filePath,
      });

      status = "Rendering HTML...";
      await WebviewWindow.getCurrent().emitTo(
        callerLabel,
        "conversion-status",
        {
          status: status,
        },
      );

      const arrayBuffer = new Uint8Array(fileBytes).buffer;
      const result = await mammoth.convertToHtml({ arrayBuffer: arrayBuffer });

      // 2. Prepare HTML
      const element = document.createElement("div");
      element.innerHTML = result.value;
      element.style.width = "210mm";
      element.style.padding = "20mm";
      element.style.background = "white";
      element.style.color = "black";
      element.style.fontSize = "12pt";
      element.style.fontFamily = "Times New Roman";

      status = "Generating PDF (Heavy Task)...";
      await WebviewWindow.getCurrent().emitTo(
        callerLabel,
        "conversion-status",
        {
          status: status,
        },
      );

      // 3. html2pdf (Heavy Blocking Task)
      const opt = {
        margin: 0,
        filename: "myfile.pdf",
        image: { type: "jpeg", quality: 0.98 } as any,
        html2canvas: { scale: 2 },
        jsPDF: { unit: "mm", format: "a4", orientation: "portrait" } as any,
      };

      const pdfArrayBuffer = await html2pdf()
        .set(opt)
        .from(element)
        .output("arraybuffer");

      status = "Saving to disk...";
      await WebviewWindow.getCurrent().emitTo(
        callerLabel,
        "conversion-status",
        {
          status: status,
        },
      );

      // 4. Save via Rust
      const pdfBytes = Array.from(new Uint8Array(pdfArrayBuffer));
      await invoke("save_binary_file", {
        path: targetPath,
        data: pdfBytes,
      });

      // Success
      await WebviewWindow.getCurrent().emitTo(
        callerLabel,
        "conversion-complete",
        {
          path: targetPath,
        },
      );
    } catch (error) {
      console.error(error);
      await WebviewWindow.getCurrent().emitTo(callerLabel, "conversion-error", {
        error: String(error),
      });
    }
  }
</script>

<div style="padding: 20px; font-family: sans-serif;">
  <h1>Background Renderer</h1>
  <p>Status: {status}</p>
</div>
