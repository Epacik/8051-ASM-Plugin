import {errLang} from "../errors"
export class errors implements errLang {
	readonly missingLabel : string = "(8051: E0001) Brakująca etykieta"
	readonly instructionNoWhitespace : string = "(8051: E0002) Instrukcje muszą zaczynać się od białego znaku";
	readonly labelWithWhitespace : string = "(8051: E0003) Etykiety nie mogą zaczynać się od białego znaku"
	readonly missingOprand : string = "(8051: E0004) Brakujący operand";
	readonly missingComa : string = "(8051: E0005) Brakujący przecinek";
	readonly wrongOperand : string = "(8051: E0006) Niewłaściwy operand";
	readonly AWasExpected : string = ", oczekiwano 'A'";
}