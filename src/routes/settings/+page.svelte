<script>

    import { invoke } from "@tauri-apps/api/core";
    import { logActivity } from "$lib/stores/activityStore";
    import { Store } from "@tauri-apps/plugin-store";
    import { appLocalDataDir } from "@tauri-apps/api/path";
    import { onMount } from "svelte";
    import _ from "lodash";

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

    <!-- AppData Path Configuration -->
    <div class="sync-status select-appdata-dir">
        <p><strong>Appdata Path:</strong> {appDataDirPath}</p>
        <button on:click={chooseAppDataDir} disabled={isSelectingDir}>
            {#if isSelectingDir}
                Selecting Directory...
            {/if}
            {#if !isSelectingDir}
                Configure App Folder
            {/if}
        </button>
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
