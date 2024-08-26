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

<style>
  .container {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem 1rem;
    text-align: center;
  }

  .logo {
    width: 150px;
    height: auto;
    margin-bottom: 20px;
  }

  .logo:hover {
    animation: logo-spin infinite 20s linear;
  }

  @keyframes logo-spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  h1 {
    font-size: 2.5rem;
    margin: 1rem 0;
    color: #24c8db;
    text-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
  }

  .sync-status,
  .recent-activity {
    background-color: #333;
    padding: 20px;
    margin: 20px 0;
    border-radius: 8px;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
  }

  .sync-status p,
  .recent-activity p {
    margin: 0 0 10px;
  }
</style>
