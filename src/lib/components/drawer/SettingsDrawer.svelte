<script lang="ts">
    import { drawerStore } from "@skeletonlabs/skeleton";
    import { previousDrawerTab } from "../../stores";
    import BackButton from "./BackButton.svelte";
    import { invoke } from "@tauri-apps/api";
    import TagSettings from "./tag/TagSettings.svelte";
    import CategorySettings from "./tag/CategorySettings.svelte";
</script>

<BackButton />
<div class="flex flex-col">
    <CategorySettings />
</div>
