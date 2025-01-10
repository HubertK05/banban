<script lang="ts">
    import { shortcut } from "../../actions/shortcut";
    import { isDebug } from "../../stores";
</script>

<button
    onclick={() => {
        $isDebug = !$isDebug;
    }}
    use:shortcut={{ control: true, key: "d" }}
    class="btn variant-ghost-warning max-h-10"
    >Debug <br /><kbd class="kbd">⌘ + D</kbd></button
>
