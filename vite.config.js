import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
    plugins: [
        sveltekit(),
        wasm(),
        topLevelAwait(),
    ],
    server: {
        fs: {
            allow: ['src-web/style', 'dist-wasm'],
        },
    },
});