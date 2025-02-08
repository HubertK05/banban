import type { Category, Tag } from "./interfaces/main";

export const categoriesRune: Map<number, Category> = $state(new Map());
export const categoryTagsRune: Map<number, Tag> = $state(new Map());
