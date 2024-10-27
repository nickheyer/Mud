
<script>
    import { onMount } from 'svelte';
    import { invoke } from "@tauri-apps/api/core";
    import { writable } from 'svelte/store';
    import { get, set, isNil, merge } from 'lodash';

    // Reactive stores for form data
    const formJson = writable(null);
    const formData = writable({});

    // Initialize form data from backend
    onMount(async () => {
        try {
            const json = await invoke('build_form_json');
            formJson.set(json);

            // Merge formData with settings default values
            const settings = get(json, 'settings', []);
            const initialData = settings.reduce((acc, { key, value, default: def }) => {
                set(acc, key, !isNil(value) ? value : !isNil(def) ? def : '');
                return acc;
            }, {});
            formData.set(initialData);

            console.log(`FormData: ${JSON.stringify(initialData, 4, 2)}`);
        } catch (error) {
            console.error('Error fetching form JSON:', error);
        }
    });

    // Update formData on input change
    const handleInputChange = (key, value) => formData.update(data => set(data, key, value));

    // Handle form submission
    const submitForm = async (event) => {
        event.preventDefault();
        const currentFormData = $formData;
        console.log(`SubmittedFormData: ${JSON.stringify(currentFormData, 4, 2)}`);

        try {
            await invoke('submit_form', { formData: currentFormData });
            console.log('Form submitted successfully');
        } catch (error) {
            console.error('Error submitting form:', error);
        }
    };
</script>

<link rel="stylesheet" href="/css/settings.css">

{#if $formJson}
    <div class="settings-container">
        <h1 class="settings-title">Settings</h1>

        {#if $formJson.settings?.length}
            <form on:submit={submitForm} class="settings-form">
                {#each $formJson.settings as { key, label, widget, options, description } (key)}
                    <div class="form-field">
                        <label for={key} class="form-label">{label}</label>

                        {#if widget === 'boolean'}
                            <input 
                                type="checkbox"
                                id={key}
                                bind:checked={$formData[key]}
                                on:change="{(e) => handleInputChange(key, e.target.checked)}"
                                class="form-checkbox"
                            />
                        {:else if widget === 'choice'}
                            <select
                                id={key}
                                bind:value={$formData[key]}
                                on:change="{(e) => handleInputChange(key, e.target.value)}"
                                class="form-select"
                            >
                                {#each options as option}
                                    <option value={option}>{option}</option>
                                {/each}
                            </select>
                        {:else if widget === 'text'}
                            <input
                                type="text"
                                id={key}
                                bind:value={$formData[key]}
                                placeholder={label}
                                class="form-input"
                            />
                        {/if}

                        <p class="form-description">{description}</p>
                    </div>
                {/each}

                <button type="submit" class="form-button">Apply Settings</button>
            </form>
        {:else}
            <p>No settings available to display.</p>
        {/if}

        {#if $formJson.metadata}
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
    </div>
{:else}
    <p>Loading settings...</p>
{/if}
