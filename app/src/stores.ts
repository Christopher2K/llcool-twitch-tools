import { readable, derived } from 'svelte/store'

import { ErrorType } from '@app/error'
import { type User, getUserFromApiObject } from '@app/models'
import { getUserData } from '@app/api'

export const user = readable<User | null | undefined>(undefined, set => {
  getUserData()
    .then(userData => {
      set(getUserFromApiObject(userData))
    })
    .catch(error => {
      if (error?.message === ErrorType.Unauthorized) {
        set(null)
      } else {
        // TODO: Dispatch a global error here
      }
    })
})

export const authIsLoading = derived(user, $user => $user === undefined)

export const userIsLogged = derived(user, $user => $user != null)
