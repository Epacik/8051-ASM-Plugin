import { ExtensionContext, workspace, commands, window, MarkdownString, Uri, Disposable } from 'vscode';
import { LanguageClient, Trace } from "vscode-languageclient/node";
import { DocumentationTreeProvider } from './documentation/documentationTreeProvider';
import IOpenDocsArguments from "./documentation/IOpenDocsArguments";
import { DocumentationPanel } from "./documentation/views/documentationPanel";

export class ClientState {
    #context: ExtensionContext;
    #client: LanguageClient;
    #isReady: boolean;
    #showPane: Disposable;
    constructor(context: ExtensionContext, client: LanguageClient) {
        this.#client = client;
        this.#context = context;
        this.#isReady = false;
        
        client.trace = Trace.Verbose;
        client.onReady().then(x => this.#isReady = true);

        window.registerTreeDataProvider("asm8051-docs-list", new DocumentationTreeProvider(this));
        this.#showPane = commands.registerCommand("asm8051.openDocs", (args?: IOpenDocsArguments) => this.openDocsCommand(context, args));
    }

    public async getClient() : Promise<LanguageClient> {
        if (!this.#isReady){
            await this.#client.onReady();
        }

        return this.#client;
    }

    async openDocsCommand(context: ExtensionContext, args?: IOpenDocsArguments) {
        DocumentationPanel.render(context.extensionUri, this, args);
    }
    
    start() {
        this.#context.subscriptions.push(this.#client.start(), this.#showPane);
    }
    stop() {
        this.#client.stop();
    }


}
