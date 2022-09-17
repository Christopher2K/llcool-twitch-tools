import { readFileSync } from 'fs'
import { defineConfig, type PluginOption, type UserConfig } from 'vite'
import { sveltekit } from '@sveltejs/kit/vite'
import svg from '@poppanator/sveltekit-svg'

export default defineConfig(({ command }) => {
  const commonConfig: UserConfig = {
    plugins: [sveltekit(), svg({}) as PluginOption],
  }

  if (command === 'build') {
    return commonConfig
  } else {
    return {
      ...commonConfig,
      server: {
        hmr: {
           path: '/_hmr',
           port: 443
        },

        https: {
          cert: readFileSync(`${__dirname}/certs/_wildcard.twitchtools.local.pem`),
          key: readFileSync(`${__dirname}/certs/_wildcard.twitchtools.local-key.pem`),
        },
      },
    }
  }
})
