import * as vscode from 'vscode'

const output = vscode.window.createOutputChannel('Slice Maps')

export function initializeOutput(
  context: vscode.ExtensionContext
): void {
  context.subscriptions.push(output)
}

export function log(
  label: string,
  data: unknown
): void {
  output.appendLine(`[${label}] ${JSON.stringify(data, null, 2)}`)
}