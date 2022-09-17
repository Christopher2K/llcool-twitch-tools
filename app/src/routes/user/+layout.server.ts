import { error, redirect } from '@sveltejs/kit'
import type { LayoutServerLoad } from './$types'

export const load: LayoutServerLoad = ({ locals }) => {
  if (!locals.user) {
    throw error(401)
  }

  if (locals.user && locals.isBotUser) {  
    throw redirect(307, '/bot')
  }
}
