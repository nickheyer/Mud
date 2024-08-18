<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from 'svelte';
  import { exit, relaunch } from '@tauri-apps/plugin-process';

  let syncStatus = "Not synced";
  $: recentActivity = [];

  // Function to sync the repo
  async function syncRepo() {
    try {
      syncStatus = "Syncing...";
      // Replace 'sync_repo' with the actual Tauri command for syncing
      //await invoke("sync_repo");
      syncStatus = "Sync successful!";
      // Refresh recent activity or any other data after sync
      recentActivity = [
        ...recentActivity,
        { time: new Date().toLocaleString(), message: "Repo synced" }
      ];
      console.log(recentActivity.length);
    } catch (error) {
      syncStatus = `Sync failed: ${error.message}`;
    }
  }

  async function handleExit(event) {
    event.preventDefault(); // Prevent the default behavior of the <a> tag
    await exit(0);
  }

  // On mount, we check the sync status from the backend
  onMount(async () => {
    const isSynced = await invoke('get_sync_status');
    syncStatus = isSynced ? 'Synced' : 'Not synced';
  });
</script>

<!-- Top Navigation Bar -->
<nav>
  <ul>
    <li><a href="/">Home</a></li>
    <li><a href="/games">Games</a></li>
    <li><a href="/settings">Settings</a></li>
    <li><a href="/sync">Sync</a></li>
    <li><a href="{null}" target="_blank" on:click={handleExit}>Exit</a></li>
  </ul>
</nav>

<!-- Main Content -->
<div class="container">
  <h1>Welcome to Mud!</h1>
  <p>Your universal mod manager for various games.</p>

  <!-- Sync Status -->
  <div class="sync-status">
    <p><strong>Sync Status:</strong> {syncStatus}</p>
    <button on:click={syncRepo}>Sync Now</button>
  </div>

  <!-- Recent Activity -->
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
  nav ul {
    display: flex;
    list-style-type: none;
    padding: 0;
    background-color: #333;
    justify-content: space-around;
  }

  nav ul li {
    margin: 0;
  }

  nav ul li a {
    display: block;
    padding: 14px 20px;
    color: white;
    text-decoration: none;
  }

  nav ul li a:hover {
    background-color: #575757;
  }

  .container {
    margin: 0;
    padding-top: 10vh;
    text-align: center;
  }

  .sync-status, .recent-activity {
    margin: 20px 0;
  }

  .sync-status p, .recent-activity p {
    margin: 0;
  }

  button {
    background-color: #4CAF50;
    color: white;
    border: none;
    border-radius: 8px;
    box-shadow: 0 4px #2b7a3b;
  }

  button:hover {
    background-color: #45a049;
  }

  button:active {
    box-shadow: 0 2px #2b7a3b;
    transform: translateY(2px);
  }

  .logo:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: 0.75s;
  }

  .row {
    display: flex;
    justify-content: center;
  }

  a {
    font-weight: 500;
    color: #646cff;
    text-decoration: inherit;
  }

  a:hover {
    color: #535bf2;
  }

  h1 {
    text-align: center;
  }

  input,
  button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }

  button {
    cursor: pointer;
  }

  button:hover {
    border-color: #396cd8;
  }
  button:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
  }

  input,
  button {
    outline: none;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    a:hover {
      color: #24c8db;
    }

    input,
    button {
      color: #ffffff;
      background-color: #0f0f0f98;
    }
    button:active {
      background-color: #0f0f0f69;
    }
  }

</style>
