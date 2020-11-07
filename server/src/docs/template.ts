import { CompletionItem, CompletionItemKind, MarkedString, MarkupContent, MarkupKind } from 'vscode-languageserver';


export const template : CompletionItem[]  = [

	{
		label: "INSTRUCTION",
		kind: CompletionItemKind.Keyword,
		detail: "Detailed name of instruction",
		documentation: {
			kind: MarkupKind.Markdown,
			value: 
`
### description in markdown
**like on discord**
\nSyntax:\n

\`\`\`asm8051
ADDC A, operand
\`\`\`

__Additional info__
`.trim(),
		},
	},

]