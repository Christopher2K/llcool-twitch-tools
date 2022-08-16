import { z } from 'zod'

import { ErrorType } from '@app/error'
import { apiUrl } from '@app/env'

export const LOGIN_URL = `${apiUrl}/auth/login`
export const LOGOUT_URL = `${apiUrl}/auth/logout`

const userApiValidator = z.object({
  id: z.string(),
  twitch_id: z.string(),
  username: z.string(),
})

export type UserApi = z.infer<typeof userApiValidator>

export async function getUserData(): Promise<UserApi> {
  // if (!browser) return undefined
  const response = await fetch(`${apiUrl}/auth/me`, {
    credentials: 'include',
  })

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
