//#region imports
import { ExtensionContext, workspace } from 'vscode';

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
//#endregion

const DEBUG: boolean = true;

let client: LanguageClient;

// this method is called when your extension is activated
// your extension is activated the very first time the command is executed
export function activate(context: ExtensionContext) {
	
	console.log('Activating 8051 support plugin');

	let serverOptions: ServerOptions;

	if (DEBUG) {
		let connectionInfo = {
			port: 8050
		};
		serverOptions = () => {
			let socket = net.connect(connectionInfo);
			let result: StreamInfo = {
				writer: socket,
				reader: socket
			};
			return Promise.resolve(result);
		};
	}
	else{
		//TODO: Add options to start a "production" server
		serverOptions = {
			run: { command: "cmd" },
			debug: { command: "cmd" },
		};
	}

	let clientOptions: LanguageClientOptions = {
		documentSelector: [
			{ scheme: "file", language: "asm8051" }
		],
		synchronize: {
			fileEvents: workspace.createFileSystemWatcher('**/.clientrc')
		}
	};

	client = new LanguageClient("8051-support", "8051 support", serverOptions, clientOptions, true);
	client.trace = Trace.Verbose;

	context.subscriptions.push(client.start());
}

// this method is called when your extension is deactivated
export function deactivate() {
	console.log('Deactivating 8051 support plugin');
	if(client) {
		return client.stop();
	}

	return undefined;
}


