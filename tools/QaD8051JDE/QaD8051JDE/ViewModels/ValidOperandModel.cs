using CommunityToolkit.Mvvm.ComponentModel;
using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Collections.Specialized;
using System.Linq;
using System.Runtime.CompilerServices;
using System.Text;
using System.Threading.Tasks;

namespace QaD8051JDE.ViewModels;

public partial class ValidOperandViewModel : ObservableObject
{
    [ObservableProperty]
    private IEnumerable<NamedItemViewModel<PossibleOperands>> _operandOptions;

    [ObservableProperty]
    private NamedItemViewModel<PossibleOperands>? _operand;
    partial void OnOperandChanged(NamedItemViewModel<PossibleOperands>? value)
    {
        OnPropertyChanged(nameof(OperandTooltip));
    }
    public string? OperandTooltip => Operand?.Name;

    [ObservableProperty]
    private IEnumerable<NamedItemViewModel<PossibleOperands>> _whenFirstIsOptions;

    [ObservableProperty]
    private ObservableCollection<NamedItemViewModel<PossibleOperands>> _whenFirstIs;
    partial void OnWhenFirstIsChanged(ObservableCollection<NamedItemViewModel<PossibleOperands>> value)
    {
        OnPropertyChanged(nameof(ButtonContent));
        OnPropertyChanged(nameof(TooltipContent));
    }

    [ObservableProperty]
    private bool _hideWhenFirstIs = false;

    public string? TooltipContent => GetContent("\n");
    public string? ButtonContent => GetContent(", ");
    private string GetContent(string separator)
        => WhenFirstIs.Count == 0 ? "Any" : string.Join(separator, WhenFirstIs.Select(x => x.Name));

    public ValidOperandViewModel(
        PossibleOperands operand,
        IEnumerable<PossibleOperands> whenFirstIs,
        bool hideWhenFirstIs)
    {
        var count = whenFirstIs.Where(x => x != PossibleOperands.Any).Count();
        OperandOptions = ValidOperand.Labels.Select(x => new NamedItemViewModel<PossibleOperands>(x.Value, x.Key));
        WhenFirstIsOptions = ValidOperand.Labels
            .Where(x => x.Key != PossibleOperands.Any)
            .Select(x => new NamedItemViewModel<PossibleOperands>(x.Value, x.Key));

        Operand = OperandOptions.FirstOrDefault(x => x.Item == operand);

        WhenFirstIs = new(WhenFirstIsOptions.Where(x => whenFirstIs.Contains(x.Item)));

        WhenFirstIs.CollectionChanged += WhenFirstIs_CollectionChanged;
        HideWhenFirstIs = hideWhenFirstIs;
    }

    private void WhenFirstIs_CollectionChanged(object? sender, NotifyCollectionChangedEventArgs e)
    {
        OnPropertyChanged(nameof(ButtonContent));
        OnPropertyChanged(nameof(TooltipContent));
    }
}
