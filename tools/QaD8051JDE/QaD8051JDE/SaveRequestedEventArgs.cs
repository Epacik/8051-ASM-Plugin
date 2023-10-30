using DocumentationTypes;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace QaD8051JDE;
public class SaveRequestedEventArgs : EventArgs
{
    public SaveRequestedEventArgs(string? mnemonic, DocumentationElement documentation)
    {
        Mnemonic = mnemonic;
        NewValue = documentation;
    }
    public readonly string? Mnemonic;
    public readonly DocumentationElement NewValue;
}

