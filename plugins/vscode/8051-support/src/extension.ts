//#region imports
import { ExtensionContext, workspace, commands, window, MarkdownString } from 'vscode';

import { Trace } from 'vscode-jsonrpc';

import {
	Executable,
	LanguageClient,
	LanguageClientOptions,
	ServerOptions,
	StreamInfo,
	TransportKind
} from 'vscode-languageclient/node';

import * as net from 'net';
import { DocumentationPanel } from './documentation/views/documentationPanel';
import IDocumentation from './documentation/documentation';
import { localize, init as initLocalization }  from 'vscode-nls-i18n';
import { DocumentationTreeProvider } from './documentation/documentationTreeProvider';
import IOpenDocsArguments from './documentation/IOpenDocsArguments';

const DEBUG: boolean = process.env.Debug8051Plugin?.trim()?.toLowerCase() == "true";

let client: LanguageClient;


// this method is called when your extension is activated
// your extension is activated the very first time the command is executed
export function activate(context: ExtensionContext) {
	initLocalization(context.extensionPath);

	console.log(localize("asm8051.messages.activateExtension"));

	let serverOptions: ServerOptions = getServerOptions();

	let clientOptions: LanguageClientOptions = {
		documentSelector: [{ scheme: "file", language: "asm8051" }],
		synchronize: {
			fileEvents: workspace.createFileSystemWatcher('**/.clientrc')
		}
	};

	client = new LanguageClient("asm8051", "8051 support", serverOptions, clientOptions, true);
	//client.registerFeature(new GetAllDocumentationFeature());
	
	client.trace = Trace.Verbose;

	window.registerTreeDataProvider("asm8051-docs-list", new DocumentationTreeProvider(client));
	
	const showPane = commands.registerCommand("asm8051.openDocs", (args?: IOpenDocsArguments) => openDocsCommand(context, args));

	context.subscriptions.push(client.start(), showPane);
}

function getServerOptions() {
	let serverOptions: ServerOptions;

	if (DEBUG) {
		serverOptions = () => {
			let socket = net.connect({ port: 8050, });
			let result: StreamInfo = {
				writer: socket,
				reader: socket
			};
			return Promise.resolve(result);
		};
	}
	else {
		//TODO: Add options to start a "production" server
		serverOptions = {
			run: { command: "cmd" },
			debug: { command: "cmd" },
		};
	}
	return serverOptions;
}

async function openDocsCommand(context: ExtensionContext, args?: IOpenDocsArguments) {
	DocumentationPanel.render(context.extensionUri, client, args);
}
	

// this method is called when your extension is deactivated
export function deactivate() {
	console.log('Deactivating 8051 support plugin');
	if(client) {
		return client.stop();
	}

	return undefined;
}