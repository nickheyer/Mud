<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import _ from "lodash";

  $: syncStatus = "Not synced";
  $: appDataDirPath = "No AppData Storage Selected";
  $: recentActivity = [];

  async function syncRepo() {
    syncStatus = "Syncing...";
    try {
      await invoke("try_sync_repo");
      syncStatus = "Sync successful!";
      recentActivity = [
        ...recentActivity,
        {
          time: new Date().toLocaleString(),
          message: "Repo synced"
        },
      ];
    } catch (error) {
      syncStatus = `Sync failed: ${error.message}`;
    }
  }

  async function chooseAppDataDir() {
    syncStatus = "Migrating AppData Directory...";
    try {
      [appDataDirPath] = await invoke("select_appdata_path");
      console.log(JSON.stringify({ newPath: appDataDirPath }, 4, 2));
      syncStatus = "Awaiting Resync, Press Sync...";
      recentActivity = [
        ...recentActivity,
        { 
          time: new Date().toLocaleString(),
          message: `AppData Directory Reconfigured: ${appDataDirPath}`
        },
      ];
    } catch (error) {
      syncStatus = `Configuration failed: ${error.message}`;
    }
  }

  onMount(async () => {
    const isSynced = await invoke("get_sync_status");
    syncStatus = isSynced ? "Synced" : "Not synced";
  });
</script>

<link rel="stylesheet" href="/css/main.css">

<div class="container">
  <img src="/images/Mud_simple_transparent_gw.svg" alt="Mud Logo" class="logo" />

  <h1>Welcome to Mud!</h1>
  <p>The multiplatform mod manager.</p>

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

