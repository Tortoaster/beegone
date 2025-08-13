/**
 * Used at the end of switch statements or if-else chains, to guarantee that all cases are handled.
 */
export const assertNever = (value: never) => {
  throw new Error(`unhandled variant: ${value}`)
}
