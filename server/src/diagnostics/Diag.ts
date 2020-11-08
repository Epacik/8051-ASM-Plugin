import { DiagnosticRelatedInformation, DiagnosticSeverity } from 'vscode-languageserver';
import { boolean } from '../docs/boolean';

type AdditionalTest = (id: string,text:string) => boolean;

export class Diag {
	constructor(_pattern : RegExp, _message : string, _severity : DiagnosticSeverity, _additionalTest?:AdditionalTest, _relatedInformation?: DiagnosticRelatedInformation[]) {
		this.severity = _severity;
		this.message = _message;
		this.relatedInformation = _relatedInformation;
		this.pattern = _pattern
		this.additionalTest = _additionalTest;
	}
	pattern : RegExp
	severity : DiagnosticSeverity;
	message : string;
	relatedInformation?: DiagnosticRelatedInformation[];
	additionalTest?:AdditionalTest;
}