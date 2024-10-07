import { writable } from "svelte/store";
import { Store } from "@tauri-apps/plugin-store";

// Create tauri & svelte store(s)
export const recentActivity = writable([]);
let store = new Store("store.bin");

// Load activity from tauri store to component
export async function loadActivity() {
    const activityStore = await store.get("activity");

    if (activityStore?.value) {
        recentActivity.set(activityStore.value);
    }
}

export function logActivity(message, time = null) {
    if (!message || !message.length) {
        console.trace("Attempting to log activity without message!");
        return;
    }

    const entry = {
        time: time || new Date().toLocaleString(),
        message,
    };

    // Update the activity log in the writable store
    recentActivity.update((activities) => {
        const updatedActivities = [...activities, entry];
        
        // Persist the updated activities to Tauri store
        store.set("activity", { value: updatedActivities });
        
        return updatedActivities;
    });
}
