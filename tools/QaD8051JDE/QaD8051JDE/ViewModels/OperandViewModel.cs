using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using CommunityToolkit.Mvvm.ComponentModel;
using QaD8051JDE.DocumentationTypes;

namespace QaD8051JDE.ViewModels;

public class OperandEditorViewModel : OperandEditorViewModel<ValidOperand, PossibleOperands>
{
    public OperandEditorViewModel(
        bool hideWhenFirstIs,
        IDictionary<PossibleOperands, string> labels,
        Func<KeyValuePair<PossibleOperands, string>, bool> labelsFilter,
        PossibleOperands defaultValue) 
        : base(hideWhenFirstIs, labels, labelsFilter, defaultValue)
    {
    }
}

public abstract partial class OperandEditorViewModel<TJsonWrapper, TEnum> : ObservableObject
    where TJsonWrapper : class, IOperand<TEnum>, new()
    where TEnum : struct, IConvertible // enum? xD

{
    private readonly TEnum _defaultValue;
    private Func<KeyValuePair<TEnum, string>, bool> _labelsFilter;

    public OperandEditorViewModel(
        bool hideWhenFirstIs,
        IDictionary<TEnum, string> labels,
        Func<KeyValuePair<TEnum, string>, bool> labelsFilter,
        TEnum defaultValue)
    {
        if (!typeof(TEnum).IsEnum)
        {
            throw new InvalidOperationException($"{nameof(TEnum)} must be an enum");
        }

        _labelsFilter = labelsFilter;
        _defaultValue = defaultValue;
        HideWhenFirstIs = hideWhenFirstIs;

        LabelsCollection = new ObservableCollection<KeyValuePair<TEnum, string>>(labels);
        _whenFirstIsCollection = new ObservableCollection<KeyValuePair<TEnum, string>>(labels.Where(_labelsFilter));
        _selectedWhenFirstIsLabels = new();
    }

    [ObservableProperty]
    private bool _hideWhenFirstIs = false;

    [ObservableProperty]
    private ObservableCollection<KeyValuePair<TEnum, string>>? _labelsCollection;

    [ObservableProperty]
    private ObservableCollection<KeyValuePair<TEnum, string>> _selectedWhenFirstIsLabels;

    partial void OnSelectedWhenFirstIsLabelsChanged(ObservableCollection<KeyValuePair<TEnum, string>> value)
    {
        OnPropertyChanged(nameof(WhenButtonContent));
        OnPropertyChanged(nameof(WhenButtonTooltipContent));
    }

    public string WhenButtonContent
        => (SelectedWhenFirstIsLabels?.Count ?? 0) == 0
        ? "Any"
        : string.Join(", ", SelectedWhenFirstIsLabels?.Select(x => x.Value) ?? Array.Empty<string>());

    public string WhenButtonTooltipContent
        => (SelectedWhenFirstIsLabels?.Count ?? 0) == 0
        ? "Any"
        : string.Join("\n", SelectedWhenFirstIsLabels?.Select(x => x.Value) ?? Array.Empty<string>());


    [ObservableProperty]
    private ObservableCollection<KeyValuePair<TEnum, string>> _whenFirstIsCollection;

    [ObservableProperty]
    private int _validLabelSelectedIndex;

    public void Set(IGrouping<TEnum, TJsonWrapper> group)
    {
        ValidLabelSelectedIndex = LabelsCollection!
            .Select((e, p) => (e, p))
            .FirstOrDefault(x => x.e.Key.Equals(group.Key)).p;

        var first = group.Select(x => x.WhenFirstIs);
        if (!first.Any(x => !x.Equals(_defaultValue)))
            return;

        for (int i = 0; i < WhenFirstIsCollection.Count; i++)
        {
            if (first.Contains(WhenFirstIsCollection[i].Key))
                SelectedWhenFirstIsLabels!.Add(WhenFirstIsCollection[i]);
        }
    }

    public IEnumerable<TJsonWrapper> Get()
    {
        List<TJsonWrapper> result = new();

        var operand = LabelsCollection![ValidLabelSelectedIndex].Key;

        foreach (var (value, _) in SelectedWhenFirstIsLabels)
        {
            result.Add(new()
            {
                Operand = operand,
                WhenFirstIs = value,
            });
        }

        if(result.Count == 0)
        {
            result.Add(new()
            {
                Operand = operand,
                WhenFirstIs = _defaultValue,
            });
        }

        return result;
    }
}
