'use strict';

import * as vscode from "vscode";
import { localize } from "vscode-nls-i18n";
import IDocumentation from "../documentation";
import { isNullishOrWhitespace, NullishableString } from "../../miscellaneousTypeAliases";
import { getWebviewUri } from "../../utilities/getUri";
import DocumentationViewBase from "../documentationViewBase";
import { LanguageClient } from "vscode-languageclient/node";
import IOpenDocsArguments from "../IOpenDocsArguments";
import { ClientState } from "../../clientState";

import panelStyle from "./styles/documentationPanel.css";
import panelScript from "./scripts/documentationPanelScript.jsa";

export class DocumentationPanel extends DocumentationViewBase {
    public static currentPanel: DocumentationPanel | undefined;
    private readonly _panel: vscode.WebviewPanel;
    private _disposables: vscode.Disposable[] = [];
    
    private constructor(panel: vscode.WebviewPanel, extensionUri: vscode.Uri, client: ClientState, args?: IOpenDocsArguments) {
        super(client);
        console.log(localize('asm8051.views.documentationPanel.creatingView'));
        this._panel = panel;

        this._panel.onDidDispose(this.dispose, null, this._disposables);

        this.getDocumentation().then(docs => {
            this._getWebviewContent(this._panel.webview, extensionUri, docs, args).then(value => {
                this._panel.webview.html = value;
            });
        });
    }


    public static render(extensionUri: vscode.Uri, client: ClientState, args?: IOpenDocsArguments) {
        if (DocumentationPanel.currentPanel) {
            DocumentationPanel.currentPanel._panel.reveal(vscode.ViewColumn.Active);
            if (args !== undefined)
                {DocumentationPanel.currentPanel.show(args);}
        }
        else { 
            const title = localize("asm8051.views.documentationPanel.title");
            const panel = vscode.window.createWebviewPanel("8051-support", title, vscode.ViewColumn.Active, {
                enableScripts: true,
                retainContextWhenHidden: true
            });

            DocumentationPanel.currentPanel = new DocumentationPanel(panel, extensionUri, client, args);
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

    private show(args: IOpenDocsArguments){
        this._panel.webview.postMessage(args);
    }

    private async _getWebviewContent(
        webview: vscode.Webview, 
        extensionUri: vscode.Uri, 
        docs: ([key: string, label: string, entries: ([name: string, data: IDocumentation])[]])[],
        args?: IOpenDocsArguments) {
        // Tip: Install the es6-string-html VS Code extension to enable code highlighting below

        const toolkitUri = getWebviewUri(webview, extensionUri, [ "node_modules", "@vscode", "webview-ui-toolkit", "dist", "toolkit.js",]);

        const scriptUri = getWebviewUri(webview, extensionUri, [ "out", "documentation", "views", "scripts", "documentationPanelScript.js" ]);

        const title = localize("asm8051.views.documentationPanel.title");
        const html = /*html*/ `
          <!DOCTYPE html>
          <html lang="en">
            <head>
              <meta charset="UTF-8">
              <meta name="viewport" content="width=device-width, initial-scale=1.0">
              <script type="module" src="${toolkitUri}"></script>
              <script>window.exports = {};</script>
              <style>${panelStyle}</style>
            </head>
            <body>
              <h1 class="documentationHeader">${title}</h1>
              <p id="scriptThingie"></p>
              <section id="docs-list">
                ${await this.#createDocsList(docs)}
              </section>
              ${args !== undefined ? `<script defer>window.initialElement = ${JSON.stringify(args)};</script>` : ""}

              <script defer>${panelScript}</script>
            </body>
          </html>
        `;

        return html;
    }

    async #createDocsList(categorizedDocs: ([key: string, label: string, entries: ([name: string, data: IDocumentation])[]])[]): Promise<string> {
        let output = "";

        for (const [key, label, entries] of categorizedDocs) {
            output += `
            <h2 class="categoryHeader" id="${key}">
                <span>${label}</span>
                <vscode-divider role="separator"></vscode-divider>
            </h2>
            `;
            for (const [docsKey, doc] of entries) {
                output += await this.#createDocElement(docsKey, doc);
            }
        }

        return output;
    }
    async #createDocElement(key: string, doc: IDocumentation): Promise<string> {

        const insertSection = (header: NullishableString, value: NullishableString) => {
            if(isNullishOrWhitespace(value)) 
                {return;}
            
            if(!isNullishOrWhitespace(header)){
                result += `<h4>${header?.trim()}</h4>`;
            }

            result += `<p>${value?.trim()}</p>`;
        };

        const getSectionFromParsed = (section: string): NullishableString => {
            section = section.toUpperCase();
            const borderChar = '▨';
            if(isNullishOrWhitespace(parsed) || !parsed?.includes(borderChar + section)) 
                {return null;}
           
            let startIndex = parsed.indexOf(borderChar + section);
            let endIndex = parsed.lastIndexOf(section + borderChar);

            let result = parsed.slice(startIndex, endIndex);
            startIndex = result.indexOf('\n');
            endIndex = result.lastIndexOf('\n');

            return result.slice(startIndex, endIndex).replace("\n", "").trim();
        };

        let result = `<h3 class="doc-mnemonic" id="${key}">${doc.label}</h3>`;
        
        let markdown = this.#prepareMarkdownToParse(doc);
        let parsed = await this.#parseMarkdown(markdown);

        insertSection(null, getSectionFromParsed("desc"));
        insertSection(null, getSectionFromParsed("stack_space_needed"));
        insertSection(localize("asm8051.views.documentationPanel.sections.syntax"),           getSectionFromParsed("syntax"));
        insertSection(localize("asm8051.views.documentationPanel.sections.addressingModes"),  getSectionFromParsed("addressing_modes"));
        insertSection(localize("asm8051.views.documentationPanel.sections.validOperands"),    getSectionFromParsed("valid_operands"));
        insertSection(localize("asm8051.views.documentationPanel.sections.affectedFlags"),    getSectionFromParsed("affected_flags"));
        insertSection(localize("asm8051.views.documentationPanel.sections.usedRegisters"),    getSectionFromParsed("used_registers"));
        insertSection(localize("asm8051.views.documentationPanel.sections.changedRegisters"), getSectionFromParsed("changed_registers"));

        return result + `<div class="doc-spacer"></div>\n\n`;
    }
    async #parseMarkdown(markdown: NullishableString): Promise<NullishableString> {
        return await vscode.commands.executeCommand('markdown.api.render', markdown);
    }

    #prepareMarkdownToParse(doc: IDocumentation): string {
        let result = "";

        const separator = "\n\n";
        const borderChar = '▨';

        const insertSection = (section: string, value: NullishableString, valuePrefix: NullishableString = "", valueSuffix: NullishableString = "") => {
            if(isNullishOrWhitespace(value)) {return;}
            
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
        insertSection("stack_space_needed", doc.stack_space_needed);
        insertSection("valid_operands", doc.valid_operands);
        insertSection("affected_flags", doc.affected_flags);
        insertSection("addressing_modes", doc.addressing_modes);
        insertSection("used_registers", doc.used_registers);
        insertSection("changed_registers", doc.changed_registers);

        return result;
    }
}

