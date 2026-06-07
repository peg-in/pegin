// Re-export @pegin/sdk session entity + any demo-web-specific selectors.
export type { PeginSession } from "@pegin/sdk";
export { selectIsExpired, selectDisplayName } from "@pegin/sdk";
