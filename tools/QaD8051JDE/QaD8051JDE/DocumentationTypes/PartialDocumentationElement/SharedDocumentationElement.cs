using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.Json.Serialization;
using System.Threading.Tasks;

namespace QaD8051JDE.DocumentationTypes.PartialDocumentationElement;

internal class SharedDocumentationElement
{
    [JsonPropertyName("valid_operands")]
    public List<List<ValidOperand>>? ValidOperands { get; set; }

    [JsonPropertyName("affected_flags")]
    public List<FlagType>? AffectedFlags { get; set; }

    [JsonPropertyName("dont_generate_syntax")]
    public bool DontGenerateSyntax { get; set; }

    [JsonPropertyName("dont_duplicate_in_all_docs")]
    public bool DontDuplicate { get; set; }

    [JsonPropertyName("prefix_required")]
    public bool PrefixRequired { get; set; }

    [JsonPropertyName("addressing_modes")]
    public List<AddressingMode>? AddressingModes { get; set; }
}
