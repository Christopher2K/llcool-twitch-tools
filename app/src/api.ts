import { browser } from '$app/env'
import { apiUrl } from './env'

export const LOGIN_URL = `${apiUrl}/auth/login`
export const LOGOUT_URL = `${apiUrl}/auth/logout`

export async function getUserData() {
  // if (!browser) return undefined
  const response = await fetch(`${apiUrl}/auth/me`, {
    credentials: 'include',
  })

  if (response.ok) {
    return await response.json()
  } else {
    return null
  }
}
