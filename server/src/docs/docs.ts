

import { CompletionItem } from 'vscode-languageserver';
import { arthmetic } from './arithmetic';
import { boolean } from './boolean';
import { dataTransfer } from './dataTransfer';
import { logical } from './logical';
import { programControl } from './programControl';

export namespace docs {
	let List : CompletionItem[] | undefined = undefined;

	export const getItems  = () : CompletionItem[] => {

		if(List == undefined){
			let list : CompletionItem[] = [];

			list = list.concat(arthmetic);
			list = list.concat(boolean);
			list = list.concat(dataTransfer);
			list = list.concat(logical);
			list = list.concat(programControl);
			List = list;
		}
		
		return List;;	
	}
}