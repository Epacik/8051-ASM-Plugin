

import { CompletionItem } from 'vscode-languageserver';
import { arthmetic } from './arithmetic';
import { boolean } from './boolean';
import { dataTransfer } from './dataTransfer';
import { logical } from './logical';
import { programControl } from './programControl';
import { registers } from './registers';

/**
 * test
 */
export namespace docs {
	/**
	 * Array of CompletionItems used to store documentation popups 
	 */
	let List : CompletionItem[] | undefined = undefined;

	
	/**
	 * @returns An array of CompletionItem
	 */
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