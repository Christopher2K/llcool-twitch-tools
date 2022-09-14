import type { Handle, HandleFetch } from '@sveltejs/kit'

import { chatBotUsername, apiUrl, appUrl } from '@app/env'
import { getUserData } from '@app/api'

export const handle: Handle = async ({ event, resolve }) => {
  const cookie = event.request.headers.get('cookie')

  if (cookie) {
    try {
      let user = await getUserData(cookie)
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

export const handleFetch: HandleFetch = async ({ request, fetch }) => {
  if (request.url.startsWith(apiUrl)) {
    // Workaround: https://github.com/sveltejs/kit/issues/6608
    request.headers.set('origin', appUrl)
  }

  return fetch(request)
}
