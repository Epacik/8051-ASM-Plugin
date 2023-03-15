using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.Json.Serialization;
using System.Threading.Tasks;

namespace QaD8051JDE.DocumentationTypes
{
    public interface IOperand<TEnum>
        where TEnum : struct, IConvertible, IEquatable<TEnum>
    {
        TEnum Operand { get; set; }
        TEnum WhenFirstIs { get; set; }
    }
}
