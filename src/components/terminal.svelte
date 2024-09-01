<script>
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import _ from "lodash";

    import { basicSetup } from "codemirror";
    import { EditorView, keymap } from "@codemirror/view";
    import { autocompletion } from "@codemirror/autocomplete";
    import { StreamLanguage } from "@codemirror/language";
    import { indentWithTab } from "@codemirror/commands";
    import { shell } from "@codemirror/legacy-modes/mode/shell";

    let inputHistory = [];
    let historyIndex = -1;
    let terminalOutput = "";
    let terminalContainer = null;
    let terminalEditor = null;
    $: currentInput = "";
    $: fullTerminalContent = terminalOutput;

    function createEditor(parent, theme) {
        return new EditorView({
            doc: "# Shift + Enter to run script!\n",
            extensions: [
                basicSetup,
                keymap.of([indentWithTab]),
                StreamLanguage.define(shell)
            ],
            parent,
        });
    }

    // const completions = [
    //     { label: "panic", type: "keyword" },
    //     { label: "park", type: "constant", info: "Test completion" },
    //     { label: "password", type: "variable" },
    // ];

    async function runCommand(command) {
        try {
            const output = await invoke("run_script", {
                scriptContent: command,
            });
            console.log(output);
        } catch (error) {
            console.error("Error running script:", error);
        }
    }

    // function myCompletions(context) {
    //     let before = context.matchBefore(/\w+/);
    //     // If completion wasn't explicitly started and there
    //     // is no word before the cursor, don't open completions.
    //     if (!context.explicit && !before) return null;
    //     return {
    //         from: before ? before.from : context.pos,
    //         options: completions,
    //         validFor: /^\w*$/,
    //     };
    // }



    onMount(() => {
        terminalContainer = document.getElementById("term-container");
        terminalEditor = createEditor(terminalContainer);
    });

</script>

<link rel="stylesheet" href="/css/terminal.css" />

<div class="repl-container">
    <img
        src="/images/Mud_512x218_txt_blk.svg"
        alt="Mud Text Logo"
        class="mud-overlay"
    />
    <div
        id="term-container"
        class="repl-interface"
        role="term"
    >
    </div>
</div>
