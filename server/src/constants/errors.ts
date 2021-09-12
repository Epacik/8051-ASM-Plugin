import * as log from 'loglevel'
import * as en from "./errors/en";
import * as pl from "./errors/pl";
import { Langs } from './langs';

export interface errLang{
	readonly missingLabel : string;
	readonly instructionNoWhitespace : string;
	readonly labelWithWhitespace : string;
	readonly missingOprand : string;
	readonly missingComa : string;
	readonly wrongOperand : string;
	readonly AWasExpected : string;
}

export class errors {
	public static missingLabel() :string {
		return errors.getLabel("missingLabel")
	};

	public static instructionNoWhitespace():string {
		return errors.getLabel("instructionNoWhitespace");
	};

	public static labelWithWhitespace():string {
		return errors.getLabel("labelWithWhitespace");
	};

	public static missingOprand():string {
		return errors.getLabel("missingOprand");
	};

	public static missingComa():string {
		return errors.getLabel("missingComa");
	};

	

	public static wrongOperand():string {
		return errors.getLabel("wrongOperand");
	};

	public static AWasExpected():string {
		return errors.getLabel("AWasExpected");
	};



	public static readonly setLang = (lang:string) => {
		log.info(`setting language to ${lang}`)
		errors.Lang = lang;
	}

	private static Lang = Langs.en;

	private static getProperty<T, K extends keyof T>(o: T, propertyName: K): T[K] {
		return o[propertyName]; // o[propertyName] is of type T[K]
	}

	private static Err : errLang = new en.errors();

	private static readonly  getLabel = (id: string) : string => {
		log.info(`Lang is ${errors.Lang}, getting ${id}`);
		if(errors.Lang == Langs.pl && !(errors.Err instanceof pl.errors)){
			errors.Err = new pl.errors();	
		}
		else if(errors.Lang == Langs.en && !(errors.Err instanceof en.errors)){
			errors.Err = new en.errors();
		}

		return errors.Err[id as keyof pl.errors];

		
	}
}