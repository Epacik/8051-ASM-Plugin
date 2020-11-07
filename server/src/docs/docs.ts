

import { CompletionItem } from 'vscode-languageserver';
import { arthmetic } from './arithmetic';
import { boolean } from './boolean';
import { dataTransfer } from './dataTransfer';
import { logical } from './logical';
import { programControl } from './programControl';
import { registers } from './registers';

export namespace docs {
	let List : CompletionItem[] | undefined = undefined;

	export const getItems  = () : CompletionItem[] => {
		if(List === undefined){
			List = [];

			List = List.concat(arthmetic);
			List = List.concat(boolean);
			List = List.concat(dataTransfer);
			List = List.concat(logical);
			List = List.concat(programControl);
			List = List.concat(registers);
			
		}
		
		return <CompletionItem[]>List;
	}
}