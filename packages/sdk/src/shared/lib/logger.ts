/**
 * SDK logger — errors only by default, raisable for dev support.
 * SDK ships this minimal wrapper with the same level semantics (see wiki: logging-strategy).
 * The single console sink for the whole SDK; everything else must log through it.
 */

type LogLevel = 'error' | 'warn' | 'info' | 'debug'

const LEVEL_ORDER: Record<LogLevel, number> = { error: 0, warn: 1, info: 2, debug: 3 }

let threshold: LogLevel = 'error'

export function setLogLevel(level: LogLevel): void {
  threshold = level
}

function emit(level: LogLevel, message: string, meta?: unknown): void {
  if (LEVEL_ORDER[level] > LEVEL_ORDER[threshold]) return
  const line = `[pegin] ${message}`
  // eslint-disable-next-line no-console -- the SDK's single console sink
  const sink = level === 'error' ? console.error : level === 'warn' ? console.warn : console.info
  if (meta === undefined) sink(line)
  else sink(line, meta)
}

export const logger = {
  error: (message: string, meta?: unknown): void => {
    emit('error', message, meta)
  },
  warn: (message: string, meta?: unknown): void => {
    emit('warn', message, meta)
  },
  info: (message: string, meta?: unknown): void => {
    emit('info', message, meta)
  },
  debug: (message: string, meta?: unknown): void => {
    emit('debug', message, meta)
  },
}
