<script>
  import Activity from "$lib/components/activity/activity.svelte";
  import { loadActivity } from "$lib/stores/activityStore";
  import { onMount } from "svelte";
  import { writable } from 'svelte/store';
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  export const scriptContentStore = writable('');

  onMount(async () => {
    const scriptContent = await invoke('get_cli_script');
    if (scriptContent) {
      await loadScriptIntoRepl(scriptContent);
    }
    await loadActivity();
  });

  async function loadScriptIntoRepl(scriptContent) {
    console.log(scriptContent);
    scriptContentStore.set(scriptContent);
    await goto('/repl', {
      state: { scriptContent }
    });
    
  }

</script>

<link rel="stylesheet" href="/css/main.css">

<div class="container">
  <img src="/images/Mud_simple_transparent_gw.svg" alt="Mud Logo" class="logo" />

  <h1>Welcome to Mud!</h1>
  <p>The multiplatform mod manager.</p>

  <Activity />
</div>

