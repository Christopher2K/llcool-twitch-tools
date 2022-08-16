import { readable, derived } from 'svelte/store'

import { ErrorType } from '@app/error'
import { getUserData } from '@app/api'

export const user = readable(undefined, set => {
  getUserData()
    .then(userData => {
      set(userData)
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
