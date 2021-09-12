import { Diagnostic, DiagnosticSeverity } from 'vscode-languageserver';
import { Diag } from './Diag';
import { errors } from '../constants/errors';
import { getKitLabels } from './kitSpecificDiags/kitSpecificLabels';
import * as log from 'loglevel'


export const basicsAndProgramControl : Diag[] = [
	new Diag(
		/[^\S\r\n][A-Za-z_]{1,}:/gm,
		errors.labelWithWhitespace,
		DiagnosticSeverity.Error),

	new Diag(
		/^\w+.*(?<!:)$/gm,
		errors.instructionNoWhitespace,
		DiagnosticSeverity.Error),
	
	new Diag(
		/\b(CALL|ACALL|LCALL|SJMP|JMP|AJMP|LJMP)\b[\s\	]{1,}[A-Za-z0-9_]{1,}/gm,
		errors.missingLabel,
		DiagnosticSeverity.Error,
		(id:string, text:string) => {

			//get name of the label
			let label = id
						.replace("LCALL", "")
						.replace("ACALL", "")
						.replace("CALL", "")
						.replace("SJMP", "")
						.replace("AJMP", "")
						.replace("LJMP", "")
						.replace("JMP", "")
						.trim();
			if(label.includes("CALL")){
				label = id.substring(id.lastIndexOf("\t"));
			}

			//shouldn't happen
			if(label == undefined) return false;

			label = label.trim();

			log.trace(`current label: ${label}`);

			if(getKitLabels().includes(label)) {
				log.trace("found label in predefined ones")
				return false;
			}
			//check if label exists
			let exists = text.includes(`${label}:`);

			//if()

			//check is whitespaces exists between new line and label
			let x = (new RegExp(`[^\\S\\r\\n](${label})+:`, "gm"));

			let matches = x.exec(text);
			log.trace(`exists: ${exists}`);
			log.trace(`matches: ${matches}`);

			return matches != null || !exists;
		}
	),
];