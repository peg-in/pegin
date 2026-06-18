// PEGIN Signer — setup tabs, sign-in, dashboard.

import { listen } from '@tauri-apps/api/event'
import { useCallback, useEffect, useState } from 'react'
import type { SubmitEvent } from 'react'
import {
  addPasskeyBackup,
  approveSignRequest,
  copyMnemonic,
  generateSeed,
  getMnemonic,
  getPendingSignRequest,
  listPasskeyBackups,
  lockVault,
  openSignRequest,
  rejectSignRequest,
  resyncPasskeyToRelay,
  sealSeed,
  sessionStatus,
  unlockVault,
  vaultStatus,
  type PasskeyBackupInfo,
  type PendingSignRequest,
  type VaultStatus,
} from '../shared/ipc/index.js'
import { errorText } from '../shared/lib/index.js'
import type { Notice } from '../shared/types/notice.js'
import type { SetupMode } from '../shared/ui/SetupTabs.js'
import { DashboardPage } from '../pages/dashboard/index.js'
import { SetupPage } from '../pages/setup/index.js'
import { UnlockPage } from '../pages/unlock/index.js'
import { SignRequestOverlay } from '../widgets/SignRequestOverlay/index.js'
import { Shell } from './Shell.js'

type Screen = 'loading' | 'setup' | 'unlock' | 'dashboard'
type WordCount = 12 | 24

const SCREEN_LABELS: Record<Screen, string> = {
  loading: '…',
  setup: 'setup',
  unlock: 'sign in',
  dashboard: 'dashboard',
}

export function App() {
  const [screen, setScreen] = useState<Screen>('loading')
  const [setupMode, setSetupMode] = useState<SetupMode>('create')
  const [status, setStatus] = useState<VaultStatus | null>(null)
  const [passkeys, setPasskeys] = useState<PasskeyBackupInfo[]>([])
  const [name, setName] = useState('')
  const [wordCount, setWordCount] = useState<WordCount>(12)
  const [created, setCreated] = useState('')
  const [seedInput, setSeedInput] = useState('')
  const [revealed, setRevealed] = useState(false)
  const [passkeyLabel, setPasskeyLabel] = useState('')
  const [busy, setBusy] = useState(false)
  const [notice, setNotice] = useState<Notice>(null)
  const [copied, setCopied] = useState(false)
  const [seedWords, setSeedWords] = useState<string[]>([])
  const [seedRevealed, setSeedRevealed] = useState(false)
  const [seedCopied, setSeedCopied] = useState(false)
  const [showSetupHint, setShowSetupHint] = useState(false)
  const [pendingSign, setPendingSign] = useState<PendingSignRequest | null>(null)

  const refreshPendingSign = useCallback(async () => {
    try {
      setPendingSign(await getPendingSignRequest())
    } catch {
      setPendingSign(null)
    }
  }, [])

  const loadPasskeys = useCallback(async (sealed: boolean, passkeyCount = 0) => {
    if (!sealed) {
      setPasskeys([])
      return
    }
    try {
      setPasskeys(await listPasskeyBackups())
    } catch (err: unknown) {
      setPasskeys([])
      if (passkeyCount > 0) {
        setNotice({ kind: 'error', text: errorText(err) })
      }
    }
  }, [])

  const loadSeedPhrase = useCallback(async () => {
    try {
      const phrase = await getMnemonic()
      setSeedWords(phrase.trim().split(/\s+/))
    } catch {
      setSeedWords([])
    }
  }, [])

  const refresh = useCallback(async () => {
    const [vault, session] = await Promise.all([vaultStatus(), sessionStatus()])
    setStatus(vault)
    await loadPasskeys(vault.sealed, vault.passkeyCount)
    if (session.unlocked) {
      await loadSeedPhrase()
      setScreen('dashboard')
      await refreshPendingSign()
    } else if (vault.sealed) {
      setSeedWords([])
      setSeedRevealed(false)
      setScreen((current) => (current === 'setup' ? 'setup' : 'unlock'))
    } else {
      setSeedWords([])
      setSeedRevealed(false)
      setScreen('setup')
    }
  }, [loadPasskeys, loadSeedPhrase, refreshPendingSign])

  useEffect(() => {
    refresh().catch((err: unknown) => {
      setNotice({ kind: 'error', text: errorText(err) })
      setScreen('setup')
    })
  }, [refresh])

  useEffect(() => {
    const unsubs: Array<Promise<() => void>> = []
    unsubs.push(
      listen<{ requestId: string; relayUrl: string }>('open-sign-request', (event) => {
        void openSignRequest(event.payload.requestId, event.payload.relayUrl)
          .then(() => refreshPendingSign())
          .catch((err: unknown) => {
            setNotice({ kind: 'error', text: errorText(err) })
          })
      }),
    )
    unsubs.push(
      listen<PendingSignRequest>('sign-request-pending', (event) => {
        setPendingSign(event.payload)
      }),
    )
    unsubs.push(
      listen('sign-request-done', () => {
        setPendingSign(null)
        setNotice({ kind: 'ok', text: 'sign request completed' })
      }),
    )
    return () => {
      void Promise.all(unsubs).then((drops) => {
        for (const drop of drops) drop()
      })
    }
  }, [refreshPendingSign])

  const resetSetupFields = () => {
    setCreated('')
    setSeedInput('')
    setRevealed(false)
    setNotice(null)
  }

  const goToSetup = (mode: SetupMode) => {
    resetSetupFields()
    setSetupMode(mode)
    setScreen('setup')
  }

  const phraseForCopy = (created || seedInput).trim().toLowerCase().replace(/\s+/g, ' ')

  const onGenerate = async () => {
    setNotice(null)
    try {
      setCreated(await generateSeed(wordCount))
    } catch (err: unknown) {
      setNotice({ kind: 'error', text: errorText(err) })
    }
  }

  const onCopyPhrase = async () => {
    if (!phraseForCopy) return
    try {
      await navigator.clipboard.writeText(phraseForCopy)
      setCopied(true)
      window.setTimeout(() => {
        setCopied(false)
      }, 2000)
    } catch (err: unknown) {
      setNotice({ kind: 'error', text: errorText(err) })
    }
  }

  const onSeal = async (
    event: SubmitEvent<HTMLFormElement>,
    mnemonic: string,
    preservePasskeys = false,
  ) => {
    event.preventDefault()
    if (busy || !name.trim() || !mnemonic.trim()) return
    setBusy(true)
    setNotice(null)
    try {
      await sealSeed(mnemonic.trim().toLowerCase().replace(/\s+/g, ' '), name.trim(), {
        preservePasskeys,
      })
      resetSetupFields()
      setSeedRevealed(true)
      setShowSetupHint(true)
      await refresh()
    } catch (err: unknown) {
      setNotice({ kind: 'error', text: errorText(err) })
    } finally {
      setBusy(false)
    }
  }

  const onUnlock = async () => {
    setBusy(true)
    setNotice(null)
    try {
      await unlockVault()
      await refresh()
    } catch (err: unknown) {
      setNotice({ kind: 'error', text: errorText(err) })
    } finally {
      setBusy(false)
    }
  }

  const onLock = async () => {
    await lockVault()
    setPasskeyLabel('')
    setSeedWords([])
    setSeedRevealed(false)
    setShowSetupHint(false)
    setNotice(null)
    await refresh()
  }

  const onCopyMnemonic = async () => {
    setBusy(true)
    setNotice(null)
    try {
      await copyMnemonic()
      setSeedCopied(true)
      window.setTimeout(() => {
        setSeedCopied(false)
      }, 2000)
    } catch (err: unknown) {
      setNotice({ kind: 'error', text: errorText(err) })
    } finally {
      setBusy(false)
    }
  }

  const onAddPasskey = async (event: SubmitEvent<HTMLFormElement>) => {
    event.preventDefault()
    if (busy || !passkeyLabel.trim()) return
    setBusy(true)
    setNotice(null)
    try {
      const backup = await addPasskeyBackup(passkeyLabel.trim())
      setPasskeyLabel('')
      setShowSetupHint(false)
      if (backup.relaySynced === false) {
        setNotice({
          kind: 'error',
          text: `passkey "${backup.label}" saved locally but not synced — start demo-web, then tap sync for web login`,
        })
      } else {
        setNotice({ kind: 'ok', text: `passkey registered: ${backup.label}` })
      }
      await refresh()
    } catch (err: unknown) {
      setNotice({ kind: 'error', text: errorText(err) })
    } finally {
      setBusy(false)
    }
  }

  const onResyncPasskey = async (credentialId: string) => {
    setBusy(true)
    setNotice(null)
    try {
      await resyncPasskeyToRelay(credentialId)
      setNotice({ kind: 'ok', text: 'passkey synced to auth relay for web login' })
    } catch (err: unknown) {
      setNotice({ kind: 'error', text: errorText(err) })
    } finally {
      setBusy(false)
    }
  }

  const onApproveSign = async () => {
    setBusy(true)
    setNotice(null)
    try {
      await approveSignRequest()
      setPendingSign(null)
    } catch (err: unknown) {
      setNotice({ kind: 'error', text: errorText(err) })
    } finally {
      setBusy(false)
    }
  }

  const onRejectSign = async () => {
    setBusy(true)
    setNotice(null)
    try {
      await rejectSignRequest()
      setPendingSign(null)
      setNotice({ kind: 'ok', text: 'sign request rejected' })
    } catch (err: unknown) {
      setNotice({ kind: 'error', text: errorText(err) })
    } finally {
      setBusy(false)
    }
  }

  const showLogo = screen === 'setup' || screen === 'loading'

  return (
    <Shell
      screenLabel={SCREEN_LABELS[screen]}
      statusHint={
        screen === 'dashboard'
          ? 'passwordless · passkeys · relay'
          : 'device-sealed · no passwords'
      }
      showLogo={showLogo}
    >
      {pendingSign && (
        <SignRequestOverlay
          request={pendingSign}
          canSign={screen === 'dashboard'}
          busy={busy}
          onApprove={() => void onApproveSign()}
          onReject={() => void onRejectSign()}
        />
      )}

      {screen === 'loading' && <p className="tui-msg">loading…</p>}

      {screen === 'setup' && (
        <SetupPage
          mode={setupMode}
          name={name}
          wordCount={wordCount}
          created={created}
          seedInput={seedInput}
          revealed={revealed}
          busy={busy}
          copied={copied}
          replacingVault={status?.sealed ?? false}
          showBack={status?.sealed ?? false}
          onModeChange={(mode) => {
            setSetupMode(mode)
            setCreated('')
            setSeedInput('')
            setRevealed(false)
            setNotice(null)
          }}
          onBack={() => {
            resetSetupFields()
            setScreen('unlock')
          }}
          onNameChange={setName}
          onWordCountChange={(count) => {
            setWordCount(count)
            setCreated('')
          }}
          onGenerate={() => void onGenerate()}
          onSeedChange={setSeedInput}
          onRevealedChange={setRevealed}
          onCopyPhrase={() => void onCopyPhrase()}
          onSealCreate={(e) => void onSeal(e, created, false)}
          onSealImport={(e) => void onSeal(e, seedInput, status?.sealed ?? false)}
        />
      )}

      {screen === 'unlock' && (
        <UnlockPage
          status={status}
          passkeys={passkeys}
          busy={busy}
          onUnlock={() => void onUnlock()}
          onSetup={goToSetup}
        />
      )}

      {screen === 'dashboard' && (
        <DashboardPage
          status={status}
          passkeys={passkeys}
          seedWords={seedWords}
          seedRevealed={seedRevealed}
          seedCopied={seedCopied}
          passkeyLabel={passkeyLabel}
          busy={busy}
          showSetupHint={showSetupHint}
          onPasskeyLabelChange={setPasskeyLabel}
          onAddPasskey={(e) => void onAddPasskey(e)}
          onResyncPasskey={(id) => void onResyncPasskey(id)}
          onSeedRevealChange={setSeedRevealed}
          onCopyMnemonic={() => void onCopyMnemonic()}
          onLock={() => void onLock()}
          onDismissSetupHint={() => {
            setShowSetupHint(false)
          }}
        />
      )}

      {notice && (
        <p
          role={notice.kind === 'error' ? 'alert' : 'status'}
          className={notice.kind === 'error' ? 'tui-msg tui-msg-error' : 'tui-msg tui-msg-ok'}
        >
          {notice.text}
        </p>
      )}
    </Shell>
  )
}
