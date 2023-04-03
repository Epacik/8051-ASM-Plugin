/* eslint-disable @typescript-eslint/naming-convention */
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
    addressing_modes: NullishableString,
    stack_space_needed: NullishableString,
    used_registers: NullishableString,
    changed_registers: NullishableString
}