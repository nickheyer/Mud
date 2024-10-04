<script>

    import { invoke } from "@tauri-apps/api/core";
    import { logActivity } from "$lib/stores/activityStore";
    import { Store } from "@tauri-apps/plugin-store";
    import { appLocalDataDir } from "@tauri-apps/api/path";
    import { onMount } from "svelte";
    import _ from "lodash";
    import { Button } from "$lib/components/ui/button";

    import * as Form from "$lib/components/ui/form";
    import { Input } from "$lib/components/ui/input";
    import { formSchema } from "$lib/components/formgen/schema";
    import {
        superForm,
    } from "sveltekit-superforms";
    import { zodClient } from "sveltekit-superforms/adapters";
    
    export let data;
    
    const form = superForm(data, {
        validators: zodClient(formSchema),
    });
    
    const { form: formData, enhance } = form;
    // https://www.shadcn-svelte.com/docs/components/form
    
    
    let store;
    let appDataDirPath = "";
    let isSelectingDir = false;

    async function chooseAppDataDir() {
        if (isSelectingDir) return;
        isSelectingDir = true;
        try {
            const selectedPath = await invoke("select_appdata_path");
            appDataDirPath = selectedPath;
            await store.set("app-data-custom", { value: appDataDirPath });
            await store.save();
        } catch (error) {
            console.log(error);
        } finally {
            isSelectingDir = false;
        }
    }

    onMount(async () => {
        store = new Store("store.bin");
        const appDataCustom = await store.get("app-data-custom");
        appDataDirPath = appDataCustom?.value || (await appLocalDataDir());
    });
</script>

<link rel="stylesheet" href="/css/main.css" />

<div class="container">
    <h1>Settings</h1>

    <form method="GET" use:enhance>
        <Form.Field {form} name="username">
          <Form.Control let:attrs>
            <Form.Label>Username</Form.Label>
            <Input {...attrs} bind:value={$formData.username} />
          </Form.Control>
          <Form.Description>This is your public display name.</Form.Description>
          <Form.FieldErrors />
        </Form.Field>
        <Form.Button>Submit</Form.Button>
      </form>


    <!-- AppData Path Configuration -->
    <div class="sync-status select-appdata-dir">
        <p><strong>Appdata Path:</strong> {appDataDirPath}</p>
        <Button on:click={chooseAppDataDir} disabled={isSelectingDir}>
            {#if isSelectingDir}
                Selecting Directory...
            {/if}
            {#if !isSelectingDir}
                Configure App Folder
            {/if}
        </Button>
    </div>
</div>

<style>
    .container {
        padding: 20px;
    }

    .sync-status {
        margin-bottom: 15px;
    }

    button:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }
</style>
