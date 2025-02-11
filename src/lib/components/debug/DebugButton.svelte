<script lang="ts">
    import { shortcut } from "../../actions/shortcut";
    import { appState } from "../../shared.svelte";
</script>

<button
    onclick={() => {
        appState.isDebug = !appState.isDebug;
    }}
    use:shortcut={{ control: true, key: "d" }}
    class="btn variant-ghost-warning max-h-10">Debug <br /><kbd class="kbd">⌘ + D</kbd></button
>
