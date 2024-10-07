<script>
    import { onMount } from 'svelte';
    import { invoke } from "@tauri-apps/api/core";
    import { writable } from 'svelte/store';

    // Reactive stores for form data
    let formJson = writable(null);
    let formData = writable({});

    // Fetch form data from backend
    onMount(async () => {
        try {
            const json = await invoke('build_form_json', {});
            console.log(`Form: ${JSON.stringify(json, null, 2)}`);
            formJson.set(json);

            // Initialize formData with default values
            if (json.settings && Array.isArray(json.settings)) {
                const data = {};
                json.settings.forEach(setting => {
                    data[setting.key] = setting.default || '';
                });
                formData.set(data);
            }

            console.log(`FormData: ${JSON.stringify($formData, null, 2)}`);
        } catch (error) {
            console.error(`Error fetching form JSON: ${error}`);
        }
    });

    // Update functions for different input types
    function handleInputChange(key, value) {
        formData.update(data => {
            data[key] = value;
            return data;
        });
    }
</script>

<link rel="stylesheet" href="/css/settings.css">

<div class="settings-container">
    {#if $formJson}
        {#if $formJson.settings}
            <h1 class="settings-title">Settings</h1>
            <form method="POST" class="settings-form">
                {#each $formJson.settings as setting (setting.key)}
                    <div class="form-field">
                        <label for={setting.key} class="form-label">{setting.label}</label>
                        
                        {#if setting.widget === 'boolean'}
                            <input 
                                type="checkbox"
                                id={setting.key}
                                checked={$formData[setting.key]} 
                                on:change="{(e) => handleInputChange(setting.key, e.target.checked)}"
                                class="form-checkbox"
                            />
                        {:else if setting.widget === 'choice'}
                            <select
                                id={setting.key}
                                value={$formData[setting.key]}
                                on:change="{(e) => handleInputChange(setting.key, e.target.value)}"
                                class="form-select"
                            >
                                {#each setting.options as option}
                                    <option value={option}>{option}</option>
                                {/each}
                            </select>
                        {:else if setting.widget === 'text'}
                            <input
                                type="text"
                                id={setting.key}
                                value={$formData[setting.key]}
                                on:input="{(e) => handleInputChange(setting.key, e.target.value)}"
                                placeholder={setting.label} 
                                class="form-input"
                            />
                        {/if}

                        <p class="form-description">{setting.description}</p>
                    </div>
                {/each}

                <button type="submit" class="form-button">Apply Settings</button>
            </form>
        {/if}
        {#if $formJson.metadata}
            <!-- Render the metadata view -->
            <h1 class="settings-title">{$formJson.metadata.name}</h1>
            <p>{$formJson.metadata.description}</p>
            <p>Version: {$formJson.metadata.version}</p>
            <p>Last Updated: {$formJson.metadata.updated}</p>
            <h2 class="form-subtitle">Supported Games:</h2>
            <ul class="games-list">
                {#each $formJson.games as game}
                    <li>
                        <strong>{game.name}</strong> - {game.description} (v{game.version})
                    </li>
                {/each}
            </ul>
        {/if}
    {:else}
        <p>Loading settings...</p>
    {/if}
</div>
