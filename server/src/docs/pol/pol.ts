import { CompletionItem } from 'vscode-languageserver';
import { arithmetic } from "./arithmetic"
import { boolean } from "./boolean"
import { dataTransfer } from "./dataTransfer"
import { logical } from "./logical"
import { pins } from "./pins"
import { programControl } from "./programControl"
import { registers } from "./registers"

export const pol : Map<string,CompletionItem> = new Map([
	...arithmetic,
	...boolean,
	...dataTransfer,
	...logical,
	...pins,
	...programControl,
	...registers,
]);