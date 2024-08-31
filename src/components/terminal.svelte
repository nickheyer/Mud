<script>
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import _ from "lodash";

    let inputHistory = [];
    let historyIndex = -1;
    let terminalOutput = "";
    let terminalCodeBlock = null;
    let terminalDiv = null;
    $: currentInput = "";
    $: fullTerminalContent =
        terminalOutput + (currentInput ? `$ ${currentInput}` : "");

    async function runCommand(command) {
        try {
            const output = await invoke("run_script", {
                scriptContent: command,
            });
            terminalOutput += `> ${command}\n${_.trim(output, '"').replace(/\\n/g, "\n")}\n`;
        } catch (error) {
            console.error("Error running script:", error);
            terminalOutput += `Error: ${JSON.stringify(error, null, 2)}\n`;
        } finally {
            currentInput = "";
            historyIndex = inputHistory.length; // Reset the history index after execution
        }
    }

    async function scrollDown() {
        await terminalDiv.scrollIntoView({ behavior: "smooth", block: "end" });
    }

    async function handleKeyPress(event) {
        if (!event.shiftKey) {
            switch (event.key) {
                case "Enter":
                    event.preventDefault();
                    inputHistory.push(currentInput);
                    historyIndex = inputHistory.length;
                    if (currentInput === "clear") {
                        terminalOutput = "";
                        currentInput = "";
                    } else {
                        await runCommand(currentInput);
                    }
                    await scrollDown();
                    break;

                default:
                    break;
            }
        } else {
            switch (event.key) {
                case "ArrowUp":
                    event.preventDefault();
                    if (historyIndex > 0) {
                        historyIndex--;
                        currentInput = inputHistory[historyIndex];
                    }
                    break;

                case "ArrowDown":
                    event.preventDefault();
                    if (historyIndex < inputHistory.length - 1) {
                        historyIndex++;
                        currentInput = inputHistory[historyIndex];
                    } else {
                        historyIndex = inputHistory.length;
                        currentInput = "";
                    }
                    break;

                default:
                    break;
            }
        }
    }


    function focusInput() {
        terminalDiv.focus();
    }

    onMount(() => {
        terminalDiv = document.getElementById("term-input");
        focusInput();
    });
</script>

<link rel="stylesheet" href="/css/terminal.css">

<div class="repl-container">
    <img
        src="/images/Mud_512x218_txt_blk.svg"
        alt="Mud Text Logo"
        class="mud-overlay"
    />
    <div class="repl-interface">
        <pre><code id="term-code" class="language-bash"
                >{fullTerminalContent}</code
            ></pre>
        <textarea
            id="term-input"
            class="term-input"
            bind:value={currentInput}
            on:keydown={handleKeyPress}
        ></textarea>
    </div>
</div>

