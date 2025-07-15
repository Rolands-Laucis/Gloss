// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
    onwarn: (warning, handler) => {
        // return; //suppress all svelte compiler warnings
        if (warning.code.startsWith('a11y-') || warning.code.startsWith('css-unused-selector')) {
            return;
        }
        handler(warning);
    },

    preprocess: [vitePreprocess({})],

    kit: {
        adapter: adapter(),
    },
};

export default config;
