export type Command = {
  id?: string
  name: string
  message: string
}

export type PatternCommand = {
  _type: 'Pattern'
  id?: string
  pattern: string
  message: string
}

export type PlainCommand = {
  _type: 'Plain'
  id?: string
  name: string
  message: string
}
