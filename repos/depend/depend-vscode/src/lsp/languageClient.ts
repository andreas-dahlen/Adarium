import {
  LanguageClient,
  TransportKind,
  Trace,
} from 'vscode-languageclient/node'

export function createLanguageClient(
  serverPath: string,
  projectRoot: string,
): LanguageClient {
  const serverOptions = {
    command: serverPath,
    args: ['lsp', '--root', projectRoot],
    transport: TransportKind.stdio,
  }

  const clientOptions = {
    documentSelector: [
      { scheme: 'file', language: 'javascript' },
      { scheme: 'file', language: 'typescript' },
    ],
  }

  const client = new LanguageClient(
    'depend',
    'Depend Language Server',
    serverOptions,
    clientOptions,
  )

  client.setTrace(Trace.Verbose)

  return client
}