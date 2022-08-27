import { z } from 'zod'

import { ErrorType } from '@app/error'
import { apiUrl } from '@app/env'

export const LOGIN_URL = `${apiUrl}/auth/login`
export const LOGOUT_URL = `${apiUrl}/auth/logout`

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
    `${apiUrl}/auth/me`,
    cookie == null
      ? {
          credentials: 'include',
        }
      : { headers: { cookie } },
  )

  if (response.ok) {
    const json = await response.json()
    return await userApiValidator.parseAsync(json)
  } else {
    const error =
      response.status === 401
        ? new Error(ErrorType.Unauthorized)
        : new Error(ErrorType.ServerError)
    throw error
  }
}

const botInfoApiValidator = z.object({
  name: z.string(),
  connected: z.boolean(),
  connectedToChat: z.boolean(),
  credentialsState: z.enum(['valid', 'invalid', 'notFound']),
})

type BotInfoApi = z.infer<typeof botInfoApiValidator>

export async function getBotInfo(
  fetchFn = fetch,
  cookie: string | undefined = undefined,
): Promise<BotInfoApi> {
  const response = await fetchFn(
    `${apiUrl}/bot/info`,
    cookie == null
      ? {
          credentials: 'include',
        }
      : { headers: { cookie } },
  )

  if (response.ok) {
    const json = await response.json()
    return await botInfoApiValidator.parseAsync(json)
  } else {
    const error =
      response.status === 401
        ? new Error(ErrorType.Unauthorized)
        : new Error(ErrorType.ServerError)
    throw error
  }
}
