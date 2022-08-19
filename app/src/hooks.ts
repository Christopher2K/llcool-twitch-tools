import type { Handle } from '@sveltejs/kit'

import { chatBotUsername } from '@app/env'
import { getUserData } from '@app/api'
import { getUserFromApiObject } from '@app/models'

export const handle: Handle = async ({ event, resolve }) => {
  const cookie = event.request.headers.get('cookie')

  if (cookie) {
    try {
      let user = getUserFromApiObject(await getUserData(cookie))
      event.locals.user = user
      event.locals.isBotUser = user.username === chatBotUsername
    } catch (e) {
      event.locals.user = undefined
    }
  } else {
    event.locals.user = undefined
  }

  return resolve(event)
}
