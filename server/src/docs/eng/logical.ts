import { CompletionItem, CompletionItemKind, MarkedString, MarkupContent, MarkupKind } from 'vscode-languageserver';


export const logical : Map<string,CompletionItem> = new Map([
	[
		"ANL",
		{
			label: "ANL",
			kind: CompletionItemKind.Keyword,
			detail: "Bitwise AND",
			documentation: {
				kind: MarkupKind.Markdown,
				value: 
	`
	Performs bitwise AND operation between operand1 and operand2, and stores the resoult in operand1
	\nSyntax:\n
	
	\`\`\`asm8051
	ANL operand1, operand2
	\`\`\`
	
	Valid operands on the left:
	ram_address (e.g. 05H), A, C

	Valid operands on the right:
	A, #number (e.g. #41H), ram_address (e.g. 05H), @R0, @R1, R0, R1, R2, R3, R4, R5, R6, R7, bit_address
	`.trim(),
			},
		}
	]
]);