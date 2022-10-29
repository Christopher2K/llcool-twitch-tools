import { z } from 'zod'

import { PUBLIC_API_URL } from '$env/static/public'

import { handleHttpError } from './utils'

const patternCommandDefinitionValidator = z.object({
  _type: z.literal('pattern'),
  pattern: z.string(),
  message: z.string(),
})

const plainCommandDefinitionValidator = z.object({
  _type: z.literal('plain'),
  name: z.string(),
  message: z.string(),
})

const commandDefinitionValidator = z.union([
  patternCommandDefinitionValidator,
  plainCommandDefinitionValidator,
])

const globalCommandValidator = z.object({
  id: z.string(),
  commandDefinition: commandDefinitionValidator,
})

export type GlobalCommand = z.infer<typeof globalCommandValidator>

export type GlobalCommandType = GlobalCommand['commandDefinition']['_type']

const globalCommandsValidator = z.array(globalCommandValidator)

export async function getAllGlobalCommands(fetchFn = fetch) {
  const response = await fetchFn(`${PUBLIC_API_URL}/global_command`, {
    credentials: 'include',
  })

  if (!response.ok) {
    handleHttpError(response)
  }

  const json = await response.json()

  return await globalCommandsValidator.parseAsync(json)
}

export async function deleteGlobalCommand(commandId: string) {
  const response = await fetch(`${PUBLIC_API_URL}/global_command/${commandId}`, {
    method: 'DELETE',
    credentials: 'include',
  })

  if (!response.ok) {
    handleHttpError(response)
  }
}

export async function createGlobalCommand(
  definition: GlobalCommand['commandDefinition'],
) {
  const response = await fetch(`${PUBLIC_API_URL}/global_command`, {
    method: 'POST',
    credentials: 'include',
    body: JSON.stringify(definition),
    headers: {
      'Content-Type': 'application/json'
    }
  })

  if (!response.ok) {
    handleHttpError(response)
  }

  const json = await response.json()

  return await globalCommandValidator.parseAsync(json)
}
