<script lang="ts">
    interface Props {
        name: string;
        children?: import('svelte').Snippet;
    }

    let { name, children }: Props = $props();
</script>

<div class="container p-2 select-none">
    <h1 class="h1 variant-soft-surface p-2 rounded-md">{name}</h1>
    {@render children?.()}
</div>
