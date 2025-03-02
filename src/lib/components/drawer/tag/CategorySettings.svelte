<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { categoriesRune, idOtherTags, idTags, otherTagsRune } from "../../../shared.svelte";
    import type { Tag } from "../../../interfaces";
    import Category from "./Category.svelte";

    idTags.update();
    idOtherTags.update();
</script>

<h2 class="h2">Tag options</h2>

{#each Object.entries(categoriesRune).sort(([_1, catA], [_2, catB]) => {
    return catA.ord - catB.ord;
}) as [categoryId, category], categoryIdx}
    <Category categoryData={{ id: +categoryId, name: category.name, idx: categoryIdx, tags: category.tags }} />
{:else}
    <p>There are no categories</p>
{/each}
<Category />
