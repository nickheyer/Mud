<script>
    import { recentActivity } from "$lib/stores/activityStore";
</script>

<div class="recent-activity">
    <h2>Recent Activity</h2>
    {#if $recentActivity.length > 0}
        <ul>
            {#each $recentActivity as activity}
                <li>{activity.time}: {activity.message}</li>
            {/each}
        </ul>
    {:else}
        <p>No recent activity.</p>
    {/if}
</div>