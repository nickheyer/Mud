<script>
    import { exit } from "@tauri-apps/plugin-process";
    
    async function handleExit(event) {
        event.preventDefault();
        await exit(0);
    }
</script>

<link rel="stylesheet" href="/css/global.css">

<nav>
    <ul>
        <li><a href="/">Home</a></li>
        <li><a href="/games">Games</a></li>
        <li><a href="/settings">Settings</a></li>
        <li><a href="/sync">Sync</a></li>
        <li><a href="/repl">Repl</a></li>
        <li><button class="link-button" on:click={handleExit}>Exit</button></li>
    </ul>
</nav>

<slot></slot>

