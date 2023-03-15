'use strict';
import { NullishableString } from "../miscellaneousTypeAliases";

export default interface IDocumentation {
    detail: NullishableString;
    description: NullishableString,
    syntax: NullishableString,
    affected_flags: NullishableString,
    valid_operands: NullishableString,
    category: string,
    label: string,
}