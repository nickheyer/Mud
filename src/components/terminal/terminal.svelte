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

    // State effect to add an underline effect
    const addUnderlineEffect = StateEffect.define();

    // Defines state field for underlines
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

    /**
     * Apply underline effect to a specific line in the editor.
     * @param {number} lineNumber - The line number to underline.
     * @param {EditorView} editor - The editor instance.
     */
    function underlineLine(lineNumber, editor) {
        editor.dispatch({
            effects: addUnderlineEffect.of(lineNumber),
        });
    }

    /**
     * Create a new editor instance and focus it.
     * @param {boolean} readOnly - Whether the editor should be read-only.
     * @returns {Object} The created editor and its editable compartment.
     */
    function createEditor(readOnly = false) {
        const editable = new Compartment();
        const editor = new EditorView({
            doc: readOnly || editors.length > 0 ? "" : initComment,
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
        return { editor, editable };
    }

    /**
     * Create an editor view to display script execution results.
     * @param {Object} result - The result object containing stdout.
     * @returns {EditorView} The result editor view.
     */
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

    /**
     * Create an editor view to display error messages.
     * @param {Object} result - The result object containing the error message.
     * @returns {EditorView} The error editor view.
     */
    function createError(result) {
        const errText = result.message;
        return new EditorView({
            doc: errText && errText.length > 0 ? errText : "Error: Unknown",
            extensions: [minimalSetup, EditorView.editable.of(false)],
            parent: terminalContainer,
        });
    }

    /**
     * Dispatch changes to the current editor and create a new one.
     * @param {Object} oldEditorObj - The previous editor object.
     */
    async function dispatchAndRotateToNewEditor(oldEditorObj) {
        const { editor, editable } = oldEditorObj;

        // Make read-only
        editor.dispatch({
            effects: editable.reconfigure(EditorView.editable.of(false)),
        });

        // Create editor for next command
        const newEditor = createEditor();
        editors.push(newEditor);

        // Append new editor and scroll into view
        terminalContainer.appendChild(newEditor.editor.dom);
        await scrollToEditor(newEditor.editor);
    }

    /**
     * Run the script code and handle the result or error.
     * @param {Object} editorObj - The editor object containing the code to run.
     */
    async function runCode(editorObj) {
        const { editor } = editorObj;
        const command = editor.state.doc.toString();
        let result = null;

        try {
            const output = await invoke("run_script", {
                scriptContent: command,
            });
            let outObj = JSON.parse(output);
            result = createResult(outObj);
        } catch (err) {
            let errObj;
            try {
                errObj = JSON.parse(err);
                if (errObj.line) {
                    underlineLine(errObj.line, editor);
                }
            } catch (e) {
                errObj = { stderr: "", stdout: "", line: null, message: "" };
            }
            result = createError(errObj);
        }

        if (result) {
            results.push(result);
        }

        // Prepare for the next input
        await dispatchAndRotateToNewEditor(editorObj);
    }

    /**
     * Smoothly scroll to the provided editor view.
     * @param {EditorView} editor - The editor to scroll to.
     */
    async function scrollToEditor(editor) {
        const elem = editor.dom;
        await elem.scrollIntoView({ behavior: "smooth", block: "end" });
        editor.focus();
    }

    /**
     * Handle Shift + Enter key combination to run the script.
     * @param {KeyboardEvent} event - The keyboard event object.
     */
    function handleKeyDown(event) {
        if (event.key === "Enter" && event.shiftKey) {
            event.preventDefault();
            const currentEditor = editors[editors.length - 1]; // Get the last editor
            runCode(currentEditor);
        }
    }

    // Init the first editor on mount
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
