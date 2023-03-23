using Avalonia.Controls;
using CommunityToolkit.Mvvm.ComponentModel;
using Mapster;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Text.Json;
using System.Text.Json.Serialization;
using System.Threading.Tasks;

namespace QaD8051JDE.ViewModels;
public partial class DocumentationElementListViewModel : BaseViewModel
{
    [ObservableProperty]
    private NamedItemViewModel<DocumentationElementEditorViewModel>[]? _elements;

    [ObservableProperty]
    private NamedItemViewModel<DocumentationElementEditorViewModel>? _selectedElement;

    private DocumentationElementEditorViewModel? Editor => SelectedElement?.Item;

    private readonly string _filePath;

    public DocumentationElementListViewModel(string filePath)
    {
        _filePath = filePath;
        LoadJson();
    }

    private void LoadJson()
    {
        var content = File.ReadAllText(_filePath);
        Dictionary<string, DocumentationElement>? deserialized = null;
        try
        {
            deserialized = JsonSerializer.Deserialize<Dictionary<string, DocumentationElement>>(content);
        }
        catch
        {
            deserialized = JsonSerializer.Deserialize<Dictionary<string, DocumentationElement>>(content, _options);
        }
        Elements = deserialized?
            .Select(x => new NamedItemViewModel<DocumentationElementEditorViewModel>(
                x.Key,
                new DocumentationElementEditorViewModel(x.Value)))
            ?.ToArray();
    }

    private readonly static JsonSerializerOptions _options = new JsonSerializerOptions
    {
        WriteIndented = true,
        Converters =
        {
            new JsonStringEnumConverter(JsonNamingPolicy.CamelCase),
        }
    };
    public void SaveJson()
    {
        if (Elements is null)
            return;

        var elements = Elements!.ToDictionary(
            key => key.Name ?? "LOST",
            value => value.Item!.AsDocumentationElement());

        var content = JsonSerializer.Serialize(elements, _options);
        File.WriteAllText(_filePath, content);
        LoadJson();
    }

    public void RevertJson()
    {
        LoadJson();
    }

    partial void OnSelectedElementChanged(NamedItemViewModel<DocumentationElementEditorViewModel>? value)
    {
        OnPropertyChanged(nameof(Editor));
    }
}

