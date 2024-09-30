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
    import { insertTab, indentLess } from "@codemirror/commands";
    import { indentationMarkers } from '@replit/codemirror-indentation-markers';
    import { mud } from "./linting-rules/mud";
    import { setDiagnostics, forceLinting } from "@codemirror/lint";
    import { invoke, Channel } from "@tauri-apps/api/core";
    import { emit } from '@tauri-apps/api/event';
    import { onMount } from "svelte";
    import dedent from "dedent";
    import {
        isPermissionGranted,
        requestPermission,
        sendNotification,
    } from '@tauri-apps/plugin-notification';

    let editors = [];
    let results = [];
    let numEditors = 0;
    let terminalContainer;
    let isRunningCode = false;

    const initComment = dedent`
    goto :motd
    ─────────────╾⧗ WELCOME TO THE MUD TERMINAL ⧗╼─────────────

     Welcome to
     ███╗   ███╗██╗   ██╗██████╗ 
     ████╗ ████║██║   ██║██╔══██╗
     ██╔████╔██║██║   ██║██║  ██║
     ██║╚██╔╝██║██║   ██║██║  ██║
     ██║ ╚═╝ ██║╚██████╔╝██████╔╝
     ╚═╝     ╚═╝ ╚═════╝ ╚═════╝ 

    ▌▛▖ 🡆 Ready to Try it out?
    ▌▛▖ **Ctrl + Space** shows you what you can do.
    ▌▛▖ **Shift + Enter** runs it.
    ▌▛▖ **Control + C** kills it.
    ▌▛▖ **Shift + Control + C** creates a new editor.
    ▌▛▖ **Shift + Up/Down Arrow** swaps content with other editor.


    Good luck, hacker.

    ────────────────────────────────────────────────────────────
    :motd\n\n
    `;

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

    const indentWithTab = { key: "Tab", run: insertTab, shift: indentLess };

    /**
     * Create a new editor instance and focus it.
     * @param {boolean} readOnly - Whether the editor should be read-only.
     * @returns {Object} The created editor and its editable compartment.
     */
    function createEditor(readOnly = false) {
        const editable = new Compartment();
        const textInit = readOnly || editors.length > 0 ? "" : initComment;
        const editor = new EditorView({
            doc: textInit,
            extensions: [
                basicSetup,
                keymap.of([indentWithTab]),
                StreamLanguage.define(mud),
                editable.of(EditorView.editable.of(!readOnly)),
                underlineField,
                indentationMarkers({
                    thickness: 1,
                    colors: {
                        dark: '#2d054f99',
                        light: '#2d054f99'
                    }
                })
            ],
            selection: {
                anchor: textInit.length
            },
            parent: terminalContainer,
        });

        editor.focus();
        return { editor, editable };
    }

    /**
     * Append new content to the existing editor.
     * @param {EditorView} editor - The editor instance.
     * @param {string} newText - The new text to append.
     */
    function appendToEditor(editor, newText) {
        const currentDoc = editor.state.doc.toString();
        const updatedDoc = currentDoc + "\n" + newText;

        // Dispatch the new state to append the text
        editor.dispatch({
            changes: { from: currentDoc.length, insert: newText },
        });
        scrollToEditor(editor);
    }

    function replaceInEditor(editor, otherEditor) {
        const currentDoc = editor.state.doc.toString();
        const prevDoc = otherEditor.state.doc.toString();
        //sendNotification({ title: 'Swapping Editor', body: prevDoc });
        
        // Dispatch the chosen editors text to replace current editor text
        editor.dispatch({
            changes: { from: 0, to: currentDoc.length, insert: prevDoc },
        });
        scrollToEditor(editor);
    }

    /**
     * Create an editor view to display script execution results.
     * @returns {EditorView} The result editor view.
     */
    function createResultEditor(updater) {
        const resEditor = new EditorView({
            extensions: [minimalSetup, EditorView.editable.of(false)],
            parent: terminalContainer,
        });
        updater.onmessage = (message) => {
            const stdoutText = message?.data?.message || null;
            if (stdoutText) {
                appendToEditor(resEditor, stdoutText);
            }
        };
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
        numEditors = editors.length;

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

        let resultEditor = null;
        const onEvent = new Channel();
        isRunningCode = true;


        // Let the command run in background with it's callback.
        createResultEditor(onEvent);
        await invoke("run_script", {
            scriptContent: command,
            onEvent
        })
        .then((message) => console.log(message))
        .catch((err) => {
            console.log(JSON.stringify(err));
            let errObj;
            try {
                errObj = JSON.parse(err);
                if (errObj.line) {
                    underlineLine(errObj.line, editor);
                }
            } catch (e) {
                errObj = { stderr: "", stdout: "", line: null, message: "" };
            }
            resultEditor = createError(errObj);
        });

        // Prepare for the next input
        isRunningCode = false;
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
    async function handleKeyDown(event) {
        if (event.key === "Enter" && event.shiftKey) {
            event.preventDefault();
            const currentEditor = editors[editors.length - 1]; // Get the last editor
            await runCode(currentEditor);
        } else if (event.ctrlKey && event.code == 'KeyC' && (event.shiftKey || isRunningCode)) {
            event.preventDefault();
            const currentEditor = editors[editors.length - 1];
            if (isRunningCode) {
                await createError({ message: 'Script terminated with Control-C' });
                await emit('page-nav', {});
            }
            if (event.shiftKey) {
                await dispatchAndRotateToNewEditor(currentEditor);
            }
        } else if (
            event.shiftKey &&
            (
                event.key == 'ArrowUp' && numEditors > 1 ||
                event.key == 'ArrowDown' && numEditors < editors.length
            ) &&
            !isRunningCode
        ) {
            event.preventDefault();
            const currentEditor = editors[editors.length - 1];
            if (event.key == 'ArrowUp') {
                numEditors -= 1;
            } else {
                numEditors += 1;
            }
            const targetEditor = editors[numEditors - 1];
            replaceInEditor(currentEditor.editor, targetEditor.editor);
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
