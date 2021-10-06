import { CompletionItem, CompletionItemKind, MarkedString, MarkupContent, MarkupKind } from 'vscode-languageserver';

export const dataTransfer : Map<string,CompletionItem> = new Map([
	[
		"MOV",
		{
			label: "MOV",
			kind: CompletionItemKind.Keyword,
			detail: "Move data from second operand to first operand",
			documentation:
			{
				kind: MarkupKind.Markdown,
				value:
`Copy data from second operand and store it in the first operand, leaving the second operand unchanged

---

Syntax:
> \`\`\`asm8051
> MOV [operand1], [operand2] 
> MOV A, #41H
> MOV A, 05H
> MOV A, @R0
> MOV A, R2
> \`\`\`

---

Affected flags:
- Carry flag (*C*) if data is being copied to carry flag
- All of flags if data is being copied to PSW

---

Valid first operands:
- A (Accumulator)
- C (Carry flag)
- Internal ram address (e.g. 05H)
- Register with a data address @R0, @R1
- Register R0 trough R7
- Register DPTR

Valid second operands:
- A (Accumulator)
- C (Carry flag)
- Internal ram address (e.g. 05H)
- Bit address (e.g. 05H)
- Register with a data address @R0, @R1
- Register R0 trough R7
- Register DPTR
- Immediate data (e.g. #64H)
`

			}
		}
	],
	[
		"MOVC",
		{
			label: "MOVC",
			kind: CompletionItemKind.Keyword,
			detail: "Move byte from program memory into accumulator",
			documentation:
			{
				kind: MarkupKind.Markdown,
				value:
`Copy data from a byte in program memory which address is determined adding current Accumulator value to Program Counter or DPTR
Program Counter will be incremented before use

---

Syntax:
> \`\`\`asm8051
> MOVC A, @A+DPTR
> MOVC A, @A+PC
> \`\`\`

---

Valid operands:
- @A + Register DPTR
- @A + Program Counter
`
			}
		}
	],
	[
		"PUSH",
		{
			label: "PUSH",
			kind: CompletionItemKind.Keyword,
			detail: "Push onto stack",
			documentation:
			{
				kind: MarkupKind.Markdown,
				value:
`Copy value from address in RAM and push it on top of the stack

---

Syntax:
> \`\`\`asm8051
> PUSH 34H
> \`\`\`
`
			}
		}
	],
	[
		"POP",
		{
			label: "POP",
			kind: CompletionItemKind.Keyword,
			detail: "Pop value from the top of the stack",
			documentation:
			{
				kind: MarkupKind.Markdown,
				value:
`Copy data from the top of the stack to address in RAM and remove that data from the top of the stack 

---

Syntax:
> \`\`\`asm8051
> POP 34H
> \`\`\`
`
			}
		}
	],
	[
		"XCH",
		{
			label: "XCH",
			kind: CompletionItemKind.Keyword,
			detail: "Exchange data between Accumulator and operand",
			documentation:
			{
				kind: MarkupKind.Markdown,
				value:
`Copies an original data from Accumulator to an operand, and copies original data from operand to the Accumulator

---

Syntax:
> \`\`\`asm8051
> XCH A, [operand]
> XCH A, 05H
> XCH A, @R0
> XCH A, R2
> \`\`\`

---

Valid operands:
- Internal ram address (e.g. 05H)
- Register with a data address @R0, @R1
- Register R0 trough R7

---

Value before
A  = #21H
R2 = #43H

Value after
A  = #43H
R2 = #21H
`

			}
		}
	],
	[
		"XCHD",
		{
			label: "XCHD",
			kind: CompletionItemKind.Keyword,
			detail: "Exchange lower nibbles between Accumulator and RAM address",
			documentation:
			{
				kind: MarkupKind.Markdown,
				value:
`Copies lower nibble of original data from Accumulator (bits 0-3) to an operand, and copies lower nibble of original data from operand to the Accumulator

---

Syntax:
> \`\`\`asm8051
> XCHD A, @R0
> XCHD A, @R1
> \`\`\`

---

Valid operands:
- Register with a data address @R0, @R1

---

Value before
A   = #21H
@R1 = #43H

Value after
A   = #23H
@R1 = #41H
`

			}
		}
	],
]);