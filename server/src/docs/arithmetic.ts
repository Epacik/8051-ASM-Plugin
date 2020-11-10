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

Possible operands:
#number (e.g. #41h), ram_address (e.g. 05H), @R0, @R1, R0, R1, R2, R3, R4, R5, R6, R7
`.trim(),
			
		},
	}

]
