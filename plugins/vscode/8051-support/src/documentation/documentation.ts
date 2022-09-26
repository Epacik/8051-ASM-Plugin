import { nullishableString } from "../miscellaneousTypeAliases";

export default interface IDocumentation {
    detail: nullishableString;
    description: nullishableString,
    syntax: nullishableString,
    affected_flags: nullishableString,
    valid_operands: nullishableString,
    category: string,
    label: string,
}