using CommunityToolkit.Mvvm.ComponentModel;
using QaD8051JDE.DocumentationTypes;
using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace QaD8051JDE.ViewModels;

public partial class DocumentationElementEditorViewModel : ObservableObject
{
    public DocumentationElementEditorViewModel(DocumentationElement? item)
    {
        if (item is null)
        {
            return;
        }
        List<List<ValidOperand>>? validOperands;
        List<Flag>? affectedFlags;

        (Detail, Description, validOperands, affectedFlags, DontGenerateSyntax, DontDuplicate, Prefix, PrefixRequired, Label) = item;

        ValidOperands = new(validOperands!.Select((x, p) => new ValidOperandPositionViewModel(x, p)));
        AffectedFlags = new(affectedFlags!.Select(x => new FlagEditorViewModel(x)));


    }

    [ObservableProperty]
    private string? _detail;

    [ObservableProperty]
    private string? _description;

    [ObservableProperty]
    private ObservableCollection<FlagEditorViewModel>? _affectedFlags;

    [ObservableProperty]
    private FlagEditorViewModel? _selectedFlag;

    [ObservableProperty]
    private ObservableCollection<ValidOperandPositionViewModel>? _validOperands;

    [ObservableProperty]
    private ValidOperandPositionViewModel? _selectedValidOperands;

    [ObservableProperty]
    private bool _dontGenerateSyntax;

    [ObservableProperty]
    private bool _dontDuplicate;

    [ObservableProperty]
    private string? _prefix;

    [ObservableProperty]
    private bool _prefixRequired;

    [ObservableProperty]
    private string? _label;

    [ObservableProperty]
    private IEnumerable<NamedItemViewModel<AddressingMode>> _addressingModes;

    [ObservableProperty]
    private ObservableCollection<NamedItemViewModel<AddressingMode>> _selectedAddressingModes;

    public void AddAffectedFlag()
    {
        AffectedFlags?.Add(new FlagEditorViewModel());
    }

    public void RemoveAffectedFlag()
    {
        if (SelectedFlag is null)
            return;

        AffectedFlags?.Remove(SelectedFlag);
        SelectedFlag = null;
    }

    public void AddValidOperandPosition()
    {
        ValidOperands.Add(new(Array.Empty<ValidOperand>(), ValidOperands.Count));
    }

    public void RemoveValidOperandPosition()
    {
        if (SelectedValidOperands is null)
            return;

        ValidOperands?.Remove(SelectedValidOperands);
        SelectedValidOperands = null;
    }
}
