// Tracks WASM initialisation so the page can render before the engine is ready.

import { useEffect, useState } from 'react'
import { loadPeginWasm } from '../../shared/api/pegin-wasm.js'

export type WasmStatus =
  | { status: 'initialising' }
  | { status: 'ready' }
  | { status: 'failed'; message: string }

export function useWasm(): WasmStatus {
  const [wasm, setWasm] = useState<WasmStatus>({ status: 'initialising' })

  useEffect(() => {
    let cancelled = false
    loadPeginWasm()
      .then(() => {
        if (!cancelled) setWasm({ status: 'ready' })
      })
      .catch((error: unknown) => {
        if (!cancelled) {
          const message = error instanceof Error ? error.message : String(error)
          setWasm({ status: 'failed', message })
        }
      })
    return () => {
      cancelled = true
    }
  }, [])

  return wasm
}
