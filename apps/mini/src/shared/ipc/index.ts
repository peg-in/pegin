// Tauri invoke() wrappers — Tauri-specific, never imported outside apps/mini (FSD rule 5).

import { invoke } from '@tauri-apps/api/core'

export interface VaultStatus {
  sealed: boolean
  name: string | null
  hasDeviceUnlock: boolean
  passkeyCount: number
}

export interface SessionStatus {
  unlocked: boolean
}

export interface PasskeyBackupInfo {
  credentialId: string
  label: string
  relaySynced?: boolean
}

interface VaultStatusRaw {
  sealed: boolean
  name: string | null
  has_device_unlock: boolean
  passkey_count: number
}

function mapVaultStatus(raw: VaultStatusRaw): VaultStatus {
  return {
    sealed: raw.sealed,
    name: raw.name,
    hasDeviceUnlock: raw.has_device_unlock,
    passkeyCount: raw.passkey_count,
  }
}

/** Whether a seed is already sealed on this device. */
export function vaultStatus(): Promise<VaultStatus> {
  return invoke<VaultStatusRaw>('vault_status').then(mapVaultStatus)
}

/** Whether the vault is unlocked in this session. */
export function sessionStatus(): Promise<SessionStatus> {
  return invoke<SessionStatus>('session_status')
}

/** Generates a fresh BIP-39 mnemonic (12 or 24 words) for the create-wallet flow. */
export function generateSeed(wordCount: 12 | 24): Promise<string> {
  return invoke<string>('generate_seed', { wordCount })
}

/** Validates + seals the seed under the OS keychain — no user password. */
export async function sealSeed(
  mnemonic: string,
  name?: string,
  options?: { preservePasskeys?: boolean },
): Promise<void> {
  await invoke('seal_seed', {
    mnemonic,
    name: name ?? null,
    preservePasskeys: options?.preservePasskeys ?? false,
  })
}

/** Unlocks the vault via OS keychain / device auth — passwordless. */
export async function unlockVault(): Promise<void> {
  await invoke('unlock_vault')
}

/** Clears the in-memory session. */
export async function lockVault(): Promise<void> {
  await invoke('lock_vault')
}

/** Copies the unlocked mnemonic to the system clipboard. */
export async function copyMnemonic(): Promise<void> {
  await invoke('copy_mnemonic')
}

/** Returns the unlocked recovery phrase for display (session must be open). */
export function getMnemonic(): Promise<string> {
  return invoke<string>('get_mnemonic')
}

/** Lists passkey backups registered on this vault. */
export function listPasskeyBackups(): Promise<PasskeyBackupInfo[]> {
  return invoke<Array<{ credential_id: string; label: string }>>('list_passkey_backups').then(
    (items) =>
      items.map((item) => ({
        credentialId: item.credential_id,
        label: item.label,
      })),
  )
}

/** Opens the system browser to enroll a passkey backup via WebAuthn PRF. */
export function addPasskeyBackup(label: string): Promise<PasskeyBackupInfo> {
  return invoke<{ credential_id: string; label: string; relay_synced: boolean }>(
    'add_passkey_backup',
    { label },
  ).then((item) => ({
    credentialId: item.credential_id,
    label: item.label,
    relaySynced: item.relay_synced,
  }))
}

/** Pushes an existing vault passkey blob to the auth relay for demo-web login. */
export async function resyncPasskeyToRelay(credentialId: string): Promise<void> {
  await invoke('resync_passkey_to_relay', { credentialId })
}

export interface PendingSignRequest {
  requestId: string
  kind: string
  origin: string
  summary: string
  message: string | null
  returnUrl: string | null
}

/** Fetches a pending sign request from the relay (deep link wake-up). */
export async function openSignRequest(requestId: string, relayUrl: string): Promise<void> {
  await invoke('open_sign_request', { requestId, relayUrl })
}

/** Returns the sign request waiting for user approval, if any. */
export function getPendingSignRequest(): Promise<PendingSignRequest | null> {
  return invoke<PendingSignRequest | null>('get_pending_sign_request')
}

/** Signs and submits the pending request, then returns to the web app. */
export async function approveSignRequest(): Promise<void> {
  await invoke('approve_sign_request')
}

/** Rejects the pending sign request. */
export async function rejectSignRequest(): Promise<void> {
  await invoke('reject_sign_request')
}
