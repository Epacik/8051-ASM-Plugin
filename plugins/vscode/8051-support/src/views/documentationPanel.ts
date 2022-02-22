import * as vscode from "vscode";
import IDocumentation from "../documentation";
import { nullishableString } from "../miscellaneousTypeAliases";
import { getUri } from "../utilities/getUri";

export class DocumentationPanel {
    public static currentPanel: DocumentationPanel | undefined;
    private readonly _panel: vscode.WebviewPanel;
    private _disposables: vscode.Disposable[] = [];
    private _docs: Map<string, IDocumentation>;


    private constructor(panel: vscode.WebviewPanel, extensionUri: vscode.Uri, docs: Map<string, IDocumentation>) {
        console.log("creating webview");
        docs = this.#objectToMap(docs);
        this._docs = docs;
        this._panel = panel;

        this._panel.onDidDispose(this.dispose, null, this._disposables);
        // this._panel.onDidChangeViewState(e => {
        //     this._panel.webview.postMessage({docs: JSON.stringify(docs, null, " ")});
        // })

        this._getWebviewContent(this._panel.webview, extensionUri, docs).then(value => {
            this._panel.webview.html = value;
        })
        
        //this._panel.webview.postMessage({docs: JSON.stringify(docs, null, " ")});
    }

    #objectToMap(docs: any): Map<string, IDocumentation> {
        let result = new Map<string, IDocumentation>();

        for (const [key, value] of Object.entries(docs)) {
            result.set(key, <IDocumentation>value);
        }
        return result;
    }

    public static render(extensionUri: vscode.Uri, docs: Map<string, IDocumentation>) {
        if (DocumentationPanel.currentPanel) {
            DocumentationPanel.currentPanel._panel.reveal(vscode.ViewColumn.Active);
        }
        else {
            const panel = vscode.window.createWebviewPanel("8051-support", "8051 Documentation", vscode.ViewColumn.Active, {
                enableScripts: true,
            });

            DocumentationPanel.currentPanel = new DocumentationPanel(panel, extensionUri, docs);
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

    private async _getWebviewContent(webview: vscode.Webview, extensionUri: vscode.Uri, docs: Map<string, IDocumentation>) {
        // Tip: Install the es6-string-html VS Code extension to enable code highlighting below

        const toolkitUri = getUri(webview, extensionUri, [ "node_modules", "@vscode", "webview-ui-toolkit", "dist", "toolkit.js",]);

        const scriptUri = getUri(webview, extensionUri, [ "out", "views", "scripts", "documentationPanelScript.js" ]);

        const cssUri = getUri(webview, extensionUri, [ "src", "views", "styles", "documentationPanel.css" ]);

        let splitted = this.#splitDocsByCategory(docs);

        return /*html*/ `
          <!DOCTYPE html>
          <html lang="en">
            <head>
              <meta charset="UTF-8">
              <meta name="viewport" content="width=device-width, initial-scale=1.0">
              <link rel="stylesheet" href="${cssUri}">
              <script type="module" src="${toolkitUri}"></script>
              <script>window.exports = {};</script>
            </head>
            <body>
              <h1 class="documentationHeader">8051 Documentation</h1>
              <section id="docs-list">
                ${await this.#createDocsList(splitted)}
              </section>
              <script src="${scriptUri}"></script>
            </body>
          </html>
        `;
    }

    async #createDocsList(splitted: Map<string, Map<string, IDocumentation>>): Promise<string> {
        let output = "";

        for (const [key, value] of splitted) {
            output += `
            <h2 class="categoryHeader">
                <span>${key}</span>
                <vscode-divider role="separator"></vscode-divider>
            </h2>
            `;
            for (const [docsKey, doc] of value) {
                output += await this.#createDocElement(docsKey, doc);
            }
            //output += `<vscode-divider role="separator"></vscode-divider>`;
        }

        return output;
    }
    async #createDocElement(key: string, doc: IDocumentation): Promise<string> {
        let result = `<h3 class="doc-mnemonic">${key}</h3>`;
        // if(!isNullishOrWhitespace(doc.detail)){
        //     result += `<h4>${await this.#parseMarkdown(doc.detail)}</h4>`;
        // }
        if(!isNullishOrWhitespace(doc.description)){
            result += `<p>${await this.#parseMarkdown(doc.description)}
            </p>`;
        }
        if(!isNullishOrWhitespace(doc.syntax)){
            result += `<h5>Syntax</h5>`;
            result += `<p>${await this.#parseMarkdown(`\`\`\`asm8051\n${doc.syntax}\n\`\`\``)}</p>`;
        }
        if(!isNullishOrWhitespace(doc.valid_operands)){
            result += `<h5>Valid operands</h5>`;
            result += `<p>${await this.#parseMarkdown(doc.valid_operands)}</p>`;
        }
        if(!isNullishOrWhitespace(doc.affected_flags)){
            result += `<h5>Affected flags</h5>`;
            result += `<p>${await this.#parseMarkdown(doc.affected_flags)}</p>`;
        }
        result += `<div class="doc-spacer"></div>\n\n`

        return result;
    }
    async #parseMarkdown(markdown: nullishableString): Promise<nullishableString> {
        return await vscode.commands.executeCommand('markdown.api.render', markdown);
    }

    #splitDocsByCategory(docs: Map<string, IDocumentation>): Map<string, Map<string, IDocumentation>> {
        let splittedDocs = new Map<string, Map<string, IDocumentation>>();
        for (const [key, value] of docs) {
            if(!splittedDocs.has(value.category)) {
                splittedDocs.set(value.category, new Map<string, IDocumentation>());
            }

            splittedDocs.get(value.category)?.set(key, value);
        }

        return splittedDocs;
    }
}

const isNullishOrWhitespace = (str: nullishableString) => str === null || str === undefined || str.trim() === "";