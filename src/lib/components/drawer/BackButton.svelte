<script lang="ts">
    import { getDrawerStore } from "@skeletonlabs/skeleton";
    import { appState } from "../../shared.svelte";
    import Fa from "svelte-fa";
    import { faArrowLeft } from "@fortawesome/free-solid-svg-icons";

    const drawerStore = getDrawerStore();

    function goBack() {
        if (appState.previousDrawerTab === null) {
            drawerStore.close();
        }
        $drawerStore.id = appState.previousDrawerTab ?? undefined;
        appState.previousDrawerTab = null;
    }
</script>

<button aria-label="Go back" class="btn" onclick={goBack}><Fa icon={faArrowLeft} /></button>
