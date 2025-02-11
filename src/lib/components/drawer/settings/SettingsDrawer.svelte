<script lang="ts">
  import { drawerStore } from "@skeletonlabs/skeleton";
  import BackButton from "../BackButton.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import TagSettings from "../tag/TagSettings.svelte";
  import CategorySettings from "../tag/CategorySettings.svelte";
  import { categoriesRune } from "../../../shared.svelte";

  let categoryName: string = $state("");
  async function createCategory() {
    const res: {
      id: number;
      name: string;
      ordinal: number;
    } = await invoke("create_category", { name: categoryName });

    categoriesRune[res.id] = {
      name: res.name,
      tags: [],
      ord: res.ordinal,
    }
  }
</script>

<BackButton />
<div class="flex flex-col">
  <CategorySettings />
  <input class="input p-2" bind:value={categoryName} />
  <button onclick={createCategory} class="btn variant-ghost-primary"
    >Add new category</button
  >
</div>
