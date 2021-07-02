import { CompletionItem } from 'vscode-languageserver';
import { eng } from './eng/eng';
import { pol } from './pol/pol';


/**
 * test
 */
export namespace docs {
	/**
	 * Array of CompletionItems used to store documentation popups 
	 */
	let Pol : (Map<string, CompletionItem> | undefined) = undefined;

	
	/**
	 * @returns An array of CompletionItem
	 */
	export const getItems  = (language : (string | undefined) ) : Map<string,CompletionItem> => {
		language ??= "english";
		if(Pol === undefined){
			Pol = new Map([
				...eng,
				...pol,
			]);

		}
		
		switch(language)
		{
			case "polish":
			case "polski":
				return <Map<string,CompletionItem>>Pol;
			
			default:
				return <Map<string,CompletionItem>>eng;

		}

		
	}

	export const getSystemSpecificSubroutines = () : CompletionItem[] => {

		return [];
	}
}