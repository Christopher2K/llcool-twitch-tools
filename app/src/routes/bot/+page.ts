import { getBotInfo } from '@app/api'

import type { PageLoad } from './$types'

export const load: PageLoad = async ({ fetch }) => {
  const botInfo = await getBotInfo(fetch)

  return {
    botInfo,
  };
}
