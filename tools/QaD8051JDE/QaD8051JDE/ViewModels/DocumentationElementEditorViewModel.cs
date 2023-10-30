using CommunityToolkit.Mvvm.ComponentModel;
using DocumentationTypes;
using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace QaD8051JDE.ViewModels;

public partial class DocumentationElementEditorViewModel : BaseViewModel
{
    public DocumentationElementEditorViewModel(DocumentationElement? item)
    {
        if (item is null)
        {
            return;
        }
        List<List<ValidOperand>>? validOperands;
        List<Flag>? affectedFlags;

        (Detail, Description, validOperands, affectedFlags, DontGenerateSyntax, DontDuplicate, Prefix, PrefixRequired, Label, RequiredStackSpace) = item;

        ValidOperands = new(validOperands!.Select((x, p) => new ValidOperandPositionViewModel(x, p)));
        AffectedFlags = new(affectedFlags!.Select(x => new FlagEditorViewModel(x)));

        AddressingModes = new List<NamedItemViewModel<AddressingMode>>
        {
            new("Implied", AddressingMode.Implied),
            new("Immediate", AddressingMode.Immediate),
            new("Register", AddressingMode.Register),
            new("Direct", AddressingMode.Direct),
            new("Register Indirect", AddressingMode.RegisterIndirect),
            new("Indexed", AddressingMode.Indexed),
        };

        Registers = new List<NamedItemViewModel<PossibleRegister>>
        {
            new ("Accumulator", PossibleRegister.Accumulator),
            new ("B Register", PossibleRegister.B),
            new ("DPTR", PossibleRegister.DPTR),
            new ("PSW", PossibleRegister.PSW),
            new ("R0", PossibleRegister.R0),
            new ("R1", PossibleRegister.R1),
            new ("R2", PossibleRegister.R2),
        };


        foreach (var mode in AddressingModes.Where(x => item.AddressingModes?.Contains(x.Item) == true))
        {
            SelectedAddressingModes.Add(mode);
        }

        foreach (var register in Registers.Where(x => item.UsedRegisters?.Contains(x.Item) == true))
        {
            SelectedUsedRegisters.Add(register);
        }
        
        foreach (var register in Registers.Where(x => item.ChangedRegisters?.Contains(x.Item) == true))
        {
            SelectedChangedRegisters.Add(register);
        }
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
    private ObservableCollection<NamedItemViewModel<AddressingMode>> _selectedAddressingModes = new();

    [ObservableProperty]
    private byte? _requiredStackSpace;

    [ObservableProperty]
    private IEnumerable<NamedItemViewModel<PossibleRegister>> _registers;

    [ObservableProperty]
    private ObservableCollection<NamedItemViewModel<PossibleRegister>> _selectedUsedRegisters = new();

    [ObservableProperty]
    private ObservableCollection<NamedItemViewModel<PossibleRegister>> _selectedChangedRegisters = new();

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
        ValidOperands!.Add(new(Array.Empty<ValidOperand>(), ValidOperands.Count));
    }

    public void RemoveValidOperandPosition()
    {
        if (SelectedValidOperands is null)
            return;

        ValidOperands?.Remove(SelectedValidOperands);
        SelectedValidOperands = null;
    }


    public DocumentationElement AsDocumentationElement()
    {
        var element = new DocumentationElement
        {
            Detail = Detail,
            Description = Description,
            DontDuplicate = DontDuplicate,
            DontGenerateSyntax = DontGenerateSyntax,
            Prefix = Prefix,
            PrefixRequired = PrefixRequired,
            Label = Label,
            ValidOperands = new(),
            AffectedFlags = new(
                AffectedFlags
                !.Select(flag => new Flag
            {
                FlagType = flag.FlagType.Item,
                WhenSet = flag.WhenSet,
                WhenUnset = flag.WhenUnset,
            })),
            AddressingModes = new(SelectedAddressingModes.Select(x => x.Item)),
            StackSpaceNeeded = RequiredStackSpace,
            UsedRegisters = new(SelectedUsedRegisters.Select(x => x.Item)),
            ChangedRegisters = new(SelectedChangedRegisters.Select(x => x.Item)),
        };

        foreach (var position in ValidOperands!)
        {
            var operandList = new List<ValidOperand>();

            foreach (var operand in position.Operands!)
            {
                if (operand.WhenFirstIs?.Count == 0)
                {
                    operandList.Add(new ValidOperand
                    {
                        Operand = operand.Operand.Item,
                        WhenFirstIs = PossibleOperands.Any,
                    });
                }
                else
                {
                    foreach (var wfi in operand.WhenFirstIs!)
                    {
                        operandList.Add(new ValidOperand
                        {
                            Operand = operand.Operand.Item,
                            WhenFirstIs = wfi.Item
                        });
                    }
                }
            }

            element.ValidOperands.Add(operandList);
        }

        return element;
    }
}
