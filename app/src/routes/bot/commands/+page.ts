import { getAllGlobalCommands } from '@app/api'

import type { PageLoad } from "./$types"

export const load: PageLoad = async ({ fetch, depends }) => {  
  depends('globalCommand:all')
  const globalCommands = await getAllGlobalCommands(fetch)


  return {
    globalCommands
  }
}
