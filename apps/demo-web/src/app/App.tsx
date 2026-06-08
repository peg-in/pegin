import { LoginCard } from '../widgets/LoginCard/index.js'

export function App() {
  return (
    <main
      style={{ fontFamily: 'sans-serif', maxWidth: 480, margin: '4rem auto', padding: '0 1rem' }}
    >
      <h1>PEGIN Demo</h1>
      <p>Demo relying party — "Login with PEGIN" button.</p>
      <LoginCard />
    </main>
  )
}
