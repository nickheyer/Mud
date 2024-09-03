<script>
    import { basicSetup, minimalSetup } from "codemirror";
    import { Compartment, StateEffect, StateField } from "@codemirror/state";
    import {
        keymap,
        EditorView,
        Decoration,
        DecorationSet,
    } from "@codemirror/view";
    import { StreamLanguage } from "@codemirror/language";
    import { indentWithTab } from "@codemirror/commands";
    import { mud } from "./linting-rules/mud";
    import { setDiagnostics, forceLinting } from "@codemirror/lint";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let editors = [];
    let results = [];
    let terminalContainer;

    const initComment = `goto :motd
    ───────────────╾⧗ WELCOME TO THE MUD TERMINAL ⧗╼───────────────

     Welcome to
     ███╗   ███╗██╗   ██╗██████╗ 
     ████╗ ████║██║   ██║██╔══██╗
     ██╔████╔██║██║   ██║██║  ██║
     ██║╚██╔╝██║██║   ██║██║  ██║
     ██║ ╚═╝ ██║╚██████╔╝██████╔╝
     ╚═╝     ╚═╝ ╚═════╝ ╚═════╝ 

    ▌▛▖ 🡆 Press **Ctrl + Space** to unveil the arsenal of commands in the standard 
    ▌▛▖ library. Let the autocomplete guide you through the digital fog.

    ▌▛▖ 🡆 Ready to execute your script? Hit **Shift + Enter** to run it in this 
    ▌▛▖ terminal and see the code come alive.

    ▌▛▖ Good luck, hacker.

    ──────────────────────────────────────────────────────────────────\n:motd
    \n\n`;

    const addUnderlineEffect = StateEffect.define();
    const underlineField = StateField.define({
        create() {
            return Decoration.none;
        },
        update(underlines, tr) {
            underlines = underlines.map(tr.changes);
            for (let effect of tr.effects) {
                if (effect.is(addUnderlineEffect)) {
                    const line = tr.state.doc.line(effect.value);
                    const underlineDeco = Decoration.line({
                        class: "term-line-err",
                    }).range(line.from);
                    underlines = underlines.update({ add: [underlineDeco] });
                }
            }
            return underlines;
        },
        provide: (field) => EditorView.decorations.from(field),
    });

    // Wraps the above code-mirror abstracted bs into this function to change the style on a single line
    function underlineLine(lineNumber, editor) {
        editor.dispatch({
            effects: addUnderlineEffect.of(lineNumber),
        });
    }

    // Spawn new editor + focus it
    function createEditor(readOnly = false) {
        const editable = new Compartment();

        const editor = new EditorView({
            doc:
                readOnly || editors.length > 0
                    ? ""
                    : initComment,
            extensions: [
                basicSetup,
                keymap.of([indentWithTab]),
                StreamLanguage.define(mud),
                editable.of(EditorView.editable.of(!readOnly)),
                underlineField,
            ],
            parent: terminalContainer,
        });

        editor.focus();
        return {
            editor,
            editable,
        };
    }

    function createResult(result) {
        const resultText = result.stdout;
        return new EditorView({
            doc:
                resultText && resultText.length > 0
                    ? resultText
                    : "No results to display",
            extensions: [minimalSetup, EditorView.editable.of(false)],
            parent: terminalContainer,
        });
    }

    function createError(result) {
        const errText = result.message;
        console.log(result);
        return new EditorView({
            doc: errText && errText.length > 0 ? errText : "Error: Unknown",
            extensions: [minimalSetup, EditorView.editable.of(false)],
            parent: terminalContainer,
        });
    }

    async function dispatchAndRotateToNewEditor(oldEditorObj) {
        const { editor, editable } = oldEditorObj;

        // Make the current editor read-only
        editor.dispatch({
            effects: editable.reconfigure(EditorView.editable.of(false)),
        });

        // Create a new editor for the next command
        const newEditor = createEditor();
        editors.push(newEditor);

        // Append the element as a new child to the parent container div + scroll to it
        terminalContainer.appendChild(newEditor.editor.dom);
        await scrollToEditor(newEditor.editor);
    }

    // Invoke script on backend
    async function runCode(editorObj) {
        const { editor, editable } = editorObj;

        const command = editor.state.doc.toString();
        console.log(command);
        let result = null;
        try {
            // Invoke with sdk runner
            const output = await invoke("run_script", {
                scriptContent: command,
            });

            // Log output
            console.log(output);
            let outObj;
            try {
                // Push output to results
                outObj = JSON.parse(output);
            } catch (e) {
                outObj = { stderr: "", stdout: "", variables: null };
            }
            result = createResult(outObj);
        } catch (err) {
            // Log error
            console.error(err);
            let errObj;
            try {
                // Parse error to object
                errObj = JSON.parse(err);
                if (errObj.line) {
                    // Generate editor diagnostic + dispatch to editor
                    underlineLine(errObj.line, editor);
                }
            } catch (e) {
                console.log(e);
                errObj = { stderr: "", stdout: "", line: null, message: "" };
            }
            result = createError(errObj);
        }

        if (result) {
            results.push(result);
        }

        // Make prev static + gen new editor
        await dispatchAndRotateToNewEditor(editorObj);
    }

    async function scrollToEditor(editor) {
        const elem = editor.dom;
        await elem.scrollIntoView({ behavior: "smooth", block: "end" });
        editor.focus();
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
