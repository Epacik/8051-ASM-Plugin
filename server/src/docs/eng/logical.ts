import { CompletionItem, CompletionItemKind, MarkedString, MarkupContent, MarkupKind } from 'vscode-languageserver';


export const logical : Map<string, CompletionItem> = new Map([
	[
		"ANL",
		{
			label: "ANL",
			kind: CompletionItemKind.Keyword,
			detail: "Bitwise AND",
			documentation: {
				kind: MarkupKind.Markdown,
				value: 
`Performs a bitwise AND operation between two operands, and stores the result in first operand

---

Syntax:
> \`\`\`asm8051
> ANL A, R1
> ANL C, #3H
> \`\`\`

---

Valid first operands:
- ram_address (e.g. 05H)
- A 
- C

Valid second operands:
- A 
- #number (e.g. #41H)
- ram_address (e.g. 05H)
- @R0
- @R1
- R0
- R1
- R2
- R3
- R4
- R5
- R6
- R7
- bit_address
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
			documentation:
			{
				kind: MarkupKind.Markdown,
				value:
`Performs a bitwise OR operation between two operands, and stores result in the first operand

---

Syntax:
> \`\`\`asm8051
> ORL A, R1
> ORL C, #3H
> \`\`\`

---

Valid first operands:
- ram_address (e.g. 05H)
- A 
- C

Valid second operands:
- A 
- #number (e.g. #41H)
- ram_address (e.g. 05H)
- @R0
- @R1
- R0
- R1
- R2
- R3
- R4
- R5
- R6
- R7
- bit_address

`.trim()
			}

		}
	],
	[
		"XRL",
		{
			label: "XRL",
			kind: CompletionItemKind.Keyword,
			detail: "Bitwise XOR",
			documentation:
			{
				kind: MarkupKind.Markdown,
				value:
`Performs a bitwise XOR operation between two operands, and stores result in the first operand

---

Syntax:
> \`\`\`asm8051
> XRL A, R1
> XRL C, #3H
> \`\`\`

---

Valid first operands:
- ram_address (e.g. 05H)
- A 
- C

Valid second operands:
- A 
- #number (e.g. #41H)
- ram_address (e.g. 05H)
- @R0
- @R1
- R0
- R1
- R2
- R3
- R4
- R5
- R6
- R7
- bit_address

`.trim()
			}

		}
	],
	[
		"CLR",
		{
			label: "CLR",
			kind: CompletionItemKind.Keyword,
			detail: "Clear register",
			documentation:
			{
				kind: MarkupKind.Markdown,
				value:
`Sets all bits in a refister or a memory to 0
If an operand is a bit, then only that bit will be affected

---

Syntax:
> \`\`\`asm8051
> CLR A
> CLR C
> CLR 05H
> \`\`\`

---

Valid operands:
- ram_address (e.g. 05H)
- A 
- C

`
			}
		}
	],
	[
		"CPL",
		{
			label: "CPL",
			kind: CompletionItemKind.Keyword,
			detail: "Complement register",
			documentation:
			{
				kind: MarkupKind.Markdown,
				value:
`Complements (reverses) value in operand
If operand is an accumulator, the value of accumulator will be reversed (all zeros will be changed to ones and vice versa)
if operand is a bit, it will be reversed
if operand points to output port, the value will be reversed based on the last value ***written*** to it

---

Syntax:
> \`\`\`asm8051
> CPL A
> CPL C
> CPL 05H
> \`\`\`

---

Valid operands:
- ram_address (e.g. 05H)
- A 
- C

---

Value before
10101100

Value after
01010011
`
			}
		}
	],
	[
		"RL",
		{
			label: "RL",
			kind: CompletionItemKind.Keyword,
			detail: "Rotate accumulator to the left",
			documentation:
			{
				kind: MarkupKind.Markdown,
				value:
`All of the bits contained in a operands will be shifted to the left
The bit stored in the most significant bit (bit 7) will be moved to least significant bit (bit 0)

---

Syntax:
> \`\`\`asm8051
> RL A
> \`\`\`

---

Value before
10101100

Value after
01011001`
			}
		}
	],
	[
		"RLC",
		{
			label: "RLC",
			kind: CompletionItemKind.Keyword,
			detail: "Rotate accumulator to the left trough Carry flag",
			documentation:
			{
				kind: MarkupKind.Markdown,
				value:
`All of the bits contained in a operands will be shifted to the left
The bit stored in the Carry flag will be moved to the least significant bit (bit 0)
The most significant bit (bit 7) will be moved to the Carry flag

---

Syntax:
> \`\`\`asm8051
> RL A
> \`\`\`

---

Affected flags:
- Carry flag (*C*)

Value before
10101100
C = 0

Value after
01011000
C = 1`

			}
		}
	],
	[
		"RR",
		{
			label: "RR",
			kind: CompletionItemKind.Keyword,
			detail: "Rotate accumulator to the right",
			documentation:
			{
				kind: MarkupKind.Markdown,
				value:
`All of the bits contained in a operands will be shifted to the right
The bit stored in the least significant bit (bit 0) will be moved to most significant bit (bit 7)

---

Syntax:
> \`\`\`asm8051
> RR A
> \`\`\`

---

Value before
10101100

Value after
01010110`
			}
		}
	],
	[
		"RLC",
		{
			label: "RLC",
			kind: CompletionItemKind.Keyword,
			detail: "Rotate accumulator to the left trough Carry flag",
			documentation:
			{
				kind: MarkupKind.Markdown,
				value:
`All of the bits contained in a operands will be shifted to the right
The bit stored in the Carry flag will be moved to the most significant bit (bit 7)
The least significant bit (bit 0) will be moved to the Carry flag

---

Syntax:
> \`\`\`asm8051
> RL A
> \`\`\`

---

Affected flags:
- Carry flag (*C*)

Value before
10100011
C = 0

Value after
01010001
C = 1`

			}
		}
	]
]);