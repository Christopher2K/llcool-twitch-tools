import { readFileSync } from 'fs'
import { defineConfig } from 'vite'
import { sveltekit } from '@sveltejs/kit/vite' 
import svg from '@poppanator/sveltekit-svg'

export default defineConfig({
  server: {
    https: {
      cert: readFileSync(`${__dirname}/../localhost.pem`),
      key: readFileSync(`${__dirname}/../localhost-key.pem`),
    },
  },
  plugins: [sveltekit(), svg({})],
})
