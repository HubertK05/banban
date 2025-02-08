import type { Category, Tag } from "./interfaces/main";
import { SvelteMap } from 'svelte/reactivity';

export const categoriesRune: Record<number, Category> = $state({});
export const categoryTagsRune: Record<number, Tag> = $state({});
