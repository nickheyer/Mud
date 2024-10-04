<script>
import { exit } from "@tauri-apps/plugin-process";
import { emit, listen } from '@tauri-apps/api/event';
import { forwardAll } from "$lib/utils/logging";
import { onMount } from "svelte";
import { Button } from "$lib/components/ui/button";

async function handleNav(event) {
    emit('page-nav', {});
}

async function handleExit(event) {
    event.preventDefault();
    await exit(0);
}

onMount(() => {
    forwardAll();
});</script>

<link rel="stylesheet" href="/css/global.css" />

<nav>
    <ul>
        <li><a href="/" on:click="{handleNav}">Home</a></li>
        <li><a href="/games" on:click="{handleNav}">Games</a></li>
        <li><a href="/settings" on:click="{handleNav}">Settings</a></li>
        <li><a href="/sync" on:click="{handleNav}">Sync</a></li>
        <li><a href="/repl" on:click="{handleNav}">Repl</a></li>
        <li><button class="link-button" on:click="{handleExit}">Exit</button></li>
    </ul>
</nav>

<slot></slot>

