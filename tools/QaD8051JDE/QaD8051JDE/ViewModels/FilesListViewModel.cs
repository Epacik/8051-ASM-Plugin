using Avalonia.Controls;
using CommunityToolkit.Mvvm.ComponentModel;
using QaD8051JDE.Models;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using CategoryViewModel = QaD8051JDE.ViewModels.NamedItemViewModel<QaD8051JDE.Models.PartialDocumentationListItemModel>;

namespace QaD8051JDE.ViewModels;
public partial class FilesListViewModel : BaseViewModel
{
    private readonly string _languagePath;
    private readonly string _sharedPath;
    [ObservableProperty]
    private CategoryViewModel[]? _categories;

    [ObservableProperty]
    private CategoryViewModel? _selectedCategory;

    [ObservableProperty]
    private DocumentationElementListViewModel? _documentationElements;

    public FilesListViewModel(string languagePath, string sharedPath)
    {
        _languagePath = languagePath;
        _sharedPath = sharedPath;
        Load(languagePath, sharedPath);
    }

    partial void OnSelectedCategoryChanged(CategoryViewModel? value)
    {
        if (DocumentationElements is not null)
        {
            DocumentationElements.PropertyChanged -= DocumentationElements_PropertyChanged;
        }
        if (value is null)
        {
            DocumentationElements = null;
            return;
        }

        DocumentationElements = new DocumentationElementListViewModel(value.Item!);
        DocumentationElements.PropertyChanged += DocumentationElements_PropertyChanged;
    }

    private void DocumentationElements_PropertyChanged(object? sender, System.ComponentModel.PropertyChangedEventArgs e)
    {
        OnPropertyChanged(nameof(DocumentationElements));
    }

    private void Load(string langPath, string sharedPath)
    {
        var files = Directory.GetFiles(langPath!).Select(f => Path.GetFileName(f));
        var sharedFiles = Directory.GetFiles(sharedPath).Select(f => Path.GetFileName(f));

        Categories = files
            .Concat(sharedFiles)
            .Distinct()
            .OrderBy(x => x)
            .Select(x => new CategoryViewModel(x, new(x, sharedPath, langPath)))
            .ToArray();
    }
}
