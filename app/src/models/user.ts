import type { UserApi } from '@app/api'

export type User = {
  id: string
  username: string
  twitchId: string
}

export function getUserFromApiObject(user: UserApi): User {
  return {
    id: user.id,
    username: user.username,
    twitchId: user.twitch_id,
  }
}
