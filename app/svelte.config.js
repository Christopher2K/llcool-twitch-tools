import adapter from '@sveltejs/adapter-auto'
import preprocess from 'svelte-preprocess'


/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://github.com/sveltejs/svelte-preprocess
  // for more information about preprocessors
  preprocess: preprocess({
    globalStyle: true,
    scss: {
      includePaths: ['./src/styles'],
    },
  }),

  kit: {
    alias: {
      '@app': 'src',
    },
    adapter: adapter(),
  },
}

export default config
