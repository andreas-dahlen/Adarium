import * as vscode from 'vscode'

import { createSettingsResolver } from './config/resolveSettings.ts'
import { createLanguageClient } from './lsp/languageClient.ts'

export function activate(context: vscode.ExtensionContext): void {
  const output = vscode.window.createOutputChannel('Depend')
  output.appendLine('Extension loading...')

  const settings = vscode.workspace.getConfiguration('depend')
  const resolver = createSettingsResolver(context, settings, output)

  const client = createLanguageClient(
    resolver.getServerPath(),
    resolver.getProjectRoot(),
  )

  context.subscriptions.push(output, client)

  output.appendLine('Starting language server')
  void client.start()
}