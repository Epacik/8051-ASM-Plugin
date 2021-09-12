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
			detail: "Add Accumulator",
			documentation: {
				kind: MarkupKind.Markdown,
				value:
	`
Adds a byte value to a value stored in the accumulator, and stores the results back in the accumulator.

---
Syntax:
> \`\`\`asm8051
> ADD A,  [operand] 
> ADD A,  #41H
> ADD A,  05H
> ADD A,  @R0
> ADD A,  R2
> \`\`\`
---

Valid operands:

- #number (e.g. #41H, #100101B)
- Internal ram address (e.g. 05H),
- Address stored in register @R0 or @R1
- Register R0 trough R7

---

Affected flags:
- **Carry** set if result exceedes 255, cleared if it does not
- **Auxillary Carry** set if result exceedes 15, cleared if it does not
- **Overflow** set if result is out of signed byte value (-128 trough 127), cleared if it does not
	`.trim(),
				
			},
		}
	],
	[
		"ADDC",
		{
			label: "ADDC",
			kind: CompletionItemKind.Keyword,
			detail: "Add Accumulator With Carry",
			documentation: {
				kind: MarkupKind.Markdown,
				value: 
	`
Adds a byte value and a carry flag to the accumulator, and stores the results in the accumulator.

---

Syntax:
> \`\`\`asm8051
> ADDC A, [operand]
> ADDC A, #41H
> ADDC A, 05H
> ADDC A, @R0
> ADDC A, R2
> \`\`\`

---

Valid operands:

- #number (e.g. #41H)
- Internal ram address (e.g. 05H),
- Register with a data address @R0, @R1
- Register R0 trough R7

---

Affected flags:
- **Carry** set if result exceedes 255, cleared if it does not
- **Auxillary Carry** set if result exceedes 15, cleared if it does not
- **Overflow** set if result is out of signed byte value (-128 trough 127), cleared if it does not
	`.trim(),
				
			},
		}
	],
	[
		"SUBB",
		{
			label: "SUBB",
			kind: CompletionItemKind.Keyword,
			detail: "Subtract from Accumulator With Borrow",
			documentation: {
				kind: MarkupKind.Markdown,
				value: 
	`
Subtract the value of operand and the Carry Flag from the value of the Accumulator, and stores the results in the accumulator.

---

Syntax:
> \`\`\`asm8051
> SUBB A, [operand] 
> SUBB A, #41H
> SUBB A, 05H
> SUBB A, @R0
> SUBB A, R2
> \`\`\`

---

Valid operands:

- #number (e.g. #41H)
- Internal ram address (e.g. 05H),
- Register with a data address @R0, @R1
- Register R0 trough R7

---

Affected flags:
- **Carry** set if operand value was greater than Accumulator value, cleared if it was not
- **Auxillary Carry** set if lower nibble of operand (bits 0 trough 3) was greater than value of lower nibble of an Accumulator, cleared if it was not
- **Overflow** set if result is out of signed byte value (-128 trough 127), cleared if it does not
	`.trim(),
				
			},
		}
	],
	[
		"INC",
		{
			label: "INC",
			kind: CompletionItemKind.Keyword,
			detail: "Increment Register",
			documentation: {
				kind: MarkupKind.Markdown,
				value: 
	`
Increases the value of the operand by 1.

If value of the operand was set to 255 (or 65535 in case of DPTR), the result will be set to 0

**When that happens the carry flag WILL NOT be set**

---

Syntax:
> \`\`\`asm8051
> INC [operand]
> INC A
> INC 05H
> INC @R1
> INC R5
> INC DPTR
> \`\`\`

---

Valid operands:

- A (Accumulator)
- Internal ram address (e.g. 05H),
- Register with a data address @R0, @R1
- Register R0 trough R7
- Register DPTR
	`.trim(),
				
			},
		}
	],
	[
		"DEC",
		{
			label: "DEC",
			kind: CompletionItemKind.Keyword,
			detail: "Decrement Register",
			documentation: {
				kind: MarkupKind.Markdown,
				value: 
	`
Decreases the value of the operand by 1.

If value of the operand was set to 0, the result will be set to 255

**When that happens the carry flag WILL NOT be set**

---

Syntax:
> \`\`\`asm8051
> DEC [operand]
> DEC A
> DEC 05H
> DEC @R1
> DEC R5
> \`\`\`

---

Valid operands:

- A (Accumulator)
- Internal ram address (e.g. 05H),
- Register with a data address @R0, @R1
- Register R0 trough R7
	`.trim(),
				
			},
		}
	],
	[
		"MUL",
		{
			label: "MUL",
			kind: CompletionItemKind.Keyword,
			detail: "Multiply Accumulator by B",
			documentation: {
				kind: MarkupKind.Markdown,
				value: 
	`
Multiplies the unsigned values of the accumulator and the register B.

The result is a 16 bit unsigned number, lower half of which will be stored in the Accumulator, and the higher half in the B register.

---

Syntax:
> \`\`\`asm8051
> MUL AB
> \`\`\`

---

Affected flags:
- **Carry** cleared
- **Overflow** set if result is greater than 255, cleared otherwise
	`.trim(),
				
			},
		}
	],
	[
		"DIV",
		{
			label: "DIV",
			kind: CompletionItemKind.Keyword,
			detail: "Divide Accumulator by B",
			documentation: {
				kind: MarkupKind.Markdown,
				value: 
	`

Divides the unsigned values of the accumulator and the register B.

Resulting quotient is stored in the Accumulator, and the reminder in the B register

---

Syntax:
> \`\`\`asm8051
> DIV AB
> \`\`\`

---

Affected flags:
- **Carry** cleared
- **Overflow** set if division by 0 was attempted, cleared otherwise
	`.trim(),
				
			},
		}
	],
	[
		"DA",
		{
			label: "DA",
			kind: CompletionItemKind.Keyword,
			detail: "Decimal Adjust Accumulator",
			documentation: {
				kind: MarkupKind.Markdown,
				value: 
	`
Adjusts the result of adding two BCD formatted numbers to a BCD format.

---

Steps:
1. 06H is added if lower nibble (bits 0 trough 3) is greater than 9 or if the Auxillary Carry flag is set
2. 60H is added if higher nibble (bits 4 trough 7) is greater than 9 or if Carry flag is set

---

**DA cannot convert a hexadecimal number in the accumulator to a BCD format**
	
---

Syntax:	
> \`\`\`asm8051
> DA A
> \`\`\`

---

Affected flags:
- **Carry** set if result is greater than 99H (or if the initial value was greater than 63H), cleared otherwise
	`.trim(),
				
			},
		}
	]
]);



