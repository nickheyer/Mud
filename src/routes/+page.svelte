<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { exit } from "@tauri-apps/plugin-process";
  import _ from "lodash";
  import { marked } from "marked";

  $: syncStatus = "Not synced";
  $: recentActivity = [];

  async function syncRepo() {
    syncStatus = "Syncing...";
    try {
      await invoke("get_sync_status");
      syncStatus = "Sync successful!";
      recentActivity = [
        ...recentActivity,
        { time: new Date().toLocaleString(), message: "Repo synced" },
      ];
    } catch (error) {
      syncStatus = `Sync failed: ${error.message}`;
    }
  }

  onMount(async () => {
    const isSynced = await invoke("get_sync_status");
    syncStatus = isSynced ? "Synced" : "Not synced";
  });
</script>

<link rel="stylesheet" href="/src/routes/main.css">

<div class="container">
  <img src="/Mud_simple_transparent_gw.svg" alt="Mud Logo" class="logo" />

  <h1>Welcome to Mud!</h1>
  <p>The multiplatform mod manager.</p>

  <div class="sync-status">
    <p><strong>Sync Status:</strong> {syncStatus}</p>
    <button on:click={syncRepo}>Sync Now</button>
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

