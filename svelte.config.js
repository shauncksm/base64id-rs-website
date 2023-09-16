import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/kit/vite';

/** @type {import('@sveltejs/kit').Config} */
const config = {
    config: {
        preprocess: vitePreprocess(),
    },
    kit: {
        adapter: adapter({
            pages: 'dist',
            assets: 'dist',
            precompress: true,
            strict: true,
        }),
        files: {
            hooks: {
                client: "src-web/hooks.client",
                server: "src-web/hooks.server",
            },
            lib: "src-web/lib",
            params: "src-web/params",
            routes: "src-web/routes",
            serviceWorker: "src-web/service-worker",
            appTemplate: "src-web/app.html",
            errorTemplate: "src-web/error.html",
        },
    }
};

export default config;