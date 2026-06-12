// Tracks WASM initialisation so the page can render before the engine is ready.

import { useEffect, useState } from 'react'

export type WasmStatus =
  | { status: 'initialising' }
  | { status: 'ready' }
  | { status: 'failed'; message: string }

export function useWasm(): WasmStatus {
  const [wasm, setWasm] = useState<WasmStatus>({ status: 'initialising' })

  useEffect(() => {
    // Holder object (not a bare `let`) so TS doesn't narrow the flag to the literal
    // `false` and flag the async closure's `!live.cancelled` as always-truthy.
    const live = { cancelled: false }
    void (async () => {
      try {
        const mod = await import('@pegin/wasm')
        await mod.default()
        if (!live.cancelled) setWasm({ status: 'ready' })
      } catch (error: unknown) {
        if (!live.cancelled) {
          const message = error instanceof Error ? error.message : String(error)
          setWasm({ status: 'failed', message })
        }
      }
    })()
    return () => {
      live.cancelled = true
    }
  }, [])

  return wasm
}
