import { error } from '@sveltejs/kit'

import { getBotInfo } from '@app/api'

import type { PageServerLoad } from './$types'

export const prerender = true

export const load: PageServerLoad = async ({ request }) => {
  const cookie = request.headers.get('cookie')

  if (!cookie) throw error(401)

  const botInfo = await getBotInfo(fetch, cookie)

  return {
    botInfo,
  }
}
