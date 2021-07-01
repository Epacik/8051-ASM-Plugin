export namespace string {
	export const isEmptyOrWhitespace = (value : string) : boolean => {
		return value === null || value === undefined || value.trim().length === 0;
	}
}