<script>
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { basicSetup, minimalSetup } from "codemirror";
    import { Compartment } from "@codemirror/state"
    import { EditorView, keymap } from "@codemirror/view";
    import { StreamLanguage } from "@codemirror/language";
    import { indentWithTab } from "@codemirror/commands";
    import { shell } from "@codemirror/legacy-modes/mode/shell";

    let editors = [];
    let results = [];
    let terminalContainer;

    // Function to create a new editor
    function createEditor(readOnly = false) {
        const editable = new Compartment();
        let editor = new EditorView({
            doc: readOnly ? "" : "# Shift + Enter to run script!\n",
            extensions: [
                basicSetup,
                keymap.of([indentWithTab]),
                StreamLanguage.define(shell),
                editable.of(EditorView.editable.of(!readOnly)), // Read only set
            ],
            parent: terminalContainer,
        });
        return {
            editor,
            editable
        };
    }

    function createResult(result) {
        const resultText = result.stdout;
        return new EditorView({
            doc: resultText && resultText.length > 0 ? resultText : 'No results to display',
            extensions: [
                minimalSetup,
                EditorView.editable.of(false)
            ],
            parent: terminalContainer,
        });
    }

    function createError(result) {
        const errText = result.message;
        return new EditorView({
            doc: errText && errText.length > 0 ? errText : 'Error: Reason unavailable',
            extensions: [
                minimalSetup,
                EditorView.editable.of(false)
            ],
            parent: terminalContainer,
        });
    }

    async function dispatchAndRotateToNewEditor(oldEditorObj) {
        const { editor, editable } = oldEditorObj;

        // Make the current editor read-only
        editor.dispatch({
            effects: editable.reconfigure(EditorView.editable.of(false))
        });

        // Create a new editor for the next command
        const newEditor = createEditor();
        editors.push(newEditor);

        // Append the element as a new child to the parent container div + scroll to it
        terminalContainer.appendChild(newEditor.editor.dom);
        await scrollToEditor(newEditor.editor);
    }

    // Function to handle running code from the editor
    async function runCode(editorObj) {
        const { editor, editable } = editorObj;

        const command = editor.state.doc.toString();
        try {

            // Invoke with sdk runner
            const output = await invoke("run_script", {
                scriptContent: command,
            });

            // Log output
            console.log(output);
            let outObj;
            try {
                // Push error message to results
                outObj = JSON.parse(output);
            } catch(e) {
                outObj = { stderr: '', stdout: '', variables: null };
            }

            // Save result editor
            results.push(createResult(outObj));

        } catch (err) {

            // Log error
            console.error(err);
            let errObj;
            try {
                // Push error message to results
                errObj = JSON.parse(err);
            } catch(e) {
                errObj = { stderr: '', stdout: '', line: null, message: ''};
            }
            results.push(createError(errObj));
        }

        // Make prev static + gen new editor
        await dispatchAndRotateToNewEditor(editorObj);
    }

    async function scrollToEditor(editor) {
        const elem = editor.dom;
        await elem.scrollIntoView({ behavior: "smooth", block: "end" });
        elem.focus();
    }

    // Function to handle Shift + Enter key combination for running the script
    function handleKeyDown(event) {
        if (event.key === "Enter" && event.shiftKey) {
            event.preventDefault();
            const currentEditor = editors[editors.length - 1]; // Get the last editor
            runCode(currentEditor);
        }
    }

    // Initialize the first editor on mount
    onMount(() => {
        terminalContainer = document.getElementById("term-container");

        let firstEditor = createEditor();
        editors.push(firstEditor);
        window.addEventListener("keydown", handleKeyDown);
        return () => {
            window.removeEventListener("keydown", handleKeyDown);
        };
    });



</script>

<link rel="stylesheet" href="/css/terminal.css" />

<div class="repl-container">
    <img
        src="/images/Mud_512x218_txt_blk.svg"
        alt="Mud Text Logo"
        class="mud-overlay"
    />
    <div id="term-container" class="repl-interface" role="term"></div>
</div>

<style>
</style>
