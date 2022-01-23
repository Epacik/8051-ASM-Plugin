import * as vscode from "vscode";
import { getUri } from "../utilities/getUri";

export class DocumentationPanel {
    public static currentPanel: DocumentationPanel | undefined;
    private readonly _panel: vscode.WebviewPanel;
    private _disposables: vscode.Disposable[] = [];

    private constructor(panel: vscode.WebviewPanel, extensionUri: vscode.Uri) {
        this._panel = panel;

        this._panel.onDidDispose(this.dispose, null, this._disposables);

        this._panel.webview.html = this._getWebviewContent(this._panel.webview, extensionUri);
    }

    public static render(extensionUri: vscode.Uri) {
        if (DocumentationPanel.currentPanel) {
            DocumentationPanel.currentPanel._panel.reveal(vscode.ViewColumn.One);
        }
        else {
            const panel = vscode.window.createWebviewPanel("8051-support", "Test Panel", vscode.ViewColumn.One, {
                enableScripts: true,
            });

            DocumentationPanel.currentPanel = new DocumentationPanel(panel, extensionUri);
        }
    }

    public dispose() {
        DocumentationPanel.currentPanel = undefined;

        this._panel.dispose();

        while (this._disposables.length) {
            const disposable = this._disposables.pop();
            if (disposable) {
                disposable.dispose();
            }
        }
    }

    private _getWebviewContent(webview: vscode.Webview, extensionUri: vscode.Uri) {
        // Tip: Install the es6-string-html VS Code extension to enable code highlighting below

        const toolkitUri = getUri(webview, extensionUri, [
            "node_modules",
            "@vscode",
            "webview-ui-toolkit",
            "dist",
            "toolkit.js",
        ]);

        return /*html*/ `
          <!DOCTYPE html>
          <html lang="en">
            <head>
              <meta charset="UTF-8">
              <meta name="viewport" content="width=device-width, initial-scale=1.0">
              <title>Hello World!</title>
              <script type="module" src="${toolkitUri}"></script>
            </head>
            <body>
              <h1>Dokumentacja 8051</h1>
              <vscode-button id="howdy">Howdy!</vscode-button>
            </body>
          </html>
        `;
    }
}