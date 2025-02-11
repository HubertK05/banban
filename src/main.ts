// Finally, your application's global stylesheet (sometimes labeled 'app.css')
import "./app.postcss";
// Standard styles
import "./styles.css";

import { computePosition, autoUpdate, offset, shift, flip, arrow } from "@floating-ui/dom";

import { storePopup } from "@skeletonlabs/skeleton";
storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

import App from "./App.svelte";
import { mount } from "svelte";

const app = mount(App, {
    target: document.getElementById("app"),
});

export default app;
