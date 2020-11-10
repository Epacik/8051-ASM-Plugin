export namespace Enum{
	//check if enum has a flag set
	export const hasFlag = (variable:number, flag:number|undefined):boolean => {
		if(variable == undefined || flag == undefined || typeof variable != "number" || typeof flag != "number") return false;
		return (variable & flag) == flag;
	}
}