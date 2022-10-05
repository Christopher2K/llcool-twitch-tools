import { getAllUserCommands } from '@app/api'

import type { PageLoad } from './$types'

export const load: PageLoad = async ({ fetch }) => {
  const userCommands = await getAllUserCommands(fetch)

  return {
    userCommands
  }
}
