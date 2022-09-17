import { z } from 'zod'

import { ErrorType } from '@app/error'
import { PUBLIC_API_URL } from '$env/static/public'

export const LOGIN_URL = `${PUBLIC_API_URL}/auth/login`
export const LOGOUT_URL = `${PUBLIC_API_URL}/auth/logout`

function handleHttpError(response: Response): never {
  let error = new Error(ErrorType.ServerError)

  switch (response.status) {
    case 401:
      error = new Error(ErrorType.Unauthorized)
      break
    case 403:
      error = new Error(ErrorType.Forbidden)
      break
  }

  throw error
}

const userApiValidator = z.object({
  id: z.string(),
  twitchId: z.string(),
  username: z.string(),
})

export type UserApi = z.infer<typeof userApiValidator>

export async function getUserData(
  cookie: string | undefined = undefined,
): Promise<UserApi> {
  const response = await fetch(
    `${PUBLIC_API_URL}/auth/me`,
    cookie == null
      ? {
          credentials: 'include',
        }
      : { headers: { cookie } },
  )

  if (!response.ok) {
    handleHttpError(response)
  }

  const json = await response.json()
  return await userApiValidator.parseAsync(json)
}

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
