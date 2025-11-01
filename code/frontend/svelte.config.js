import adapter from "@sveltejs/adapter-static";
//import { vitePreprocess } from '@sveltejs/kit';

const config = {
	//	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter({
			fallback: "index.html", // Needed for SPA-style routing
		}),

		// no "ssr" key here anymore!
		prerender: {
			entries: [], // disable prerendering
		},
	},
};

export default config;
