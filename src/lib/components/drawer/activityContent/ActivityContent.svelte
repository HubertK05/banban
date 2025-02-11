<script lang="ts">
    import ActivityBody from "./ActivityBody.svelte";
    import ActivityName from "./ActivityName.svelte";

    const { activityId }: { activityId: number } = $props();
</script>

<h2 class="h2">Content</h2>
<div class="flex flex-col place-content-center">
    <ActivityName {activityId} />
    <ActivityBody {activityId} />
</div>
