using Avalonia.Controls;
using CommunityToolkit.Mvvm.ComponentModel;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace QaD8051JDE.ViewModels;
public partial class FilesListViewModel : ObservableObject
{
    private readonly string _languagePath;
    [ObservableProperty]
    private NamedItemViewModel<string>[]? _categories;

    [ObservableProperty]
    private NamedItemViewModel<string>? _selectedCategory;

    [ObservableProperty]
    private DocumentationElementListViewModel? _documentationElements;

    public FilesListViewModel(string languagePath)
    {
        _languagePath = languagePath;
        Load(languagePath);
    }

    partial void OnSelectedCategoryChanged(NamedItemViewModel<string>? value)
    {
        if (value is null)
        {
            DocumentationElements = null;
            return;
        }

        DocumentationElements = new DocumentationElementListViewModel(value.Item!);
    }

    private void Load(string lang)
    {
        var files = Directory.GetFiles(lang!);
        Categories = files.Select(x => new NamedItemViewModel<string>(Path.GetFileName(x), x))
            .ToArray();
    }
}
