<script>
  import Activity from "$lib/components/activity/activity.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { logActivity } from "$lib/stores/activityStore";
  import { LazyStore } from '@tauri-apps/plugin-store';
  import { appLocalDataDir } from "@tauri-apps/api/path";
  import { onMount } from "svelte";
  import _ from "lodash";

  let store;
  let syncStatus = "Not synced";
  let appDataDirPath = '';
  let isSyncing = false;
  let isSelectingDir = false;

  async function updateSyncStatus(statusMessage, log = true) {
      syncStatus = statusMessage;
      if (log) {
          await logActivity(statusMessage);
      }
  }

  // Syncs the repository
  async function syncRepo() {
      if (isSyncing) return; // Prevents double clicks
      isSyncing = true;
      await updateSyncStatus("Syncing...", false);

      try {
          await invoke("try_sync_repo", { appDataDir: appDataDirPath });
          await updateSyncStatus("Sync successful!");
      } catch (error) {
          await updateSyncStatus(`Sync failed: ${error.message || error}`);
      } finally {
          isSyncing = false;
      }
  }

  async function chooseAppDataDir() {
      if (isSelectingDir) return;
      isSelectingDir = true;
      await updateSyncStatus("Migrating AppData Directory...", false);

      try {
          const selectedPath = await invoke("select_appdata_path");
          appDataDirPath = selectedPath;
          await store.set('app-data-custom', { value: appDataDirPath });
          await updateSyncStatus(`AppData Directory Reconfigured, Awaiting Sync!`);
          await store.save();
      } catch (error) {
          await updateSyncStatus(`Configuration failed: ${error.message}`);
      } finally {
          isSelectingDir = false;
      }
  }

  onMount(async () => {
      store = new LazyStore('store.bin');
      const appDataCustom = await store.get('app-data-custom');
      appDataDirPath = await appLocalDataDir();
      const isSynced = await invoke("get_sync_status");
      syncStatus = isSynced ? "Synced" : "Not synced!";
  });
</script>


<link rel="stylesheet" href="/css/main.css">
<div class="container">
  <img src="/images/Mud_simple_transparent_gw.svg" alt="Mud Logo" class="logo" />

  <h1>Sync Settings</h1>

  <!-- Sync Status Section -->
  <div class="sync-status">
      <p><strong>Sync Status:</strong> {syncStatus}</p>
      <button on:click={syncRepo} disabled={isSyncing}>
          {#if isSyncing} Syncing... {/if}
          {#if !isSyncing} Sync Now {/if}
      </button>
  </div>

  <!-- AppData Path Configuration -->
  <div class="sync-status select-appdata-dir">
      <p><strong>Appdata Path:</strong> {appDataDirPath}</p>
      <button on:click={chooseAppDataDir} disabled={isSelectingDir}>
          {#if isSelectingDir} Selecting Directory... {/if}
          {#if !isSelectingDir} Configure App Folder {/if}
      </button>
  </div>

  <!-- Recent Activity Component -->
  <Activity />
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
