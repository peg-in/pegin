// Sage-style BIP-39 word grid — read-only numbered display of a parsed/generated phrase.
// The editable seed entry is a single field (so password managers can fill it); this grid
// shows the words 1..N so the user can eyeball the phrase.

export function WordGrid({ words }: { words: string[] }) {
  return (
    <div className="tui-words">
      {words.map((word, i) => (
        <div className="tui-word" key={`word-${i}`}>
          <span className="tui-word-num">{i + 1}</span>
          <span className="tui-word-input">{word}</span>
        </div>
      ))}
    </div>
  )
}
