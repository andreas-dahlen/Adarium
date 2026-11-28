import fs from 'node:fs'
import * as vscode from 'vscode'

import { getPlatform } from './getPlatform.js'

export function createSettingsResolver(
  context: vscode.ExtensionContext,
  settings: vscode.WorkspaceConfiguration,
  output: vscode.OutputChannel,
) {
  const projectRoot = getProjectRoot(settings, output)
  const cliPath = getServerPath(context)

  return {
    getServerPath(): string {
      return cliPath
    },

    getProjectRoot(): string {
      return projectRoot
    },
  }
}

function getServerPath(context: vscode.ExtensionContext): string {
  const executable = process.platform === 'win32'
    ? 'depend.exe'
    : 'depend'

  const serverPath = vscode.Uri.joinPath(
    context.extensionUri,
    'bin',
    getPlatform(),
    executable,
  ).fsPath

  if (!fs.existsSync(serverPath)) {
    throw new Error(`Depend executable not found: ${serverPath}`)
  }

  return serverPath
}

function getProjectRoot(
  settings: vscode.WorkspaceConfiguration,
  output: vscode.OutputChannel,
): string {
  const workspaceFolder = vscode.workspace.workspaceFolders?.[0]

  if (!workspaceFolder) {
    output.appendLine('ERROR: workspace folder is missing')
    throw new Error(' ')
  }

  const projectRoot = settings.get<string>('projectRoot')

  if (!projectRoot) {
    output.appendLine('ERROR: projectRoot setting is missing')
    throw new Error(' ')
  }

  return vscode.Uri.joinPath(
    workspaceFolder.uri,
    ...projectRoot.split('/'),
  ).fsPath
}