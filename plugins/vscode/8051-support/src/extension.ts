'use strict';
//#region imports
import { ExtensionContext, workspace, commands, window, MarkdownString, Uri } from 'vscode';

import { Trace } from 'vscode-jsonrpc';

import {
	Executable,
	LanguageClient,
	LanguageClientOptions,
	ServerOptions,
	StreamInfo,
	TransportKind
} from 'vscode-languageclient/node';

import * as ChildProcess from "child_process";

import * as net from 'net';
import { DocumentationPanel } from './documentation/views/documentationPanel';
import IDocumentation from './documentation/documentation';
import { localize, init as initLocalization }  from 'vscode-nls-i18n';
import { DocumentationTreeProvider } from './documentation/documentationTreeProvider';
import IOpenDocsArguments from './documentation/IOpenDocsArguments';
import { ClientState } from './clientState';

const DEBUG: boolean = process.env.Debug8051Plugin?.trim()?.toLowerCase() === "true";

let client: ClientState;


// this method is called when your extension is activated
// your extension is activated the very first time the command is executed
export function activate(context: ExtensionContext) {
	initLocalization(context.extensionPath);

	console.log(localize("asm8051.messages.activateExtension"));

	let serverOptions: ServerOptions = getServerOptions(context.extensionUri);

	let clientOptions: LanguageClientOptions = {
		documentSelector: [{ scheme: "file", language: "asm8051" }],
        
		synchronize: {
			fileEvents: workspace.createFileSystemWatcher('**/.clientrc')
		},
		markdown: {
			isTrusted: true,
		},
	};

    client = new ClientState(context, new LanguageClient("asm8051", "8051 support", serverOptions, clientOptions, true));

	client.start();
}

function getServerOptions(extensionUri: Uri): ServerOptions {

	if (DEBUG) {
		return () => {

			let socket = net.connect({ port: 8050, timeout: 30000 });
			let result: StreamInfo = {
				writer: socket,
				reader: socket
			};
			return Promise.resolve(result);
		};
	}
	else {
		let path = Uri.joinPath(extensionUri, ...["out", "bin", "asm8051_lsp"]).fsPath;
		if(process.platform === "linux") { //there's probably a better way, but I'm lazy
			ChildProcess.execSync(`chmod +x "${path}"`);
		}
		return {
			command: path,
			args: [ "--use-stdio" ],
            options: {
                shell: false,
            }
		};
	}
}

// this method is called when your extension is deactivated
export function deactivate() {
	console.log('Deactivating 8051 support plugin');
	if(client) {
		return client.stop();
	}

	return undefined;
}