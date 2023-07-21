<script lang="ts">
    export let name: string;
</script>

<div class="container p-2">
    <h1 class="h1 variant-soft-surface p-2 rounded-md">{name}</h1>
    <slot />
</div>
