import { Diagnostic, DiagnosticSeverity, TextDocument } from 'vscode-languageserver';
import { DocSettings } from '../server';
import { Diag } from './Diag';
import { errors } from './errors';







export const basicsAndProgramControl : Diag[] = [
	new Diag(
		/[^\S\r\n][A-Za-z_]{1,}:/gm,
		errors.labelWithWhitespace,
		DiagnosticSeverity.Error),

	new Diag(
		/^\n\w+.*(?<!:)$/gm,
		errors.instructionNoWhitespace,
		DiagnosticSeverity.Error),
	
	new Diag(
		/^\s{1,}(CALL|ACALL|LCALL)[\s\	]{1,}[A-Za-z_]{1,}/gm,
		errors.missingLabel,
		DiagnosticSeverity.Error,
		(id:string, text:string) => {

			//get name of the label
			let label = id.substring(id.lastIndexOf(" "));
			if(label.includes("CALL")){
				label = id.substring(id.lastIndexOf("\t"));
			}

			//shouldn't happen
			if(label == undefined) return false;

			label = label.trim();

			//check if label exists
			let exists = text.includes(`${label}:`)
			//check is whitespaces exists between new line and label
			let x = (new RegExp(`[^\\S\\r\\n](${label})+:`, "gm"));

			let matches = x.exec(text);

			return matches != null || !exists;
		}
	),
];