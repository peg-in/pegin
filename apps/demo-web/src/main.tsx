import React from "react";
import ReactDOM from "react-dom/client";

function App() {
  return (
    <main>
      <h1>PEGIN Demo</h1>
      <p>Login with PEGIN demo relying party — coming in feat-9.</p>
    </main>
  );
}

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
