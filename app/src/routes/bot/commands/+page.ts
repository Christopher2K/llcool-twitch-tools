import { getAllGlobalCommands } from '@app/api'

import type { PageLoad } from "./$types"

export const load: PageLoad = async ({ fetch }) => {  
  const globalCommands = await getAllGlobalCommands(fetch)

  return {
    globalCommands
  }
}
