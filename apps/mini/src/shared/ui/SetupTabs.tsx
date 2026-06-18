// Create | import tab bar for the setup screen.

export type SetupMode = 'create' | 'import'

interface SetupTabsProps {
  mode: SetupMode
  onModeChange: (mode: SetupMode) => void
}

/** Switches between generate-new and import-existing wallet flows. */
export function SetupTabs({ mode, onModeChange }: SetupTabsProps) {
  return (
    <div className="tui-tabs" role="tablist">
      <button
        type="button"
        role="tab"
        aria-selected={mode === 'create'}
        className="tui-tab"
        onClick={() => {
          onModeChange('create')
        }}
      >
        create
      </button>
      <button
        type="button"
        role="tab"
        aria-selected={mode === 'import'}
        className="tui-tab"
        onClick={() => {
          onModeChange('import')
        }}
      >
        import
      </button>
    </div>
  )
}
