<script>
    import { exit } from "@tauri-apps/plugin-process";

    async function handleExit(event) {
        event.preventDefault();
        await exit(0);
    }
</script>

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

<style>
    :root {
        font-family: "Inter", "Avenir", "Helvetica", "Arial", sans-serif;
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
        transition:
            background-color 0.3s ease,
            color 0.3s ease;
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

    button:not(.link-button) {
        border-radius: 8px;
        border: 1px solid transparent;
        padding: 0.8em 1.5em;
        font-size: 1.1em;
        font-weight: 600;
        color: #ffffff;
        background-color: #24c8db;
        transition:
            background-color 0.3s ease,
            transform 0.2s ease;
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

    a,
    .link-button {
        font-weight: 500;
        color: #646cff;
        text-decoration: inherit;
    }

    a:hover,
    .link-button:hover {
        color: #24c8db;
    }
</style>
