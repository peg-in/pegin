// authenticate feature — orchestrates login flow using @pegin/sdk.
// Full implementation in feat-9.

export type AuthStatus = "idle" | "loading" | "authenticated" | "error";

export interface AuthState {
  status: AuthStatus;
  errorMessage: string | null;
}

export const initialAuthState: AuthState = {
  status: "idle",
  errorMessage: null,
};
