import { invoke } from "@tauri-apps/api/core";
import type { Category, Tag } from "./interfaces/main";
import { SvelteMap } from 'svelte/reactivity';
import { otherTags } from "./stores";

export const categoriesRune: Record<number, Category> = $state({});
export const categoryTagsRune: Record<number, Tag & { categoryId: number }> = $state({});

class IdTags {
    // we need both reactivity and functioning drag and drop at once.
    // $derived() rune on idTags confuses the drag and drop library.
    // $state() without explicit update is non-reactive.
    inner: { id: number, tag: Tag & { categoryId: number } }[][] = $state([])

    update = () => {
        this.inner = Object.entries(categoriesRune).map(([categoryId, category]) => {
            return category.tags.map(tagId => {
                const tag = categoryTagsRune[tagId];
                console.assert(tag !== undefined, "Category tag not found");
                return {id: tagId, tag};
            }).sort((idTagA, idTagB) => {
                return idTagA.tag.ord - idTagB.tag.ord;
            });
        })
    }
}

export const idTags = new IdTags();

export async function changeCategoryTagColor(
    newColor: string,
    tagId: number,
) {
    const tag = categoryTagsRune[tagId];
    await invoke("update_tag_color", {
        data: { categoryTagId: tagId, color: newColor.slice(1) },
    });
    tag.color = newColor;
    categoryTagsRune[tagId] = tag;
}

// export async function changeOtherTagColor(
//     newColor: string,
//     tagId: number,
// ) {
//     const tag = otherTagsRune[tagId];
//     await invoke("update_tag_color", {
//         data: { categoryTagId: tagId, color: newColor.slice(1) },
//     });
//     otherTagsRune[tagId] = {...tag, color: newColor};
// }
