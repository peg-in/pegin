import { describe, expect, it } from 'vitest'
import { createServer, type IncomingMessage, type ServerResponse } from 'node:http'

import { createPeginAuthMiddleware } from '../src/server/pegin-auth.handler.js'

describe('pegin auth handler cookies', () => {
  it('sets only pegin_session after successful login', async () => {
    const wasmPath = '../../../crates/pegin-wasm/pkg-node/pegin_wasm.js'
    const { deriveWalletKeys, mintJwt, signChallenge } = await import(wasmPath)
    const { deterministicTestPhrase } = await import(
      '../../../crates/pegin-wasm/test-support/deterministic-phrase.mjs'
    )
    const TEST_DID = 'did:chia:1gt7hae94wd0c33v07k4kkwgjy9jjtcnzhwvl5yxuvmj28mqsnsjqvgw9uu'

    const handler = createPeginAuthMiddleware()
    const server = createServer((req: IncomingMessage, res: ServerResponse) => {
      void handler(req, res, () => {
        res.statusCode = 404
        res.end()
      })
    })

    await new Promise<void>((resolve) => server.listen(0, resolve))
    const port = (server.address() as { port: number }).port
    const origin = `http://127.0.0.1:${port}`

    const nonceRes = await fetch(`${origin}/nonce`, {
      method: 'POST',
      headers: { Origin: origin },
    })
    const { loginId, nonce, aud } = (await nonceRes.json()) as {
      loginId: string
      nonce: string
      aud: string
    }
    expect(nonceRes.headers.getSetCookie()).toEqual([])
    expect(loginId).toMatch(/^[0-9a-f]{32}$/)

    const keys = deriveWalletKeys(deterministicTestPhrase())
    const jwt = mintJwt(keys, TEST_DID, aud, 3600, nonce)
    const challengeSig = signChallenge(keys, nonce)
    keys.free()

    const sessionRes = await fetch(`${origin}/session`, {
      method: 'POST',
      headers: { Origin: origin, 'content-type': 'application/json' },
      body: JSON.stringify({ loginId, jwt, challengeSig }),
    })
    expect(sessionRes.status).toBe(200)
    const cookies = sessionRes.headers.getSetCookie()
    expect(cookies.some((c) => c.startsWith('pegin_session='))).toBe(true)
    expect(
      cookies.some((c) => c.startsWith('pegin_login_nonce=') && !c.includes('Max-Age=0')),
    ).toBe(false)

    server.close()
  })
})
