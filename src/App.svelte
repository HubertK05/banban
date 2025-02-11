<script lang="ts">
    import { AppShell, Drawer, Modal, Tab, TabGroup, drawerStore } from "@skeletonlabs/skeleton";
    import Board from "./lib/components/board/Board.svelte";
    import BoardDrawer from "./lib/components/drawer/BoardDrawer.svelte";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    import { Toast, toastStore } from "@skeletonlabs/skeleton";
    import type { ToastSettings } from "@skeletonlabs/skeleton";
    import { fetchAll } from "./lib/shared.svelte";

    let tabSet: number = 0;

    onMount(async () => {
        await fetchAll();
        await invoke("close_splashscreen");
    });
</script>

<div style="display: contents" class="h-full overflow-hidden">
    <BoardDrawer />
    <Modal />
    <Toast />
    <AppShell>
        <Board />
    </AppShell>
</div>
