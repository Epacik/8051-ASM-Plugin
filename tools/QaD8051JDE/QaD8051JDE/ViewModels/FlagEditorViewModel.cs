using Avalonia.Controls;
using CommunityToolkit.Mvvm.ComponentModel;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace QaD8051JDE.ViewModels;

public partial class FlagEditorViewModel : ObservableObject
{
    [ObservableProperty]
    private NamedItemViewModel<FlagType>[]? _allFlags;

    [ObservableProperty]
    private NamedItemViewModel<FlagType> _flagType;

    [ObservableProperty]
    private string? _whenSet;

    [ObservableProperty]
    private string? _whenUnset;

    public FlagEditorViewModel()
    {
        AllFlags = ((FlagType[])Enum.GetValues(typeof(FlagType)))
                .Select(x => new NamedItemViewModel<FlagType>(GetName(x), x))
                .ToArray();
    }

    private static string GetName(FlagType x) => $"{(int)x}: {x}";

    public FlagEditorViewModel(Flag x) : this()
    {
        FlagType type;
        (type, WhenSet, WhenUnset) = x;

        FlagType = new(GetName(type), type);
    }
}
