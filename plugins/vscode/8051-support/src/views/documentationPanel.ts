import * as vscode from "vscode";
import { localize } from "vscode-nls-i18n";
import IDocumentation from "../documentation";
import { nullishableString } from "../miscellaneousTypeAliases";
import { getUri } from "../utilities/getUri";


export class DocumentationPanel {
    public static currentPanel: DocumentationPanel | undefined;
    private readonly _panel: vscode.WebviewPanel;
    private _disposables: vscode.Disposable[] = [];
    private _docs: Map<string, IDocumentation>;
    
    private constructor(panel: vscode.WebviewPanel, extensionUri: vscode.Uri, docs: Map<string, IDocumentation>) {
        console.log(localize('asm8051.views.documentationPanel.creatingView'));
        docs = this.#objectToMap(docs);
        this._docs = docs;
        this._panel = panel;

        this._panel.onDidDispose(this.dispose, null, this._disposables);

        this._getWebviewContent(this._panel.webview, extensionUri, docs).then(value => {
            this._panel.webview.html = value;
        })
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
            const title = localize("asm8051.views.documentationPanel.title");
            const panel = vscode.window.createWebviewPanel("8051-support", title, vscode.ViewColumn.Active, {
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

        const title = localize("asm8051.views.documentationPanel.title");
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
              <h1 class="documentationHeader">${title}</h1>
              <section id="docs-list">
                ${await this.#createDocsList(splitted)}
              </section>
              <!--<script src="${scriptUri}"></script>-->
            </body>
          </html>
        `;
    }

    async #createDocsList(splitted: Map<string, Map<string, IDocumentation>>): Promise<string> {
        let output = "";
        const keys: Array<[key: string, label: string]> = [];

        for (const key of splitted.keys()){
            keys.push([key, localize('asm8051.views.documentationPanel.categories.' + key)]);
        }
        keys.sort((a, b): number => {
            if(a[1] < b[1]) return -1;
            else if (a[1] > b[1]) return 1;
            return 0;
        });
        for (const kl of keys){
            const value = splitted.get(kl[0]);
            if(value === undefined) continue;

            output += `
            <h2 class="categoryHeader">
                <span>${kl[1]}</span>
                <vscode-divider role="separator"></vscode-divider>
            </h2>
            `;
            for (const [docsKey, doc] of value) {
                output += await this.#createDocElement(docsKey, doc);
            }
        }


        return output;
    }
    async #createDocElement(key: string, doc: IDocumentation): Promise<string> {

        const insertSection = (header: nullishableString, value: nullishableString) => {
            if(isNullishOrWhitespace(value)) return;
            
            if(!isNullishOrWhitespace(header)){
                result += `<h5>${header?.trim()}</h5>`
            }

            result += `<p>${value?.trim()}</p>`;
        };

        const getSectionFromParsed = (section: string): nullishableString  => {
            section = section.toUpperCase();
            const borderChar = '▨';
            if(isNullishOrWhitespace(parsed) || !parsed?.includes(borderChar + section)) return null;
           
            let startIndex = parsed.indexOf(borderChar + section);
            let endIndex = parsed.lastIndexOf(section + borderChar);

            let result = parsed.slice(startIndex, endIndex);
            startIndex = result.indexOf('\n');
            endIndex = result.lastIndexOf('\n');

            return result.slice(startIndex, endIndex).replace("\n", "").trim();
        }

        let result = `<h3 class="doc-mnemonic">${key}</h3>`;
        
        let markdown = this.#prepareMarkdownToParse(doc);
        let parsed = await this.#parseMarkdown(markdown);

        insertSection(null, getSectionFromParsed("desc"));
        insertSection("Syntax", getSectionFromParsed("syntax"));
        insertSection("Valid operands", getSectionFromParsed("valid_operands"));
        insertSection("Affected flags", getSectionFromParsed("affected_flags"));

        return result + `<div class="doc-spacer"></div>\n\n`;
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

    #prepareMarkdownToParse(doc: IDocumentation): string {
        let result = "";

        const separator = "\n\n";
        const borderChar = '▨';

        const insertSection = (section: string, value: nullishableString, valuePrefix: nullishableString = "", valueSuffix: nullishableString = "") => {
            if(isNullishOrWhitespace(value)) return;
            
            section = section.toUpperCase();
            result += 
                borderChar + section +
                separator +
                valuePrefix + value?.trim() + valueSuffix +
                separator + 
                section + borderChar;
        };

        insertSection("desc", doc.description);
        insertSection("syntax", doc.syntax, "```asm8051\n", "\n```");
        insertSection("valid_operands", doc.valid_operands);
        insertSection("affected_flags", doc.affected_flags);

        return result;
    }
}

const isNullishOrWhitespace = (str: nullishableString) => str === null || str === undefined || str.trim() === "";