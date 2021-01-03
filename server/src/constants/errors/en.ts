import {errLang} from "../errors"

export class errors implements errLang {
	readonly missingLabel : string = "(8051: E0001) Missing label"
	readonly instructionNoWhitespace : string = "(8051: E0002) Instructions have to start with a whitespace";
	readonly labelWithWhitespace : string = "(8051: E0003) Labels cannot start with a whitespace"
	readonly missingOprand : string = "(8051: E0004) Missing operand";
	readonly missingComa : string = "(8051: E0005) Missing comma";
	readonly wrongOperand : string = "(8051: E0006) Wrong operand";
	readonly AWasExpected : string = ", 'A' was expected";
}