import * as vscode from "vscode";
import * as cp from "child_process";
import * as path from "path";
import * as fs from "fs";
import * as os from "os";

function resolveCliCommand(projectRoot: string): string {
  const homeDir = os.homedir();
  const cargoBinPath = path.join(homeDir, ".cargo", "bin", "lint-arwaky-cli");
  if (fs.existsSync(cargoBinPath)) {
    return `"${cargoBinPath}"`;
  }
  // Check local build target
  const debugBin = path.join(projectRoot, "target", "debug", "lint-arwaky-cli");
  if (fs.existsSync(debugBin)) {
    return `"${debugBin}"`;
  }
  const releaseBin = path.join(
    projectRoot,
    "target",
    "release",
    "lint-arwaky-cli",
  );
  if (fs.existsSync(releaseBin)) {
    return `"${releaseBin}"`;
  }
  // Fallback to cargo run if we are inside the lint-arwaky project workspace
  const rootCargoToml = path.join(projectRoot, "Cargo.toml");
  if (fs.existsSync(rootCargoToml)) {
    return "cargo run --release --bin lint-arwaky-cli --";
  }
  return "lint-arwaky-cli";
}

export function activate(context: vscode.ExtensionContext) {
  let activePanel: vscode.WebviewPanel | undefined;

  // 1. Sidebar View Provider Registration
  const sidebarProvider = new LintArwakyGraphViewProvider(context);
  context.subscriptions.push(
    vscode.window.registerWebviewViewProvider(
      "lint-arwaky.graphView",
      sidebarProvider,
    ),
  );

  // 2. Refresh Command Registration
  context.subscriptions.push(
    vscode.commands.registerCommand("lint-arwaky-graph.refresh", () => {
      sidebarProvider.refresh();
    }),
  );

  // 3. Full Screen/Tab Command Registration
  let disposable = vscode.commands.registerCommand(
    "lint-arwaky-graph.show",
    () => {
      const workspaceFolders = vscode.workspace.workspaceFolders;
      if (!workspaceFolders) {
        vscode.window.showErrorMessage(
          "Please open a workspace to visualize the dependency graph.",
        );
        return;
      }

      const rootPath = workspaceFolders[0].uri.fsPath;
      const panel = vscode.window.createWebviewPanel(
        "lintArwakyGraph",
        "Lint Arwaky: Code Graph Visualizer",
        vscode.ViewColumn.One,
        {
          enableScripts: true,
          localResourceRoots: [
            vscode.Uri.file(path.join(context.extensionPath, "media")),
          ],
          retainContextWhenHidden: true,
        },
      );

      activePanel = panel;
      panel.onDidDispose(
        () => {
          if (activePanel === panel) {
            activePanel = undefined;
          }
        },
        null,
        context.subscriptions,
      );

      panel.webview.html = getHtmlContent(context, panel.webview, false);

      // Fetch graph data from Rust CLI
      vscode.window.withProgress(
        {
          location: vscode.ProgressLocation.Notification,
          title: "Generating Lint Arwaky Code Graph...",
          cancellable: false,
        },
        () => {
          return new Promise<void>((resolve, reject) => {
            const cliCmd = resolveCliCommand(rootPath);
            const cmd = `${cliCmd} vscode-graph`;

            cp.exec(
              cmd,
              { cwd: rootPath, maxBuffer: 100 * 1024 * 1024 },
              (err, stdout, stderr) => {
                if (err) {
                  vscode.window.showErrorMessage(
                    `Failed to run Lint Arwaky CLI: ${stderr || err.message}`,
                  );
                  panel.dispose();
                  reject(err);
                  return;
                }

                try {
                  const graphData = JSON.parse(stdout);
                  panel.webview.postMessage({
                    command: "loadGraph",
                    data: graphData,
                    rootPath,
                  });

                  // Select current active file if any
                  const activeEditor = vscode.window.activeTextEditor;
                  if (activeEditor) {
                    const activeFile = activeEditor.document.uri.fsPath;
                    if (activeFile.startsWith(rootPath)) {
                      const relPath = path
                        .relative(rootPath, activeFile)
                        .replace(/\\/g, "/");
                      panel.webview.postMessage({
                        command: "selectNode",
                        file: relPath,
                      });
                    }
                  }
                  resolve();
                } catch (parseErr: any) {
                  vscode.window.showErrorMessage(
                    `Failed to parse graph JSON: ${parseErr.message}`,
                  );
                  console.error("CLI output was:", stdout);
                  panel.dispose();
                  reject(parseErr);
                }
              },
            );
          });
        },
      );

      // Handle messages from the Webview (like opening a file or double-clicking node)
      panel.webview.onDidReceiveMessage(
        (message) => {
          switch (message.command) {
            case "openFile":
              const filePath = path.join(rootPath, message.file);
              const fileUri = vscode.Uri.file(filePath);
              vscode.workspace.openTextDocument(fileUri).then(
                (doc) => {
                  vscode.window.showTextDocument(doc, {
                    selection: message.line
                      ? new vscode.Range(
                          message.line - 1,
                          0,
                          message.line - 1,
                          0,
                        )
                      : undefined,
                  });
                },
                () => {
                  vscode.window.showErrorMessage(
                    `Failed to open file: ${filePath}`,
                  );
                },
              );
              return;
            case "showError":
              vscode.window.showErrorMessage(message.text);
              return;
          }
        },
        undefined,
        context.subscriptions,
      );
    },
  );

  context.subscriptions.push(disposable);

  // 4. Listen to active text editor changes to dynamically highlight node
  context.subscriptions.push(
    vscode.window.onDidChangeActiveTextEditor((editor) => {
      if (editor) {
        const document = editor.document;
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (workspaceFolders) {
          const rootPath = workspaceFolders[0].uri.fsPath;
          const filePath = document.uri.fsPath;
          if (filePath.startsWith(rootPath)) {
            const relPath = path
              .relative(rootPath, filePath)
              .replace(/\\/g, "/");
            sidebarProvider.selectNode(relPath);
            if (activePanel) {
              activePanel.webview.postMessage({
                command: "selectNode",
                file: relPath,
              });
            }
          }
        }
      }
    }),
  );
}

class LintArwakyGraphViewProvider implements vscode.WebviewViewProvider {
  private _view?: vscode.WebviewView;
  private _lastActiveFile?: string;

  constructor(private readonly _context: vscode.ExtensionContext) {}

  public resolveWebviewView(
    webviewView: vscode.WebviewView,
    _context: vscode.WebviewViewResolveContext,
    _token: vscode.CancellationToken,
  ) {
    this._view = webviewView;

    webviewView.webview.options = {
      enableScripts: true,
      localResourceRoots: [
        vscode.Uri.file(path.join(this._context.extensionPath, "media")),
      ],
    };

    webviewView.webview.html = getHtmlContent(
      this._context,
      webviewView.webview,
      true,
    );

    webviewView.webview.onDidReceiveMessage((message) => {
      switch (message.command) {
        case "openFile":
          const workspaceFolders = vscode.workspace.workspaceFolders;
          if (!workspaceFolders) return;
          const rootPath = workspaceFolders[0].uri.fsPath;
          const filePath = path.join(rootPath, message.file);
          const fileUri = vscode.Uri.file(filePath);
          vscode.workspace.openTextDocument(fileUri).then((doc) => {
            vscode.window.showTextDocument(doc, {
              selection: message.line
                ? new vscode.Range(message.line - 1, 0, message.line - 1, 0)
                : undefined,
            });
          });
          return;
      }
    });

    this.refresh();
  }

  public selectNode(file: string) {
    this._lastActiveFile = file;
    if (this._view) {
      this._view.webview.postMessage({ command: "selectNode", file });
    }
  }

  public refresh() {
    if (!this._view) return;

    const workspaceFolders = vscode.workspace.workspaceFolders;
    if (!workspaceFolders) {
      this._view.webview.postMessage({
        command: "loadGraph",
        data: { nodes: [], edges: [] },
      });
      return;
    }

    const rootPath = workspaceFolders[0].uri.fsPath;
    const cliCmd = resolveCliCommand(rootPath);
    const cmd = `${cliCmd} vscode-graph`;

    cp.exec(
      cmd,
      { cwd: rootPath, maxBuffer: 100 * 1024 * 1024 },
      (err, stdout, stderr) => {
        if (err) {
          console.error(err);
          vscode.window.showErrorMessage(
            `Failed to generate Lint Arwaky graph: ${stderr || err.message}`,
          );
          return;
        }
        try {
          const graphData = JSON.parse(stdout);
          this._view?.webview.postMessage({
            command: "loadGraph",
            data: graphData,
            rootPath,
          });

          // If there's an active text editor, or a last active file, select it
          const activeEditor = vscode.window.activeTextEditor;
          const fileToSelect = activeEditor
            ? path
                .relative(rootPath, activeEditor.document.uri.fsPath)
                .replace(/\\/g, "/")
            : this._lastActiveFile;

          if (fileToSelect && this._view) {
            this._view.webview.postMessage({
              command: "selectNode",
              file: fileToSelect,
            });
          }
        } catch (e) {
          console.error(e);
        }
      },
    );
  }
}

function getHtmlContent(
  context: vscode.ExtensionContext,
  webview: vscode.Webview,
  isSidebar: boolean,
): string {
  const htmlPath = path.join(context.extensionPath, "media", "webview.html");
  if (fs.existsSync(htmlPath)) {
    let html = fs.readFileSync(htmlPath, "utf8");

    // Resolve local cytoscape.min.js URI
    const scriptPath = vscode.Uri.file(
      path.join(context.extensionPath, "media", "cytoscape.min.js"),
    );
    const scriptUri = webview.asWebviewUri(scriptPath);

    // Replace CDN script source with local webview URI
    html = html.replace(
      "https://cdnjs.cloudflare.com/ajax/libs/cytoscape/3.29.2/cytoscape.min.js",
      scriptUri.toString(),
    );

    const bodyClass = isSidebar ? "mode-sidebar" : "mode-full";
    return html.replace("<body>", `<body class="${bodyClass}">`);
  }

  // Fallback simple HTML
  return `<!DOCTYPE html>
    <html lang="en">
    <head><title>Error</title></head>
    <body><h3>Error: webview.html not found.</h3></body>
    </html>`;
}

export function deactivate() {}
