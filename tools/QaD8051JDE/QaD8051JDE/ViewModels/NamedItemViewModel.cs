using CommunityToolkit.Mvvm.ComponentModel;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace QaD8051JDE.ViewModels;

public partial class NamedItemViewModel<T> : BaseViewModel, IEquatable<NamedItemViewModel<T>?>
{
    [ObservableProperty]
    private string? _name;

    [ObservableProperty]
    private T? _item;

    [ObservableProperty]
    private bool _isDirty;

    public NamedItemViewModel(string name, T item)
    {
        Name = name;
        Item = item;

        if (item is BaseViewModel bvm)
        {
            bvm.PropertyChanged += Bvm_PropertyChanged;
        }
    }

    private void Bvm_PropertyChanged(object? sender, System.ComponentModel.PropertyChangedEventArgs e)
    {
        IsDirty = true;
    }

    public override bool Equals(object? obj)
    {
        return Equals(obj as NamedItemViewModel<T>);
    }

    public bool Equals(NamedItemViewModel<T>? other)
    {
        return other is not null &&
               Name == other.Name;
    }

    public override int GetHashCode()
    {
        return HashCode.Combine(Name);
    }

    public static bool operator ==(NamedItemViewModel<T>? left, NamedItemViewModel<T>? right)
    {
        return EqualityComparer<NamedItemViewModel<T>>.Default.Equals(left, right);
    }

    public static bool operator !=(NamedItemViewModel<T>? left, NamedItemViewModel<T>? right)
    {
        return !(left == right);
    }
}
