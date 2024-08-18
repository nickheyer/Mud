<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from 'svelte';

  function sleep(seconds) {
    return new Promise(resolve => setTimeout(resolve, seconds * 1000));
  }

  async function setup() {
    // Fake perform some really heavy setup task
    console.log('Performing really heavy frontend setup task...')
    await sleep(6);
    console.log('Frontend setup task complete!')
    invoke('set_complete', {task: 'frontend'})
  }
  
  // Navigate to the home page after a delay
  onMount(async () => {
    await setup()
  });
</script>

<div class="splash-container">
  <img src="/Mud_full_transparent_scaled_gw.png" alt="Mud Logo" class="splash-logo" />
</div>

<style>
  .splash-container {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
    background-color: #2f2f2f; /* Background color for the splash screen */
  }

  .splash-logo {
    height: 200px;
    width: 200px;
    animation: pulse-glow 5s infinite;
  }

  @keyframes pulse-glow {
    0% {
      filter: drop-shadow(0 0 2em #24c8db);
    }
    50% {
      filter: drop-shadow(0 0 3em #24c8db) blur(5px);
    }
    100% {
      filter: drop-shadow(0 0 2em #24c8db);
    }
  }
</style>
