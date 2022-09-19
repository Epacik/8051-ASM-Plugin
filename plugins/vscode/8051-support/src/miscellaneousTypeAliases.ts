export type nullishableString = string | undefined | null;
export const isNullishOrWhitespace = (str: nullishableString) => str === null || str === undefined || str.trim() === "";