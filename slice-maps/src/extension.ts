import * as vscode from 'vscode'
import { createSliceProvider } from './core/sliceProvider.ts'
import { register } from './vscode/register.ts'
import { initializeOutput, log } from './utils/outputChannel.ts'

export async function activate(context: vscode.ExtensionContext): Promise<void> {
  initializeOutput(context)

  const root = vscode.workspace.workspaceFolders?.[0]

  if (!root) {
    log('ERROR', `couldn't resolve workspace Root`)
    void vscode.window.showErrorMessage(
      'Slice Maps requires an open workspace.'
    )
    return
  }
  // await context.workspaceState.update('sliceMaps', undefined)

  try {

    const provider = createSliceProvider(context, root)
    await provider.ctrl.general.startup()
    register(context, provider)

    log('Slice Maps', 'Initialization success')

  } catch (error) {

    log('ERROR', 'Extension activation failed')
    log('ERROR', error instanceof Error
      ? error.stack ?? error.message
      : String(error))

    void vscode.window.showErrorMessage('Slice Maps failed to start. See the Slice Maps output channel for details.')
  }
}

export function deactivate(): void { }