declare type Item = import("svelte-dnd-action").Item;
declare type DndEvent<ItemType = Item> = import("svelte-dnd-action").DndEvent<ItemType>;
declare namespace svelteHTML {
    interface HTMLAttributes<T> {
        onconsider?: (event: CustomEvent<DndEvent<ItemType>> & { target: EventTarget & T }) => void;
        onfinalize?: (event: CustomEvent<DndEvent<ItemType>> & { target: EventTarget & T }) => void;
    }
}
declare module '@fortawesome/pro-solid-svg-icons/index.es' {
    export * from '@fortawesome/pro-solid-svg-icons';
}