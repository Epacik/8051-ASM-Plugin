import { DiagnosticSeverity } from 'vscode-languageserver';
import { Diag, DiagNamedParameters } from './Diag';
import { errors } from '../constants/errors';

export const arithmetic : Diag[] = [

	
	new Diag(
		/(ADD|ADDC|SUBB)\s{0,}A(.*)/gm, //TODO: sprawdź czy zmiana {0,} na * sprawi że będzie działać dokładnie tak samo
		errors.missingComa,
		DiagnosticSeverity.Error,
		(id:string, text:string) => {
			return !id.includes(",");
		}
	),

	new Diag(
		/(ADD|ADDC|SUBB)\s{0,}(.*)/gm, //TODO: sprawdź czy zmiana {0,} na * sprawi że będzie działać dokładnie tak samo
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
	
	new Diag({
		_pattern: /(ADD|ADDC|SUBB)\s{0,}(.*)/gm,
		_messageFunc: errors.wrongOperand,
		_message2Func: errors.AWasExpected,
		_severity:DiagnosticSeverity.Error,
		_additionalTest: (id:string, text:string) => {
			//remove 
			let x = id.replace("ADD", "").replace("ADDC", "").replace("SUBB", "").trim();
			
			// if(x.includes(";")){
			// 	x = x.substring(0, id.lastIndexOf(";")).trim();
			// }
			
			// if(x.includes(",")){
			// 	x = x.substring(0, id.lastIndexOf(",")).trim();
			// }
	
			let res = !x.startsWith("A");
			
			return res;
		}
	}),
];
