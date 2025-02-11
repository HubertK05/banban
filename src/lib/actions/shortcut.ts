export const shortcut = (
    node: HTMLElement,
    params: { alt?: boolean; shift?: boolean; control?: boolean; key: string; callback?: Function },
) => {
    let handler;
    const removeHandler = () => window.removeEventListener("keydown", handler),
        setHandler = () => {
            removeHandler();
            if (!params) return;
            handler = (e: KeyboardEvent) => {
                if (
                    !!params.alt != e.altKey ||
                    !!params.shift != e.shiftKey ||
                    !!params.control != (e.ctrlKey || e.metaKey) ||
                    params.key != e.key
                )
                    return;
                e.preventDefault();
                params.callback ? params.callback() : node.click();
            };
            window.addEventListener("keydown", handler);
        };
    setHandler();
    return {
        update: setHandler,
        destroy: removeHandler,
    };
};
