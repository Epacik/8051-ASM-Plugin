import { CompletionItem, CompletionItemKind, MarkedString, MarkupContent, MarkupKind } from 'vscode-languageserver';

/**
 * List of documentation popups for arthmetic operations
 */
export const arithmetic : Map<string,CompletionItem>  = new Map([
	[
		"ADD", 
		{
			label: "ADD",
			kind: CompletionItemKind.Keyword,
			detail: "Dodaj do Akumulatora",
			documentation: {
				kind: MarkupKind.Markdown,
				value:
	`
	Instrukcja ADD dodaje bajt do wartości przechowywanej w akumulatorze a następnie zapisuje wynik w akumulatorze.
	Wiele flag rejestrów zostanie ustawione.
	\nSkładnia:\n
	\`\`\`asm8051
	ADD A, operand
	\`\`\`
	
	Poprawne operandy:
	#numer (np. #41H), adres_ram (np. 05H), @R0, @R1, R0, R1, R2, R3, R4, R5, R6, R7
	`.trim(),
			},
		}
	],
	
]);



