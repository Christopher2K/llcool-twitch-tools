import { z } from 'zod'

import { PUBLIC_API_URL } from '$env/static/public'

import { handleHttpError } from './utils'

const botInfoApiValidator = z.object({
  name: z.string(),
  connected: z.boolean(),
  connectedToChat: z.boolean(),
  credentialsState: z.enum(['valid', 'invalid', 'notFound']),
})

type BotInfoApi = z.infer<typeof botInfoApiValidator>

export async function getBotInfo(fetchFn = fetch): Promise<BotInfoApi> {
  const response = await fetchFn(`${PUBLIC_API_URL}/bot/info`, {
    credentials: 'include'
  })

  if (!response.ok) {
    handleHttpError(response)
  }

  const json = await response.json()
  return await botInfoApiValidator.parseAsync(json)
}

export async function askBotToJoinChat(): Promise<void> {
  const response = await fetch(`${PUBLIC_API_URL}/bot/join`, {
    credentials: 'include',
  })

  if (!response.ok) {
    handleHttpError(response)
  }
}

export async function askBotToLeaveChat(): Promise<void> {
  const response = await fetch(`${PUBLIC_API_URL}/bot/leave`, {
    credentials: 'include',
  })

  if (!response.ok) {
    handleHttpError(response)
  }
}

export async function connectBotToTwitch(): Promise<void> {
  const response = await fetch(`${PUBLIC_API_URL}/bot/connect`, {
    credentials: 'include',
  })

  if (!response.ok) {
    handleHttpError(response)
  }
}
