<script>
  import { invoke } from "@tauri-apps/api/core";
  import { Store } from '@tauri-apps/plugin-store';
  import { appLocalDataDir } from '@tauri-apps/api/path';
  import { onMount } from "svelte";
  import _ from "lodash";

  let store;
  $: recentActivity = [];

  onMount(async () => {
    store = new Store('store.bin');
    const activityStore = await store.get('activity');
    recentActivity = activityStore?.value || [];
  });
</script>

<link rel="stylesheet" href="/css/main.css">

<div class="container">
  <img src="/images/Mud_simple_transparent_gw.svg" alt="Mud Logo" class="logo" />

  <h1>Welcome to Mud!</h1>
  <p>The multiplatform mod manager.</p>

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

