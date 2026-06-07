import type { Base64Url } from "../types/index.js";

// Pure helpers — no side effects, no domain knowledge.

export function decodeJwtPayload(jwt: string): Record<string, unknown> {
  const [, payload] = jwt.split(".");
  if (!payload) throw new Error("Invalid JWT: missing payload segment");
  const decoded = atob((payload as Base64Url).replace(/-/g, "+").replace(/_/g, "/"));
  return JSON.parse(decoded) as Record<string, unknown>;
}

export function isJwtExpired(jwt: string): boolean {
  const payload = decodeJwtPayload(jwt);
  const exp = payload["exp"];
  if (typeof exp !== "number") return false;
  return Date.now() / 1000 > exp;
}
