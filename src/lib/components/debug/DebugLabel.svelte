<script lang="ts">
    import { scale } from "svelte/transition";
    import { appState } from "../../shared.svelte";

    interface Props {
        text: string;
    }

    let { text }: Props = $props();
</script>

{#if appState.isDebug}
    <b transition:scale class="variant-soft-warning rounded-md">{text}</b>
{/if}
