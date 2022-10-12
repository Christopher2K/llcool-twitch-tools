import type { Handle, HandleFetch } from '@sveltejs/kit'

import { CHAT_BOT_USERNAME } from '$env/static/private'

import { PUBLIC_APP_URL, PUBLIC_API_URL } from '$env/static/public'

import { getUserData } from '@app/api'

export const handle: Handle = async ({ event, resolve }) => {
  const cookie = event.request.headers.get('cookie')

  if (cookie) {
    try {
      let user = await getUserData(cookie)
      event.locals.user = user
      event.locals.isBotUser = user.username === CHAT_BOT_USERNAME
    } catch (e) {
      event.locals.user = undefined
    }
  } else {
    event.locals.user = undefined
  }

  return resolve(event)
}

export const handleFetch: HandleFetch = async ({ request, fetch, event }) => {
  const cookie = event.request.headers.get('cookie')

  if (request.url.startsWith(PUBLIC_API_URL)) {
    // Workaround: https://github.com/sveltejs/kit/issues/6608
    request.headers.set('origin', PUBLIC_APP_URL)
    if (cookie) {
      request.headers.set('cookie', cookie)
    }
  }

  return fetch(request)
}
