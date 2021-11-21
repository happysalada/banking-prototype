import preprocess from "svelte-preprocess";
import adapter from '@sveltejs/adapter-static';

/** @type {import('@sveltejs/kit').Config} */
export default {
  extensions: [".svelte"],
  // Consult https://github.com/sveltejs/svelte-preprocess
  // for more information about preprocessors
  preprocess: [
    preprocess({
      postcss: true,
    }),
  ],
  kit: {
    adapter: adapter({ fallback: "index.html" }),
    prerender: {
      enabled: false,
    },
    ssr: false,
    // hydrate the <div id="svelte"> element in src/app.html
    target: "#svelte",
  },
};
