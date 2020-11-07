import { CompletionItem, CompletionItemKind, MarkedString, MarkupContent, MarkupKind } from 'vscode-languageserver';


export const boolean : CompletionItem[]  = [
{
	label: "CLR",
	kind: CompletionItemKind.Keyword,
	detail: "Clear Carry flag",
	documentation: {
		kind: MarkupKind.Markdown,
		value: 
`
CLR clears (sets to 0) all the bit(s) of the indicated register.
If the register is a bit (including the carry bit), only the specified bit is affected.
Clearing the Accumulator sets the Accumulator's value to 0.
\nSyntax:\n

\`\`\`asm8051
ADDC A, operand
\`\`\`

__Additional info__
`.trim(),
	},
},
]