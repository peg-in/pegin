// Pure utility functions — no side effects, no domain knowledge.

/**
 * Whether the seed-phrase enrollment path is shown. Production login is passkey-only;
 * the seed → passkey enrollment screen appears only when the build runs with
 * `PEGIN_LOGIN_MODE=demo` (dev / CI). See feat-18.
 */
export const seedEnrollEnabled: boolean = import.meta.env.PEGIN_LOGIN_MODE === 'demo'
