using Avalonia.Controls;
using CommunityToolkit.Mvvm.ComponentModel;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Text.Json;
using System.Threading.Tasks;

namespace QaD8051JDE.ViewModels;
public partial class DocumentationElementListViewModel : ObservableObject
{
    [ObservableProperty]
    private NamedItemViewModel<DocumentationElement>[]? _elements;

    [ObservableProperty]
    private NamedItemViewModel<DocumentationElement>? _selectedElement;

    [ObservableProperty]
    private DocumentationElementEditorViewModel? _editor;

    private readonly string _filePath;

    public DocumentationElementListViewModel(string filePath)
    {
        _filePath = filePath;
        LoadJson();
    }

    private void LoadJson()
    {
        var content = File.ReadAllText(_filePath);
        Elements = JsonSerializer
            .Deserialize<Dictionary<string, DocumentationElement>>(content)?
            .Select(x => new NamedItemViewModel<DocumentationElement>(x.Key, x.Value))?
            .ToArray();
    }

    partial void OnSelectedElementChanged(NamedItemViewModel<DocumentationElement>? value)
    {
        if (value is null)
        {
            Editor = null;
            return;
        }

        Editor = new DocumentationElementEditorViewModel(value.Item);
    }
}

