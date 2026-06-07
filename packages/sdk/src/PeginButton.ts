export interface PeginButtonOptions {
  rpId: string;
  onSuccess: (jwt: string) => void;
  onError: (err: Error) => void;
}

// Stub — full implementation in feat-9
export class PeginButton {
  constructor(private readonly options: PeginButtonOptions) {}

  mount(_container: HTMLElement): void {
    throw new Error("PeginButton.mount — not yet implemented (feat-9)");
  }

  unmount(): void {}
}
