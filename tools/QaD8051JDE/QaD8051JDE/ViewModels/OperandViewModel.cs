using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using QaD8051JDE.DocumentationTypes;
using ReactiveUI;

namespace QaD8051JDE.ViewModels;

public class OperandViewModel<TJsonWrapper, TEnum> : ReactiveObject
    where TJsonWrapper : class, IOperand<TEnum>, new()
    where TEnum : struct, IConvertible, IEquatable<TEnum> // enum? xD

{
    public OperandViewModel(
        bool hideWhenFirstIs,
        IDictionary<TEnum, string> labels,
        Func<KeyValuePair<TEnum, string>, bool> labelsFilter,
        TEnum defaultValue)
    {
        if (!typeof(TEnum).IsEnum)
        {
            throw new InvalidOperationException($"{nameof(TEnum)} must be an enum");
        }

        LabelsFilter = labelsFilter;
        _defaultValue = defaultValue;
        HideWhenFirstIs = hideWhenFirstIs;

        ValidLabels = new ObservableCollection<KeyValuePair<TEnum, string>>(labels);
        _whenFirstIsCollection = new ObservableCollection<KeyValuePair<TEnum, string>>(labels.Where(LabelsFilter));
        _selectedWhenFirstIsLabels = new();
    }

    private bool _hideWhenFirstIs = false;

    public Func<KeyValuePair<TEnum, string>, bool> LabelsFilter { get; }
    public bool HideWhenFirstIs
    {
        get => _hideWhenFirstIs;
        set => this.RaiseAndSetIfChanged(ref _hideWhenFirstIs, value, nameof(HideWhenFirstIs));
    }
    private ObservableCollection<KeyValuePair<TEnum, string>>? _labelsCollection;
    public ObservableCollection<KeyValuePair<TEnum, string>>? ValidLabels
    {
        get => _labelsCollection;
        set => this.RaiseAndSetIfChanged(ref _labelsCollection, value);
    }

    private ObservableCollection<KeyValuePair<TEnum, string>> _selectedWhenFirstIsLabels;
    public ObservableCollection<KeyValuePair<TEnum, string>> SelectedWhenFirstIsLabels
    {
        get => _selectedWhenFirstIsLabels;
        set
        {
            this.RaiseAndSetIfChanged(ref _selectedWhenFirstIsLabels, value);
            this.RaisePropertyChanged(nameof(WhenButtonContent));
            this.RaisePropertyChanged(nameof(WhenButtonTooltipContent));
        }
    }

    public string WhenButtonContent
        => (SelectedWhenFirstIsLabels?.Count ?? 0) == 0
        ? "Any"
        : string.Join(", ", SelectedWhenFirstIsLabels?.Select(x => x.Value) ?? Array.Empty<string>());

    public string WhenButtonTooltipContent
        => (SelectedWhenFirstIsLabels?.Count ?? 0) == 0
        ? "Any"
        : string.Join("\n", SelectedWhenFirstIsLabels?.Select(x => x.Value) ?? Array.Empty<string>());

    private ObservableCollection<KeyValuePair<TEnum, string>> _whenFirstIsCollection;
    public ObservableCollection<KeyValuePair<TEnum, string>> WhenFirstIsCollection
    { 
        get => _whenFirstIsCollection; 
        private set => this.RaiseAndSetIfChanged(ref _whenFirstIsCollection, value);
    }

    private int _validLabelSelectedIndex;
    private readonly TEnum _defaultValue;

    public int ValidLabelSelectedIndex
    {
        get => _validLabelSelectedIndex;
        set => this.RaiseAndSetIfChanged(ref _validLabelSelectedIndex, value);
    }



    public void Set(IGrouping<TEnum, TJsonWrapper> group)
    {
        ValidLabelSelectedIndex = ValidLabels!
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

        var operand = ValidLabels[ValidLabelSelectedIndex].Key;

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
