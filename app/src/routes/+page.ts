import { getBotInfo } from '@app/api'

import type { PageLoad } from './$types'

export const load: PageLoad = async ({ fetch }) => {
  try {
    const botInfo = await getBotInfo(fetch)

    return {
      botInfo,
    }
  } catch (e) {
    return {}
  }
}
