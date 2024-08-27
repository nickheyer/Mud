<script>
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import _ from "lodash";

    let inputHistory = [];
    let historyIndex = -1;
    let terminalOutput = "";
    let terminalDiv = null;
    $: currentInput = "";
    $: fullTerminalContent = terminalOutput;

    // Function to run the script command
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
        }
  }

  async function scrollDown() {
    await terminalDiv.scrollIntoView({
      behavior: 'smooth',
      block: 'end'
    });
  }

  // Keypress event handler for the terminal
  async function handleKeyPress(event) {
    
    if (event.key === "Enter" && !event.shiftKey) {
      event.preventDefault();
      inputHistory.push(currentInput);
      historyIndex = inputHistory.length;
      if (currentInput === 'clear') {
        terminalOutput = '';
        currentInput = '';
      } else {
        await runCommand(currentInput);
      }
      await scrollDown();
    } else if (event.key === "ArrowUp") {
      event.preventDefault();
      if (historyIndex > 0) {
        historyIndex--;
        currentInput = inputHistory[historyIndex];
      }
    } else if (event.key === "ArrowDown") {
      event.preventDefault();
      if (historyIndex < inputHistory.length - 1) {
        historyIndex++;
        currentInput = inputHistory[historyIndex];
      } else {
        historyIndex = inputHistory.length;
        currentInput = "";
      }
    }
  }

  function focusInput() {
    terminalDiv.focus();
  }
  
  onMount(async () => {
    terminalDiv = document.getElementById("term-input");
    focusInput()
  });
  
</script>

<div class="container">
  

  <!-- Terminal Interface -->
  <div
    id="terminal"
    class="repl-interface"
    on:mouseover={focusInput}
    on:focus={null}
    on:blur={null}
    role="group"
  >
    <img src="/Mud_512x218_txt_blk.svg" alt="Mud Text Logo" class="mud-overlay" />
    <pre>{fullTerminalContent}</pre>
    <label for="term-input" style="display: none;">Input:</label>
    <input
      id="term-input"
      bind:value={currentInput}
      on:keydown={handleKeyPress}
      class="term-input"
      placeholder="Type your command..."
    />
  </div>
</div>

<style>
  .container {
    position: relative;
    margin-top: 2rem;
    text-align: start;
  }

  .mud-overlay {
    position: absolute;
    top: 1rem;
    right: 3rem;
    width: 10rem;
    opacity: 0.3;
    pointer-events: none;
  }

  .repl-interface {
    background-color: #333;
    color: #f6f6f6;
    padding: 20px;
    scroll-padding-bottom: 20px;
    border-radius: 8px;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
    font-family: monospace;
    white-space: pre-wrap;

    height: 85vh;
    display: flex;
    overflow-y: scroll;
    flex-direction: column;
    margin: 0 2rem 0 2rem;
  }

  .term-input {
    width: 100%;
    background-color: transparent;
    color: #f6f6f6;
    border: none;
    outline: none;
    font-family: monospace;
    padding: 0;
    margin-top: 5px;
    font-size: 1rem !important;
    line-height: 1.5 !important;
    resize: horizontal;
  }

  .term-input::placeholder {
    color: #bbb;
  }

  pre {
    margin: 0;
    font-size: 1rem;
    line-height: 1.5;
  }
</style>