import * as path from 'path';
import * as vscode from 'vscode';
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activateLanguageServer(context: vscode.ExtensionContext) {
  // The server is implemented in Rust
  const serverPath = context.asAbsolutePath(
    path.join('server', 'target', 'debug', 'mini-wdl-server')
  );
  // logging the path
  // console.log("Server Path:", serverPath);

  // If the extension is launched in debug mode then the debug server options are used
  // Otherwise the run options are used
  const serverOptions: ServerOptions = {
    run: { command: serverPath },
    debug: { command: serverPath }
  };

  // Options to control the language client
  const clientOptions: LanguageClientOptions = {
    // Register the server for WDL documents
    documentSelector: [{ scheme: 'file', language: 'mini-wdl' }],
    synchronize: {
      // Notify the server about file changes to .wdl files
      fileEvents: vscode.workspace.createFileSystemWatcher('**/*.wdl')
    }
  };

  // Create the language client and start the client
  client = new LanguageClient(
    'mini-wdl-language-server',
    'Mini WDL Language Server',
    serverOptions,
    clientOptions
  );

  // Start the client
  client.start();
}

export function deactivateLanguageServer(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
