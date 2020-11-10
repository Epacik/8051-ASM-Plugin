import {
	createConnection,
	TextDocuments,
	Diagnostic,
	DiagnosticSeverity,
	ProposedFeatures,
	InitializeParams,
	DidChangeConfigurationNotification,
	CompletionItem,
	CompletionItemKind,
	TextDocumentPositionParams,
	TextDocumentSyncKind,
	InitializeResult,
	Hover,
	MarkedString,
	MarkupContent
} from 'vscode-languageserver';

import {
	TextDocument
} from 'vscode-languageserver-textdocument';
import {docs} from './docs/docs'
import {List} from 'linq-typescript'
import { diagnostics } from './diagnostics/diagnostics';
import { debug } from './debug';



// Create a connection for the server, using Node's IPC as a transport.
// Also include all preview / proposed LSP features.
let connection = createConnection(ProposedFeatures.all );

// Create a simple text document manager. 
let documents: TextDocuments<TextDocument> = new TextDocuments(TextDocument);

let hasConfigurationCapability: boolean = false;
let hasWorkspaceFolderCapability: boolean = false;
let hasDiagnosticRelatedInformationCapability: boolean = false;

connection.onInitialize((params: InitializeParams) => {

	debug.setOptions(new debug.DebugOptions({logLevel: debug.LogLevel.All}))


	let capabilities = params.capabilities;

	// Does the client support the `workspace/configuration` request?
	// If not, we fall back using global settings.
	hasConfigurationCapability = !!(
		capabilities.workspace && !!capabilities.workspace.configuration
	);
	hasWorkspaceFolderCapability = !!(
		capabilities.workspace && !!capabilities.workspace.workspaceFolders
	);
	hasDiagnosticRelatedInformationCapability = !!(
		capabilities.textDocument &&
		capabilities.textDocument.publishDiagnostics &&
		capabilities.textDocument.publishDiagnostics.relatedInformation
	);

	const result: InitializeResult = {
		capabilities: {
			textDocumentSync: TextDocumentSyncKind.Incremental,
			// Tell the client that this server supports code completion.
			completionProvider: {
				resolveProvider: true
			},
			hoverProvider: true,
		}
	};
	if (hasWorkspaceFolderCapability) {
		result.capabilities.workspace = {
			workspaceFolders: {
				supported: true
			}
		};
	}
	return result;
});

connection.onInitialized(() => {
	if (hasConfigurationCapability) {
		// Register for all configuration changes.
		connection.client.register(DidChangeConfigurationNotification.type, undefined);
	}
	if (hasWorkspaceFolderCapability) {
		connection.workspace.onDidChangeWorkspaceFolders(_event => {
			connection.console.log('Workspace folder change event received.');
		});
	}
});

// The example settings
export interface DocSettings {
	maxNumberOfProblems: number;
}

// The global settings, used when the `workspace/configuration` request is not supported by the client.
// Please note that this is not the case when using this server with the client provided in this example
// but could happen with other clients.
const defaultSettings: DocSettings = { maxNumberOfProblems: 1000 };
let globalSettings: DocSettings = defaultSettings;

// Cache the settings of all open documents
let documentSettings: Map<string, Thenable<DocSettings>> = new Map();

connection.onDidChangeConfiguration(change => {
	if (hasConfigurationCapability) {
		// Reset all cached document settings
		documentSettings.clear();
	} else {
		globalSettings = <DocSettings>(
			(change.settings.languageServerExample || defaultSettings)
		);
	}

	// Revalidate all open text documents
	documents.all().forEach(validateTextDocument);
});

function getDocumentSettings(resource: string): Thenable<DocSettings> {
	if (!hasConfigurationCapability) {
		return Promise.resolve(globalSettings);
	}
	let result = documentSettings.get(resource);
	if (!result) {
		result = connection.workspace.getConfiguration({
			scopeUri: resource,
			section: 'languageServerExample'
		});
		documentSettings.set(resource, result);
	}
	return result;
}

// Only keep settings for open documents
documents.onDidClose(e => {
	documentSettings.delete(e.document.uri);
});

// The content of a text document has changed. This event is emitted
// when the text document first opened or when its content has changed.
documents.onDidChangeContent(change => {
	validateTextDocument(change.document);
});

async function validateTextDocument(textDocument: TextDocument): Promise<void> {
	// In this simple example we get the settings for every validate run.
	let settings = await getDocumentSettings(textDocument.uri);

	
	let diags: Diagnostic[] = diagnostics.getDisgnostics(textDocument, settings);

	// Send the computed diagnostics to VSCode.
	connection.sendDiagnostics({ uri: textDocument.uri, diagnostics: diags });
}

connection.onDidChangeWatchedFiles(_change => {
	// Monitored files have change in VSCode
	connection.console.log('We received an file change event');
});



// This handler provides the initial list of the completion items.
connection.onCompletion(
	(_textDocumentPosition: TextDocumentPositionParams): CompletionItem[] => {
		var items = docs.getItems();
		return items;
	}
);

// This handler resolves additional information for the item selected in
// the completion list.
connection.onCompletionResolve(
	(item: CompletionItem): CompletionItem => item
);

connection.onHover((params: TextDocumentPositionParams): Hover|undefined => {
	
	const document = documents.get(params.textDocument.uri);
	if(document == undefined) return undefined;

    const start = {
      line: params.position.line,
      character: 0,
    };
    const end = {
      line: params.position.line + 1,
      character: 0,
    };
    const text = document.getText({ start, end });
    const index = document.offsetAt(params.position) - document.offsetAt(start);
	const word = getWord(text, index);
	console.log(`word: ${word}`)
	 


    if (word !== '') {
		const item = new List<CompletionItem>(docs.getItems()).firstOrDefault(x => x.label.trim() === word.trim()); 
		//console.log(item);

		if(item === undefined) return undefined;
		var det = item.detail !== undefined ? `\n#### ${item.detail}` : '';
		var dc = item.documentation !== undefined ? `\n${(<MarkupContent>item.documentation).value}` : '';
		//console.log(dc)

		let doc : MarkedString[] = [`### ${item.label}${det}${dc}`]
		return {
			contents: doc,
		};
    }

    return undefined;

  });


function getWord(text: string, index: number) {
    var startIndex = (function _this (pos) :number {
        if (!text.substring(pos, pos + 1).match(/[\p{L}\p{N}_.]/u)) {
            return pos + 1;
        } else if (pos === 0) {
            return 0;
        } else {
            return _this(pos - 1);
        }
    })(index - 1);
    var endIndex = (function _this (pos):number {
        if (!text.substring(pos, pos + 1).match(/[\p{L}\p{N}_.]/u) || pos === text.length) {
            return pos;
        } else {
            return _this(pos + 1);
        }
    })(index + 1);

    return text.substring(startIndex, endIndex);
}

// Make the text document manager listen on the connection
// for open, change and close text document events
documents.listen(connection);

// Listen on the connection
connection.listen();
