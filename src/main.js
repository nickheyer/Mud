const { invoke } = window.__TAURI__.tauri;
const { open } = window.__TAURI__.dialog;

let currentScreen = 'welcome-screen';

function showScreen(screenId) {
  document.querySelector(`#${currentScreen}`).classList.remove('active');
  currentScreen = screenId;
  document.querySelector(`#${currentScreen}`).classList.add('active');
}

async function installMods(files) {
  document.querySelector("#install-status").textContent = await invoke('install_mods', { files });
  showScreen("completion-screen");
}

async function syncRepo() {
  const repoUrl = "https://github.com/nickheyer/Mud.Community.git";
  const localPath = await open({
    directory: true,
    title: 'Choose a directory to store the repository',
  });

  if (localPath) {
    document.querySelector("#sync-status").textContent = "Syncing with the community repository...";
    const message = await invoke('sync_with_repo', { repoUrl, localPath });
    document.querySelector("#sync-status").textContent = message;
    showScreen("setup-screen");
  } else {
    console.log('Repository sync was cancelled or failed');
  }
}

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#start-setup").addEventListener("click", () => {
    showScreen("setup-screen");
  });

  document.querySelector("#sync-repo-button").addEventListener("click", () => {
    syncRepo();
  });

  document.querySelector("#setup-form").addEventListener("submit", async (e) => {
    e.preventDefault();

    let modDir = document.querySelector("#mod-directory").value;
    if (!modDir) {
      modDir = await open({
        directory: true,
        title: 'Choose a mod directory',
        filters: [{
          name: 'Mod Dir',
          extensions: ['*']
        }]
      });
    }
    document.querySelector("#setup-status").textContent = `Setting up game at ${modDir}...`;
    const message = await invoke('create_mud', { name: modDir });
    document.querySelector("#setup-status").textContent = message;
    showScreen("mod-install-screen");
  });

  document.querySelector("#browse-files").addEventListener("click", async (e) => {
    e.preventDefault();
    const selected = await open({
      multiple: true,
      title: 'Choose a mod file',
      filters: [{
        name: 'Mod Files',
        extensions: ['*']
      }]
    });

    document.querySelector("#install-status").textContent = "Installing mods...";
    if (selected) {
      const fileList = document.querySelector("#file-list");
      fileList.innerHTML = '';
      if (Array.isArray(selected)) {
        selected.forEach((file) => {
          const li = document.createElement("li");
          li.textContent = file;
          fileList.appendChild(li);
        });
        await installMods(selected);
      } else if (selected === null) {
        console.log('Nothing was selected');
      } else {
        const li = document.createElement("li");
        li.textContent = selected;
        fileList.appendChild(li);
        await installMods([selected]);
      }
    }
  });

  document.querySelector("#finish-setup").addEventListener("click", () => {
    alert("Setup Complete! Happy Modding!");
  });

  // Initially show the welcome screen
  showScreen("welcome-screen");
});
