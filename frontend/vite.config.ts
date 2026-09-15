import tailwindcss from '@tailwindcss/vite';
import adapter from '@sveltejs/adapter-static';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig, loadEnv } from 'vite';

export default defineConfig(({ mode }) => {
	// The whole project shares the .env file in the repo root. VITE_PORT is
	// read by the backend too, so the dev server and its proxy always agree.
	const env = loadEnv(mode, '..', 'VITE_');
	const port = Number(env.VITE_PORT) || 5173;

	return {
		envDir: '..',
		server: {
			port,
			strictPort: true,
			// Pages are normally opened through the backend, which proxies plain
			// HTTP to Vite but not WebSockets. Point the HMR client straight at
			// Vite's own port instead of the page's.
			hmr: { clientPort: port }
		},
		plugins: [
			tailwindcss(),
			sveltekit({
				compilerOptions: {
					// Force runes mode for the project, except for libraries. Can be removed in svelte 6.
					runes: ({ filename }) =>
						filename.split(/[/\\]/).includes('node_modules') ? undefined : true
				},
				// Client-rendered app: `ssr = false` in the root layout, and every
				// route falls back to index.html, which the backend serves.
				adapter: adapter({ fallback: 'index.html' })
			})
		]
	};
});
