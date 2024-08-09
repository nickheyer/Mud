const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;
let opStatusEl;

async function greet() {
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
  opStatusEl.textContent = await invoke('create_mud', { name: greetInputEl.value })
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  opStatusEl = document.querySelector("#op-status");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
