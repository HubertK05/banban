<script lang="ts">
    import { scale } from "svelte/transition";
    import { isDebug } from "../../stores";

    interface Props {
        text: string;
    }

    let { text }: Props = $props();
</script>

{#if $isDebug}
    <b transition:scale class="variant-soft-warning rounded-md">{text}</b>
{/if}
