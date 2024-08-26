<script>
    // import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    // import { exit } from "@tauri-apps/plugin-process";
    // import _ from 'lodash';
    // import { marked } from 'marked';

    // $: scriptInput = "";
    // $: scriptOutput = "";
  
    // async function runScript() {
    //   try {
    //     const scriptContent = `${scriptInput}`;
    //     const output = await invoke('run_script', { scriptContent });
    //     scriptOutput = _.trim(output, '"')
    //       .replace(/\\n/g, '\n');
    //   } catch (error) {
    //     scriptOutput = JSON.stringify(error, 4, 2);
    //     console.error("Error running script:", error);
    //   }
    // }
    
    import Terminal from "../../components/terminal.svelte";

    onMount(async () => {
      console.log('On repl page!');
    });

</script>


<Terminal />



<style>

/* .container {
  max-width: 800px;
  margin: 0 auto;
  padding: 2rem 1rem;
  text-align: center;
}

.logo {
  width: 150px;
  height: auto;
  margin-bottom: 20px;
}

.repl-interface {
  background-color: #333;
  padding: 20px;
  margin: 20px 0;
  border-radius: 8px;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
}

.repl-interface h2 {
  margin: 0 0 10px;
}

.repl {
  margin: auto;
  width: auto;
  min-width: 60%;
  padding: 10px;
  margin-bottom: 10px;
  border-radius: 5px;
  border: 1px solid #555;
  background-color: #222;
  color: #f6f6f6;
  resize: vertical;
}

.repl-output {
  white-space: pre-wrap;
} */

</style>

