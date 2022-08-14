import { readable, derived } from 'svelte/store'

import { getUserData } from '@app/api'

export const user = readable(undefined, set => {
  getUserData().then(userData => {
    set(userData)
  })
})

export const authIsLoading = derived(user, $user => $user === undefined)

export const userIsLogged = derived(user, $user => $user != null)
