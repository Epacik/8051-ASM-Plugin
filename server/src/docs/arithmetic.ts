import { CompletionItem, CompletionItemKind, MarkedString, MarkupContent, MarkupKind } from 'vscode-languageserver';

/**
 * List of documentation popups for arthmetic operations
 */
export const arthmetic : CompletionItem[]  = [
	{
		label: "ADD",
		kind: CompletionItemKind.Keyword,
		detail: "Add Accumulator",
		documentation: {
			kind: MarkupKind.Markdown,
			value:
`
The ADD instruction adds a byte value to the accumulator and stores the results back in the accumulator. Several of the flag registers are affected.\n
\nSyntax:\n
\`\`\`asm8051
ADD A, operand
\`\`\`

Valid operands:
#number (e.g. #41H), ram_address (e.g. 05H), @R0, @R1, R0, R1, R2, R3, R4, R5, R6, R7
`.trim(),
			
		},
	},

	
	{
		label: "ADDC",
		kind: CompletionItemKind.Keyword,
		detail: "Add Accumulator With Carry",
		documentation: {
			kind: MarkupKind.Markdown,
			value: 
`
The ADDC instruction adds a byte value and the value of the carry flag to the accumulator. The results of the addition are stored back in the accumulator. Several of the flag registers are affected.\n
The **Carry bit (C)** is set if there is a carry-out of bit 7. In other words, if the unsigned summed value of the Accumulator, operand and (in the case of ADDC) the Carry flag exceeds 255 Carry is set. Otherwise, the Carry bit is cleared.

The **Auxillary Carry (AC)** bit is set if there is a carry-out of bit 3. In other words, if the unsigned summed value of the low nibble of the Accumulator, operand and (in the case of ADDC) the Carry flag exceeds 15 the Auxillary Carry flag is set. Otherwise, the Auxillary Carry flag is cleared.

The **Overflow (OV)** bit is set if there is a carry-out of bit 6 or out of bit 7, but not both. In other words, if the addition of the Accumulator, operand and (in the case of ADDC) the Carry flag treated as signed values results in a value that is out of the range of a signed byte (-128 through +127) the Overflow flag is set. Otherwise, the Overflow flag is cleared.
\nSyntax:\n

\`\`\`asm8051
ADDC A, operand
\`\`\`

Valid operands:
#number (e.g. #41H), ram_address (e.g. 05H), @R0, @R1, R0, R1, R2, R3, R4, R5, R6, R7
`.trim(),
			
		},
	},
	{
		label: "SUBB",
		kind: CompletionItemKind.Keyword,
		detail: "Subtract from Accumulator With Borrow",
		documentation: {
			kind: MarkupKind.Markdown,
			value: 
`
The SUBB instruction subtract the value of operand and the Carry Flag from the value of the Accumulator, leaving the resulting value in the Accumulator.
The value operand is not affected.

The Carry Bit (C) is set if a borrow was required for bit 7, otherwise it is cleared.
In other words, if the unsigned value being subtracted is greater than the Accumulator the Carry Flag is set.

The Auxillary Carry (AC) bit is set if a borrow was required for bit 3, otherwise it is cleared.
In other words, the bit is set if the low nibble of the value being subtracted was greater than the low nibble of the Accumulator.

The Overflow (OV) bit is set if a borrow was required for bit 6 or for bit 7, but not both.
In other words, the subtraction of two signed bytes resulted in a value outside the range of a signed byte (-128 to 127). Otherwise it is cleared.
\nSyntax:\n

\`\`\`asm8051
SUBB A,operand
\`\`\`

Valid operands:
#number (e.g. #41h), ram_address (e.g. 05H), @R0, @R1, R0, R1, R2, R3, R4, R5, R6, R7
`.trim(),
			
		},
	},
	{
		label: "INC",
		kind: CompletionItemKind.Keyword,
		detail: "Increment Register",
		documentation: {
			kind: MarkupKind.Markdown,
			value: 
`
INC increments the value of register by 1. If the initial value of register is 255 (0xFF Hex), incrementing the value will cause it to reset to 0. Note: The Carry Flag is NOT set when the value "rolls over" from 255 to 0.

In the case of "INC DPTR", the value two-byte unsigned integer value of DPTR is incremented. If the initial value of DPTR is 65535 (0xFFFF Hex), incrementing the value will cause it to reset to 0. Again, the Carry Flag is NOT set when the value of DPTR "rolls over" from 65535 to 0
\nSyntax:\n

\`\`\`asm8051
INC operand
\`\`\`

Valid operands:
A, ram_address (e.g. 05H), @R0, @R1, R0, R1, R2, R3, R4, R5, R6, R7, DPTR
`.trim(),
			
		},
	},
	{
		label: "DEC",
		kind: CompletionItemKind.Keyword,
		detail: "Decrement Register",
		documentation: {
			kind: MarkupKind.Markdown,
			value: 
`
DEC decrements the value of register by 1. If the initial value of register is 0, decrementing the value will cause it to reset to 255 (0xFF Hex). Note: The Carry Flag is NOT set when the value "rolls over" from 0 to 255.

\nSyntax:\n

\`\`\`asm8051
DEC operand
\`\`\`

Valid operands:
A, ram_address (e.g. 05H), @R0, @R1, R0, R1, R2, R3, R4, R5, R6, R7, DPTR
`.trim(),
			
		},
	},

	{
		label: "MUL",
		kind: CompletionItemKind.Keyword,
		detail: "Multiply Accumulator by B",
		documentation: {
			kind: MarkupKind.Markdown,
			value: 
`
Multiples the unsigned value of the Accumulator by the unsigned value of the "B" register. The least significant byte of the result is placed in the Accumulator and the most-significant-byte is placed in the "B" register.

The Carry Flag (C) is always cleared.

The Overflow Flag (OV) is set if the result is greater than 255 (if the most-significant byte is not zero), otherwise it is cleared.

\nSyntax:\n

\`\`\`asm8051
MUL AB
\`\`\`
`.trim(),
			
		},
	},

	{
		label: "DIV",
		kind: CompletionItemKind.Keyword,
		detail: "Divide Accumulator by B",
		documentation: {
			kind: MarkupKind.Markdown,
			value: 
`
Divides the unsigned value of the Accumulator by the unsigned value of the "B" register. The resulting quotient is placed in the Accumulator and the remainder is placed in the "B" register.

The Carry flag (C) is always cleared.

The Overflow flag (OV) is set if division by 0 was attempted, otherwise it is cleared.

\nSyntax:\n

\`\`\`asm8051
DIV AB
\`\`\`
`.trim(),
			
		},
	},

]
