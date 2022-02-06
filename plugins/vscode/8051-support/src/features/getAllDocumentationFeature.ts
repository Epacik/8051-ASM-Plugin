import { ClientCapabilities, DocumentSelector, InitializeParams, ServerCapabilities, StaticFeature } from 'vscode-languageclient';

export default class GetAllDocumentationFeature implements StaticFeature {
    fillInitializeParams?: ((params: InitializeParams) => void) | undefined;
    fillClientCapabilities(capabilities: ClientCapabilities): void {
        //throw new Error('Method not implemented.');
    }
    initialize(capabilities: ServerCapabilities<any>, documentSelector: DocumentSelector | undefined): void {
        //throw new Error('Method not implemented.');
    }
    dispose(): void {
        //throw new Error('Method not implemented.');
    }
    
}