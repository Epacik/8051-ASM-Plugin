using CommunityToolkit.Mvvm.ComponentModel;
using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace QaD8051JDE.ViewModels;

public partial class ValidOperandPositionViewModel : ObservableObject
{
    public ValidOperandPositionViewModel(IEnumerable<ValidOperand> operands, int position)
    {
        Operands = new(
            operands
            .GroupBy(x => x.Operand)
            .Select(
                x => new ValidOperandViewModel(x.Key, x.Select(x => x.WhenFirstIs), position == 0)));
        Position = position;
    }

    [ObservableProperty]
    private int _position;

    [ObservableProperty]
    private ObservableCollection<ValidOperandViewModel>? _operands;

    [ObservableProperty]
    private ValidOperandViewModel? _selectedOperand;

    public void AddOperand()
    {
        Operands?.Add(new ValidOperandViewModel(PossibleOperands.Any, Array.Empty<PossibleOperands>(), Position == 0));
    }

    public void RemoveOperand()
    {
        if (SelectedOperand is null)
            return;

        Operands?.Remove(SelectedOperand);
        SelectedOperand = null;

    }
}
