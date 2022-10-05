import { ErrorType } from '@app/error'

export function handleHttpError(response: Response): never {
  let error = new Error(ErrorType.ServerError)

  switch (response.status) {
    case 401:
      error = new Error(ErrorType.Unauthorized)
      break
    case 403:
      error = new Error(ErrorType.Forbidden)
      break
  }

  throw error
}


