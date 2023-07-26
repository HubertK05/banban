<script lang="ts">
    import {
        AppShell,
        Drawer,
        Modal,
        Tab,
        TabGroup,
        drawerStore,
    } from "@skeletonlabs/skeleton";
    import Board from "./lib/components/board/Board.svelte";
    import BoardDrawer from "./lib/components/drawer/BoardDrawer.svelte";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api";
    let tabSet: number = 0;

    onMount(async () => {
        await invoke("close_splashscreen");
    });
</script>

<div style="display: contents" class="h-full overflow-hidden">
    <BoardDrawer />
    <Modal />
    <AppShell>
        <TabGroup>
            <Tab bind:group={tabSet} name="tab1" value={0}>Tab 1</Tab>
            <Tab bind:group={tabSet} name="tab2" value={1}>Tab 2</Tab>
            <Tab bind:group={tabSet} name="tab3" value={2}>Tab 3</Tab>
            <svelte:fragment slot="panel">
                {#if tabSet === 0}
                    <Board />
                {:else if tabSet === 1}
                    <h1 class="h1">Tab 2</h1>
                {:else if tabSet === 2}
                    <h1 class="h1">Tab 3</h1>
                {/if}
            </svelte:fragment>
        </TabGroup>
    </AppShell>
</div>
