import path from 'path'
import { readFileSync } from 'fs'
import { fileURLToPath } from 'url'

import adapter from '@sveltejs/adapter-auto'
import preprocess from 'svelte-preprocess'
import svg from '@poppanator/sveltekit-svg'

const __dirname = path.dirname(fileURLToPath(import.meta.url))

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
    vite: {
      server: {
        https: {
          cert: readFileSync(`${__dirname}/../localhost.pem`),
          key: readFileSync(`${__dirname}/../localhost-key.pem`),
        },
      },
      plugins: [svg()],
    },
    adapter: adapter(),
  },
}

export default config
