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
  $: fullTerminalContent = terminalOutput + (currentInput ? `\n>$ ${currentInput}` : '');

  async function runCommand(command) {
      try {
          const output = await invoke("run_script", { scriptContent: command });
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
      await terminalDiv.scrollIntoView({ behavior: 'smooth', block: 'end' });
  }

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

  onMount(() => {
      terminalDiv = document.getElementById("term-input");
      focusInput();
  });

</script>
<div class="repl-container">
  <img src="/Mud_512x218_txt_blk.svg" alt="Mud Text Logo" class="mud-overlay" />
  <div class="repl-interface">
    <pre><code id="term-code" class="language-bash">{fullTerminalContent}</code></pre>
    <textarea id="term-input" class="term-input" bind:value={currentInput} on:keydown={handleKeyPress} rows="1"></textarea>
  </div>
</div>


<style>
  .repl-container {
    position: relative;
    padding: 20px;
  }

  .mud-overlay {
    position: absolute;
    top: 3rem;
    right: 3rem;
    width: 10rem;
    opacity: 0.3;
    pointer-events: none;
  }

  .repl-interface {
      background-color: #333;
      color: #f6f6f6;
      padding: 20px;
      padding-bottom: 0;
      scroll-padding: 20px;
      border-radius: 8px;
      box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
      font-family: monospace;
      white-space: pre-wrap;
      height: 85vh; /* Increased to allow space for the input */
      display: flex;
      overflow-y: scroll;
      flex-direction: column;
      margin: 1rem auto;
  }

  .term-input {
      width: 100%;
      background-color: transparent;
      color: #f6f6f6;
      border: none;
      outline: none;
      font-family: monospace;
      padding: 5px; /* Added some padding */
      margin-top: 5px;
      font-size: 1rem !important;
      line-height: 1.5 !important;
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