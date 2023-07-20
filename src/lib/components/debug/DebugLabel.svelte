<script lang="ts">
    import { scale } from "svelte/transition";
    import { isDebug } from "../../stores";

    export let text: string;
</script>

{#if $isDebug}
    <b transition:scale class="variant-soft-warning rounded-md">{text}</b>
{/if}
