// Typed HTTP client for the PEGIN backend API.
// Only typed fetch wrappers live here — no business logic.
// Full implementation in feat-8 / feat-9.

export interface PeginApiOptions {
  baseUrl: string;
}

export class PeginApiClient {
  constructor(private readonly options: PeginApiOptions) {}

  async post<T>(path: string, body: unknown): Promise<T> {
    const res = await fetch(`${this.options.baseUrl}${path}`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body),
    });
    if (!res.ok) throw new Error(`PEGIN API ${path} → ${res.status}`);
    return res.json() as Promise<T>;
  }
}
