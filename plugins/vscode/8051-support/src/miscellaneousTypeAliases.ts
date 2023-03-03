export type NullishableString = string | undefined | null;
export const isNullishOrWhitespace = (str: NullishableString) => str === null || str === undefined || str.trim() === "";