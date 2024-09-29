<script>
  import { invoke } from "@tauri-apps/api/core";
  import { Store } from '@tauri-apps/plugin-store';
  import { appLocalDataDir } from '@tauri-apps/api/path';
  import { onMount } from "svelte";
  import _ from "lodash";

  let store;
  $: syncStatus = "Not synced";
  $: appDataDirPath = '';
  $: recentActivity = [];
  
  async function logActivity(message, time = null) {
    if (!message || !message.length) {
      console.trace('Attempting to log activity without message!');
      return;
    }

    recentActivity = [
      ...recentActivity,
      {
        time: time || new Date().toLocaleString(),
        message
      }
    ];

    await store.set('activity', {
      value: recentActivity
    });
  }

  async function syncRepo() {
    syncStatus = "Syncing...";
    try {
      await invoke("try_sync_repo", {
        appDataDir: appDataDirPath
      });
      syncStatus = "Sync successful!";
      await logActivity(syncStatus);
    } catch (error) {
      syncStatus = `Sync failed: ${error}`;
      await logActivity(syncStatus);
    }
  }

  async function chooseAppDataDir() {
    syncStatus = "Migrating AppData Directory...";
    await logActivity('Starting AppData migration process (User selection)');
    try {
      [appDataDirPath] = await invoke("select_appdata_path");
      await store.set('app-data-custom', { 'value': appDataDirPath });

      syncStatus = "Awaiting Resync, Press Sync...";
      await logActivity(`AppData Directory Reconfigured: ${appDataDirPath}`);

      await store.save();
    } catch (error) {
      syncStatus = `Configuration failed: ${error.message}`;
      await logActivity(syncStatus);
    }
  }

  onMount(async () => {
    store = new Store('store.bin');
    const appDataCustom = await store.get('app-data-custom');
    appDataDirPath = appDataCustom?.value || await appLocalDataDir();

    const isSynced = await invoke("get_sync_status");
    syncStatus = isSynced ? "Synced" : "Not synced";

    const activityStore = await store.get('activity');
    recentActivity = activityStore?.value || [];
  });

</script>

<link rel="stylesheet" href="/css/main.css">

<div class="container">
  <img src="/images/Mud_simple_transparent_gw.svg" alt="Mud Logo" class="logo" />

  <h1>Sync Settings</h1>

  <div class="sync-status">
    <p><strong>Sync Status:</strong> {syncStatus}</p>
    <button on:click={syncRepo}>Sync Now</button>
  </div>

  <div class="sync-status select-appdata-dir">
    <p><strong>Appdata Path:</strong> {appDataDirPath}</p>
    <button on:click={chooseAppDataDir}>Configure App Folder</button>
  </div>

  <div class="recent-activity">
    <h2>Recent Activity</h2>
    {#if recentActivity.length > 0}
      <ul>
        {#each recentActivity as activity}
          <li>{activity.time}: {activity.message}</li>
        {/each}
      </ul>
    {:else}
      <p>No recent activity.</p>
    {/if}
  </div>
</div>

