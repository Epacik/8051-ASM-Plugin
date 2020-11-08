import { DiagnosticSeverity } from 'vscode-languageserver';
import { Diag } from './Diag';
import { errors } from './errors';

export const arithmetic : Diag[] = [

	new Diag(
		/(ADD|ADDC|SUBB)\s{0,}A(.*)/gm,
		errors.missingComa,
		DiagnosticSeverity.Error,
		(id:string, text:string) => {
			return !id.includes(",");
		}
	),

	new Diag(
		/(ADD|ADDC|SUBB)\s{0,}(.*)/gm,
		errors.missingOprand,
		DiagnosticSeverity.Error, 
		(id:string, text:string) => {
			//remove 
			let x = id.trim();
			if(x.includes(";")){
				x = x.substring(0, id.lastIndexOf(";")).trim();
			}
			let res = x.endsWith(",") || x.endsWith("A");

			x = x.substring(0, id.lastIndexOf(",")).trim();
			res = res || x.endsWith("ADD") || x.endsWith("ADDC") || x.endsWith("SUBB");
			
			return res;
		}
	),
	
	new Diag(
		/(ADD|ADDC|SUBB)\s{0,}(.*)/gm,
		`${errors.wrongOperand}, 'A' was expected`,
		DiagnosticSeverity.Error, 
		(id:string, text:string) => {
			//remove 
			let x = id.trim();
			if(x.includes(";")){
				x = x.substring(0, id.lastIndexOf(";")).trim();
			}
			

			x = x.substring(0, id.lastIndexOf(",")).trim();

			let res = !x.endsWith("A");
			
			return res;
		}
	),
];