import path from 'node:path'
import * as vscode from 'vscode'
import { fileURLToPath } from 'node:url'

export function createSettingsResolver(
  settings: vscode.WorkspaceConfiguration,
  output: vscode.OutputChannel,
) {
  const projectRoot = getProjectRoot(settings, output)
  const cliPath = getCliSpawnPath()

  return {
    getCliSpawnPath(): string {
      return cliPath
    },

    getProjectRootArg(): string {
      return projectRoot
    },
  }
}

function getCliSpawnPath(): string {
  return fileURLToPath(import.meta.resolve('cascade/cli'))
}

function getProjectRoot(
  settings: vscode.WorkspaceConfiguration,
  output: vscode.OutputChannel,
): string {
  const workspaceFolder = vscode.workspace.workspaceFolders?.[0]

  if (!workspaceFolder) {
    output.appendLine('ERROR: workspace folder is missing')
    throw new Error('No workspace folder')
  }

  const configuredRoot = settings.get<string>('projectRoot')

  return configuredRoot
    ? path.resolve(workspaceFolder.uri.fsPath, configuredRoot)
    : workspaceFolder.uri.fsPath
}