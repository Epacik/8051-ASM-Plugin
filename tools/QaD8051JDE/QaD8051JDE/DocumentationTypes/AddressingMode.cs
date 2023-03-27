using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace QaD8051JDE.DocumentationTypes;

public enum AddressingMode
{
    Implied          = 0,
    Immediate        = 1,
    Register         = 2,
    Direct           = 3,
    RegisterIndirect = 4,
    Indexed          = 5,
}