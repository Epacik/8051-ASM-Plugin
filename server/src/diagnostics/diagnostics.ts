import { Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, TextDocument } from 'vscode-languageserver';
import * as log from 'loglevel'
import { DocSettings } from "../server";
import { arithmetic } from './arithmetic';
import { basicsAndProgramControl } from './basicsAndProgramControl';
import { Diag } from './Diag';

export namespace diagnostics{
	let currentKit : string = "generic";

	export const getDisgnostics = (textDocument: TextDocument ,_settings : DocSettings) : Diagnostic[] => {
		let diag: Diagnostic[] = [];
		let txt = textDocument.getText();

		if(diags == undefined || _settings.kit != currentKit){
			log.info("getting new disgnostics")
			currentKit = _settings.kit;
			diags = [];
			diags = diags.concat(basicsAndProgramControl);
			diags = diags.concat(arithmetic);
		}

		diag = diag.concat(generateDiags(txt, _settings, textDocument));

		return diag;
	}
}

let diags:Diag[] | undefined;

const generateDiags = (_docText : string ,_settings : DocSettings, textDocument: TextDocument) : Diagnostic[] => {
	let diag: Diagnostic[] = [];

	let txt = _docText;
	let m: RegExpExecArray | null;

	if(diags == undefined) return [];

	diags.forEach(d => {
		while ((m = d.pattern.exec(txt))) {
				
				let diagnostic: Diagnostic = {
					severity:d.severity,
					range: {
						start: textDocument.positionAt(m.index),
						end: textDocument.positionAt(m.index + m[0].length)
					},
					message: d.message,
					source: m[0]
				};
				if (d.relatedInformation != undefined) {
					diagnostic.relatedInformation  ;
				}

				let txt2 = _docText;
				if(d.additionalTest != undefined && d.additionalTest(m[0], txt2)){
					diag.push(diagnostic);
				}
				else if(d.additionalTest == undefined){
					diag.push(diagnostic);
				}
				
			}
	});

	return diag;
}

