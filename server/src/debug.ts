import { Enum } from './tools/enum';

/**
 * Tools for debugging
 */
export namespace debug {

	/**
	 * Types of messages that can be displayed in console
	 */
	export enum LogLevel {
		Errors = 1,
		Warns = 2,
		Logs = 4,	
		Info = 8,

		All = Errors | Warns | Logs | Info,

	}	

	/**
	 * Used in {@link DebugOptions} to provide named parameters in a constructor
	 */
	export interface DebugOptionsParameters{
		/**
		 * If set to instance of {@link DebugOptions} then values of properties will be copied to new object instance
		 */
		oldOptions?: DebugOptions;

		/**
		 * Defines what type of messages should be displayed in the console
		 */
		logLevel?: LogLevel;
	}

	/**
	 * 
	 */
	export class DebugOptions {

		/**
		 * Defines what type of messages should be displayed in the console
		 */
		logLevel: LogLevel = LogLevel.Errors;

		/**
		 * @param  {DebugOptions} oldOptions - options to be copied to new object
		 * @param  {LogLevel} logLevel - which messages are allowed
		 */
		constructor({ oldOptions, logLevel }: DebugOptionsParameters)
		{

			//setting values from old options object
			if(oldOptions != undefined){
				for (const key in oldOptions) {
					const K : string = key.toString();
					(<any>this)[K] = (<any>oldOptions)[K];
				}
			}

			//overriting options
			if(logLevel != undefined) this.logLevel = logLevel;

		}
	}

	
	/**
	 * Sets debug options 
	 * @param  {DebugOptions} _options 
	 */
	export const setOptions = (_options : DebugOptions) => {
		
		options = _options;
	}


	export const getOptions = () : DebugOptions => options;

	let options: DebugOptions;

	/**
	 * Prints log in console
	 * @param text Text to print in console
	 */
	export const log = (text : string)=>{
		//return if not configured or logLevel is not allowing to log
		if (options == undefined || options == null || !Enum.hasFlag(options.logLevel, LogLevel.Logs)) return;
		console.log(`Log: ${text}`);
	}

	/**
	 * Prints warning in console
	 * @param text Text to print in console
	 */
	export const warn = (text : string)=>{
		//return if not configured or logLevel is not allowing to warn
		if (options == undefined || options == null || !Enum.hasFlag(options.logLevel, LogLevel.Warns)) return;
		console.log(`!Warn!: ${text}`);
	}

	/**
	 * Prints error in console
	 * @param text Text to print in console
	 */
	export const error = (text : string)=>{
		//return if not configured or logLevel is not allowing to error
		if (options == undefined || options == null || !Enum.hasFlag(options.logLevel, LogLevel.Errors)) return;
		console.log(`!!!ERROR!!!: ${text}`);
	}

	/**
	 * Prints information in console
	 * @param text Text to print in console
	 */
	export const info = (text : string)=>{
		//return if not configured or logLevel is not allowing to error
		if (options == undefined || options == null || !Enum.hasFlag(options.logLevel, LogLevel.Info)) return;
		console.log(`Info: ${text}`);
	}
}