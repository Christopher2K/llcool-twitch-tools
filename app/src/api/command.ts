import { z } from 'zod'

import { PUBLIC_API_URL } from '$env/static/public'

import { handleHttpError } from './utils'

const userCommandValidator = z.object({
  id: z.string(),
  name: z.string(),
  message: z.string(),
})

const userCommandsValidator = z.array(userCommandValidator)

export type UserCommand = z.infer<typeof userCommandValidator>

export type CreateCommand = {
  name: string
  message: string
}

export type EditCommand = CreateCommand

export async function getAllUserCommands(fetchFn = fetch) {
  const response = await fetchFn(`${PUBLIC_API_URL}/command`, {
    credentials: 'include',
  })

  if (!response.ok) {
    handleHttpError(response)
  }

  const json = await response.json()
  return await userCommandsValidator.parseAsync(json)
}

export async function createUserCommand(newCommand: CreateCommand) {
  const response = await fetch(`${PUBLIC_API_URL}/command`, {
    method: 'POST',
    body: JSON.stringify(newCommand),
    credentials: 'include',
    headers: {
      'Content-Type': 'application/json',
    },
  })

  if (!response.ok) {
    handleHttpError(response)
  }

  const json = await response.json()
  return await userCommandValidator.parseAsync(json)
}

export async function editUserCommand(
  commandId: string,
  editedCommand: EditCommand,
) {
  const response = await fetch(`${PUBLIC_API_URL}/command/${commandId}`, {
    method: 'PATCH',
    body: JSON.stringify(editedCommand),
    credentials: 'include',
    headers: {
      'Content-Type': 'application/json',
    },
  })

  if (!response.ok) {
    handleHttpError(response)
  }

  const json = await response.json()
  return await userCommandValidator.parseAsync(json)
}

export async function deleteUserCommand(commandId: string) {
  const response = await fetch(`${PUBLIC_API_URL}/command/${commandId}`, {
    method: 'DELETE',
    credentials: 'include',
  })

  if (!response.ok) {
    handleHttpError(response)
  }
}
