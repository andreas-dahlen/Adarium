import * as vscode from 'vscode'
import { createSettingsResolver } from './config/resolveSettings.ts'
import { createTerminal } from './terminal/createTerminal.ts'
import { updateStatusBar } from './vscode/statusBar.ts'
import { createCommandSubscriptions } from './vscode/subscriptions.ts'


export function activate(context: vscode.ExtensionContext): void {
  const output = vscode.window.createOutputChannel('Cascade Compiler')
  output.appendLine('Extension loading...')
  const statusBar = vscode.window.createStatusBarItem(
    vscode.StatusBarAlignment.Left,
  )
  let terminal: vscode.Terminal | undefined


  context.subscriptions.push(
    output,
    statusBar,
    ...createCommandSubscriptions({
      startCompiler,
      stopCompiler,
      restartCompiler
    }),
    vscode.window.onDidCloseTerminal(closedTerminal => {
      if (closedTerminal !== terminal) {
        return
      }
      output.appendLine('Stopping compiler service')
      terminal = undefined
      updateStatusBar(statusBar, terminal)
    }),

    vscode.workspace.onDidChangeConfiguration(event => {
      if (!event.affectsConfiguration('cascade')) {
        return
      }
      output.appendLine('Configuration changed')
      restartCompiler()
    }),

    {
      dispose() {
        stopCompiler()
      },
    },
  )

  statusBar.show()
  startCompiler()

  function startCompiler() {
    output.appendLine('Starting compiler service')
    output.appendLine('really...')
    if (terminal) {
      terminal.show()
      return
    }
    const settings = vscode.workspace.getConfiguration(
      'cascade')

    let resolver
    try {
      resolver = createSettingsResolver(settings, output)
    } catch (error) {
      output.appendLine(`RESOLVER ERROR: ${String(error)}`)
      output.appendLine(error instanceof Error ? error.stack ?? '' : '')
      return
    }
    const cliFile = resolver.getCliSpawnPath()
    const projectRoot = resolver.getProjectRootArg()

    output.appendLine(`CLI: ${cliFile}`)
    output.appendLine(`ROOT: ${projectRoot}`)

    terminal = createTerminal(cliFile, projectRoot)

    updateStatusBar(statusBar, terminal)
    terminal.show()
  }

  function stopCompiler() {
    output.appendLine('Stopping compiler service')
    terminal?.dispose()
    terminal = undefined
    updateStatusBar(statusBar, terminal)
  }

  function restartCompiler() {
    stopCompiler()
    startCompiler()
  }
}

export function deactivate(): void { }