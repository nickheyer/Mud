import "./lib/htmx.min.js";
import "./lib/tauri-plugin-htmx.js";

const { invoke } = window.__TAURI__.tauri;
window.addEventListener("DOMContentLoaded", () => {
    console.log('Main.js was loaded, so were htmx min, tauri-htmx, and invoke!')
});