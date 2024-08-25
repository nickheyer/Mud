<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { exit } from "@tauri-apps/plugin-process";
  import _ from 'lodash';
  import { marked } from 'marked';

  $: syncStatus = "Not synced";
  $: recentActivity = [];
  $: scriptInput = "";
  $: scriptOutput = "";

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

  async function runScript() {
    try {
      const scriptContent = `${scriptInput}`;
      const output = await invoke('run_script', { scriptContent });
      scriptOutput = _.trim(output, '"')
        .replace(/\\n/g, '\n');
    } catch (error) {
      scriptOutput = JSON.stringify(error, 4, 2);
      console.error("Error running script:", error);
    }
  }

  async function handleExit(event) {
    event.preventDefault();
    await exit(0);
  }

  onMount(async () => {
    const isSynced = await invoke("get_sync_status");
    syncStatus = isSynced ? "Synced" : "Not synced";
  });
</script>


<nav>
  <ul>
    <li><a href="/">Home</a></li>
    <li><a href="/games">Games</a></li>
    <li><a href="/settings">Settings</a></li>
    <li><a href="/sync">Sync</a></li>
    <li><button class="link-button" on:click={handleExit}>Exit</button></li>
  </ul>
</nav>

<div class="container">
  <img src="/Mud_simple_transparent_gw.svg" alt="Mud Logo" class="logo" />
  
  <h1>Welcome to Mud!</h1>
  <p>Your universal mod manager for various games.</p>

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

  <!-- REPL Interface -->
  <div class="repl-interface">
    <h2>Mudscript Repl</h2>
    <textarea class="repl-input repl" bind:value={scriptInput} placeholder="Type your script here..." rows="5"></textarea>
    <br>
    <textarea class="repl-output repl" bind:value={scriptOutput} readonly placeholder="Your script output goes here..." rows="5"></textarea>
    <br>
    <button on:click={runScript}>Run Script</button>
  </div>
</div>

<style>
:root {
  font-family: 'Inter', 'Avenir', 'Helvetica', 'Arial', sans-serif;
  font-size: 16px;
  line-height: 1.5;
  color: #f6f6f6;
  background-color: #1e1e1e;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

nav {
  background-color: #121212;
  padding: 10px 0;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
  position: sticky;
  top: 0;
  z-index: 10;
}

nav ul {
  display: flex;
  justify-content: center;
  list-style-type: none;
  margin: 0;
  padding: 0;
}

nav ul li {
  margin: 0 15px;
  display: flex;
  justify-content: start;
}

nav ul li a,
nav ul li .link-button {
  color: #f6f6f6;
  text-decoration: none;
  font-size: 1.2rem;
  padding: 10px 15px;
  transition: background-color 0.3s ease, color 0.3s ease;
}

nav ul li a:hover,
nav ul li .link-button:hover {
  background-color: #24c8db;
  color: #ffffff;
  border-radius: 5px;
}

.link-button {
  background: none;
  border: none;
  cursor: pointer;
}

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
.recent-activity,
.repl-interface {
  background-color: #333;
  padding: 20px;
  margin: 20px 0;
  border-radius: 8px;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
}

.sync-status p,
.recent-activity p,
.repl-interface h2 {
  margin: 0 0 10px;
}

button:not(.link-button) {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.8em 1.5em;
  font-size: 1.1em;
  font-weight: 600;
  color: #ffffff;
  background-color: #24c8db;
  transition: background-color 0.3s ease, transform 0.2s ease;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
  cursor: pointer;
}

button:not(.link-button):hover {
  background-color: #1a9caf;
  transform: translateY(-2px);
}

button:not(.link-button):active {
  background-color: #177e92;
  transform: translateY(0);
}

.repl-interface {
  margin-top: 20px;
}

textarea {
  margin: auto;
  width: auto;
  min-width: 60%;
  padding: 10px;
  margin-bottom: 10px;
  border-radius: 5px;
  border: 1px solid #555;
  background-color: #222;
  color: #f6f6f6;
  resize: vertical;
}

.repl-output {
  white-space: pre-wrap;
}

a, .link-button {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover, .link-button:hover {
  color: #24c8db;
}

</style>

