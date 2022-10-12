import { z } from 'zod'

import { PUBLIC_API_URL } from '$env/static/public'

import { handleHttpError } from './utils'

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

