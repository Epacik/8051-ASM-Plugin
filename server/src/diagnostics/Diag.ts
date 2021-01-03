import { Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity } from 'vscode-languageserver';
import { boolean } from '../docs/boolean';

type AdditionalTest = (id: string,text:string) => boolean;

export interface DiagNamedParameters{
	_pattern : RegExp,
	_messageFunc : ()=> string,
	_severity : DiagnosticSeverity, 
	_additionalTest?:AdditionalTest, 
	_relatedInformation?: DiagnosticRelatedInformation[], 
	_message2Func?:()=> string
}

export class Diag {
	constructor(params: DiagNamedParameters);
	constructor(_pattern : RegExp | DiagNamedParameters, _messageFunc : ()=> string, _severity : DiagnosticSeverity, _additionalTest?:AdditionalTest, _relatedInformation?: DiagnosticRelatedInformation[], _message2Func?:()=> string);

	constructor(_patternOrNamedParams : RegExp | DiagNamedParameters, _messageFunc?: ()=> string, _severity?: DiagnosticSeverity, _additionalTest?:AdditionalTest, _relatedInformation?: DiagnosticRelatedInformation[], _message2Func?:()=> string) {
		if(_patternOrNamedParams instanceof RegExp){
			this.severity = _severity ?? (DiagnosticSeverity.Error);
			this.msg = _messageFunc ?? ((): string => "");
			this.relatedInformation = _relatedInformation;
			this.pattern = _patternOrNamedParams
			this.additionalTest = _additionalTest;
			this.msg2 = _message2Func ?? ((): string => "");
		}
		else {
			const x = _patternOrNamedParams;
			this.severity = x._severity;
			this.msg = x._messageFunc;
			this.relatedInformation = x._relatedInformation;
			this.pattern = x._pattern
			this.additionalTest = x._additionalTest;
			this.msg2 = x._message2Func ?? ((): string => "");
		}
		
	}
	pattern : RegExp
	severity : DiagnosticSeverity;
	private msg : () => string = () => "";
	private msg2 : () => string = () => "";
	get message ():string {
		return `${this.msg()} ${this.msg2()}`;
	}
	relatedInformation?: DiagnosticRelatedInformation[];
	additionalTest?:AdditionalTest;
}