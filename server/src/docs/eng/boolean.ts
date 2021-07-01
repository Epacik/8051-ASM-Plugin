import { CompletionItem, CompletionItemKind, MarkedString, MarkupContent, MarkupKind } from 'vscode-languageserver';

export const boolean : Map<string,CompletionItem> = new Map([
	[
		"CLR",
		{
			label: "CLR",
			kind: CompletionItemKind.Keyword,
			detail: "Clear Register",
			documentation: {
				kind: MarkupKind.Markdown,
				value: 
		`
		CLR clears (sets to 0) all the bit(s) of the indicated register.
		If the register is a bit (including the carry bit), only the specified bit is affected.
		Clearing the Accumulator sets the Accumulator's value to 0.
		\nSyntax:\n
		
		\`\`\`asm8051
		CLR operand
		\`\`\`
		
		Valid operands:
		A, C, ram_address (e.g. 05H)
		
		`.trim(),
			},
		}
	],
	[
		"SETB",
		{
			label: "SETB",
			kind: CompletionItemKind.Keyword,
			detail: "Set Bit",
			documentation: {
				kind: MarkupKind.Markdown,
				value: 
		`
		Sets the specified bit.
		\nSyntax:\n
		
		\`\`\`asm8051
		SETB operand
		\`\`\`
		
		Valid operands:
		C, ram_address (e.g. 05H)
		
		`.trim(),
			},
		}
	],
	[
		"CPL",
		{
			label: "CPL",
			kind: CompletionItemKind.Keyword,
			detail: "Complement Register",
			documentation: {
				kind: MarkupKind.Markdown,
				value: 
		`
		CPL complements operand, leaving the result in operand. If operand is a single bit then the state of the bit will be reversed. If operand is the Accumulator then all the bits in the Accumulator will be reversed. This can be thought of as "Accumulator Logical Exclusive OR 255" or as "255-Accumulator." If the operand refers to a bit of an output Port, the value that will be complemented is based on the last value written to that bit, not the last value read from it.
		\nSyntax:\n
		
		\`\`\`asm8051
		CPL operand
		\`\`\`
		
		Valid operands:
		A, C, ram_address (e.g. 05H)
		
		`.trim(),
			},
		}
	],
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
		ANL does a bitwise "AND" operation between operand1 and operand2, leaving the resulting value in operand1. The value of operand2 is not affected. A logical "AND" compares the bits of each operand and sets the corresponding bit in the resulting byte only if the bit was set in both of the original operands, otherwise the resulting bit is cleared.
		\nSyntax:\n
		
		\`\`\`asm8051
		ANL operand1, operand2
		\`\`\`
		
		Valid operands:
		
		operand1: ram_address (e.g. 05H), A, C
		
		operand2: #number (e.g. #41H), ram_address (e.g. 05H), /bit (e. g. /22h), @R0, @R1, R0, R1, R2, R3, R4, R5, R6, R7, A
		
		`.trim(),
			},
		}
	],
	[
		"ORL",
		{
			label: "ORL",
			kind: CompletionItemKind.Keyword,
			detail: "Bitwise OR",
			documentation: {
				kind: MarkupKind.Markdown,
				value: 
		`
		ORL does a bitwise "OR" operation between operand1 and operand2, leaving the resulting value in operand1. The value of operand2 is not affected. A logical "OR" compares the bits of each operand and sets the corresponding bit in the resulting byte if the bit was set in either of the original operands, otherwise the resulting bit is cleared.
		\nSyntax:\n
		
		\`\`\`asm8051
		ORL operand1, operand2
		\`\`\`
		
		Valid operands:
		
		operand1: ram_address (e.g. 05H), A, C
		
		operand2: #number (e.g. #41H), ram_address (e.g. 05H), /bit (e. g. /22h), @R0, @R1, R0, R1, R2, R3, R4, R5, R6, R7, A
		
		`.trim(),
			},
		}
	],
	[
		"XRL",
		{
			label: "XRL",
			kind: CompletionItemKind.Keyword,
			detail: "Bitwise XOR",
			documentation: {
				kind: MarkupKind.Markdown,
				value: 
		`
		XRL does a bitwise "EXCLUSIVE OR" operation between operand1 and operand2, leaving the resulting value in operand1. The value of operand2 is not affected. A logical "EXCLUSIVE OR" compares the bits of each operand and sets the corresponding bit in the resulting byte if the bit was set in either (but not both) of the original operands, otherwise the bit is cleared.
		\nSyntax:\n
		
		\`\`\`asm8051
		XRL operand1, operand2
		\`\`\`
		
		Valid operands:
		
		operand1: ram_address (e.g. 05H), A, C
		
		operand2: #number (e.g. #41H), ram_address (e.g. 05H), /bit (e. g. /22h), @R0, @R1, R0, R1, R2, R3, R4, R5, R6, R7, A
		
		`.trim(),
			},
		}
	]
]);

