<script lang="ts">
    import Greet from "./lib/Greet.svelte";
    import { AppShell, Tab, TabGroup } from "@skeletonlabs/skeleton";
    let tabSet: number = 0;
</script>

<div style="display: contents" class="h-full overflow-hidden">
    <AppShell>
        <TabGroup>
            <Tab bind:group={tabSet} name="tab1" value={0}>Tab 1</Tab>
            <Tab bind:group={tabSet} name="tab2" value={1}>Tab 2</Tab>
            <Tab bind:group={tabSet} name="tab3" value={2}>Tab 3</Tab>
            <svelte:fragment slot="panel">
                {#if tabSet === 0}
                    <h1 class="h1">Tab 1</h1>
                {:else if tabSet === 1}
                    <h1 class="h1">Tab 2</h1>
                {:else if tabSet === 2}
                    <h1 class="h1">Tab 3</h1>
                {/if}
            </svelte:fragment>
        </TabGroup>
    </AppShell>
</div>
