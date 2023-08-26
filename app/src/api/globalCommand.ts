import { z } from 'zod'

import { PUBLIC_API_URL } from '$env/static/public'

import { handleHttpError } from './utils'

function makeSpecificGlobalCommandValidator<T extends z.ZodTypeAny>(validator: T) {
  return z.object({
    id: z.string(),
    commandDefinition: validator,
  })
}

const patternCommandDefinitionValidator = z.object({
  _type: z.literal('pattern'),
  pattern: z.string().trim(),
  message: z.string().min(1),
})
const globalPatternCommandValidator = makeSpecificGlobalCommandValidator(
  patternCommandDefinitionValidator,
)
export type GlobalPatternCommand = z.infer<typeof globalPatternCommandValidator>
export type PatternCommandDefinition = z.infer<typeof patternCommandDefinitionValidator>

const plainCommandDefinitionValidator = z.object({
  _type: z.literal('plain'),
  name: z.string().trim(),
  message: z.string().min(1),
})
const globalPlainCommandValidator = makeSpecificGlobalCommandValidator(
  plainCommandDefinitionValidator,
)
export type GlobalPlainCommand = z.infer<typeof globalPlainCommandValidator>
export type PlainCommandDefinition = z.infer<typeof plainCommandDefinitionValidator>

const commandDefinitionValidator = z.union([
  patternCommandDefinitionValidator,
  plainCommandDefinitionValidator,
])

const globalCommandValidator = makeSpecificGlobalCommandValidator(
  commandDefinitionValidator,
)
const globalCommandsValidator = z.array(globalCommandValidator)
export type GlobalCommand = z.infer<typeof globalCommandValidator>
export type GlobalCommandType = GlobalCommand['commandDefinition']['_type']

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
      'Content-Type': 'application/json',
    },
  })

  if (!response.ok) {
    handleHttpError(response)
  }

  const json = await response.json()

  return await globalCommandValidator.parseAsync(json)
}

export async function updateGlobalCommand(
  commandId: string,
  definition: GlobalCommand['commandDefinition'],
) {
  const response = await fetch(`${PUBLIC_API_URL}/global_command/${commandId}`, {
    method: 'PATCH',
    credentials: 'include',
    body: JSON.stringify(definition),
    headers: {
      'Content-Type': 'application/json',
    },
  })

  if (!response.ok) {
    handleHttpError(response)
  }

  const json = await response.json()

  return await globalCommandValidator.parseAsync(json)
}
