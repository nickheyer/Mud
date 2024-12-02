import { writable } from "svelte/store";
import { LazyStore } from '@tauri-apps/plugin-store';

// SVELTE + TAURI STORE
export const recentActivity = writable([]);
let store = new LazyStore("store.bin");

// LOAD FROM TAURI STORE
export async function loadActivity() {
    const activityStore = await store.get("activity");

    if (activityStore?.value) {
        recentActivity.set(activityStore.value);
    }
}

// UPDATE ACTIVITY LOG IN SVELTE STORE
export function logActivity(message, time = null) {
    if (!message || !message.length) {
        console.trace("Attempting to log activity without message!");
        return;
    }

    const entry = {
        time: time || new Date().toLocaleString(),
        message,
    };

    recentActivity.update((activities) => {
        const updatedActivities = [...activities, entry];
        
        // RESET TAURI STORE TO SAME AS SVELTE
        store.set("activity", { value: updatedActivities });
        
        return updatedActivities;
    });
}
