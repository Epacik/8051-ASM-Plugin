using Avalonia;
using Avalonia.Controls;
using CommunityToolkit.Mvvm.ComponentModel;
using Mapster;
using DocumentationTypes.PartialDocumentationElement;
using QaD8051JDE.Models;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Text.Json;
using System.Text.Json.Serialization;
using System.Threading.Tasks;
using DocumentationTypes;

namespace QaD8051JDE.ViewModels;
public partial class DocumentationElementListViewModel : BaseViewModel
{
    [ObservableProperty]
    private NamedItemViewModel<DocumentationElementEditorViewModel>[]? _elements;

    [ObservableProperty]
    private NamedItemViewModel<DocumentationElementEditorViewModel>? _selectedElement;

    private DocumentationElementEditorViewModel? Editor => SelectedElement?.Item;

    private readonly PartialDocumentationListItemModel _file;

    public event EventHandler? Saved;

    public DocumentationElementListViewModel(PartialDocumentationListItemModel fileModel)
    {
        _file = fileModel;
        LoadJson();
    }

    private void LoadJson()
    {
        var content = File.Exists(_file.MainPath) ? File.ReadAllText(_file.MainPath) : "{}";
        var sharedContent = File.Exists(_file.SharedPath) ? File.ReadAllText(_file.SharedPath) : "{}";

        Dictionary<string, DocumentationElement>? deserialized = null;

        //try
        //{
        //    deserialized = JsonSerializer.Deserialize<Dictionary<string, DocumentationElement>>(content, _options);
        //}
        //catch (Exception)
        //{
            var main = JsonSerializer.Deserialize<Dictionary<string, MainDocumentationElement>>(content, _options);
            var shared = JsonSerializer.Deserialize<Dictionary<string, SharedDocumentationElement>>(sharedContent, _options);

            deserialized = new();

            if (main is not null)
            {
                foreach (var (key, value) in main)
                {
                    if (!deserialized.ContainsKey(key))
                        deserialized[key] = new DocumentationElement();

                    deserialized[key].Detail = value.Detail;
                    deserialized[key].Description = value.Description;
                    deserialized[key].Prefix = value.Prefix;
                    deserialized[key].Label = value.Label;
                    deserialized[key].AffectedFlags = value.AffectedFlags
                        ?.Select(x => new Flag()
                        {
                            FlagType = x.Key,
                            WhenSet = x.Value.WhenSet,
                            WhenUnset = x.Value.WhenUnset,
                        })
                        .ToList();
                }
            }

            if (shared is not null)
            {
                foreach (var (key, value) in shared)
                {
                    if (!deserialized.ContainsKey(key))
                        deserialized[key] = new DocumentationElement();

                    deserialized[key].ValidOperands = value.ValidOperands;
                    deserialized[key].DontDuplicate = value.DontDuplicate;
                    deserialized[key].DontGenerateSyntax = value.DontGenerateSyntax;
                    deserialized[key].PrefixRequired = value.PrefixRequired;
                    deserialized[key].AddressingModes = value.AddressingModes;

                    deserialized[key].AffectedFlags ??= new List<Flag>();

                    var additionalFlags = value.AffectedFlags
                        ?.Where(x => !deserialized[key].AffectedFlags!.Any(y => y.FlagType == x))
                        ?.ToList();

                    if (additionalFlags is not null && additionalFlags.Count > 0)
                        deserialized[key].AffectedFlags!.AddRange(additionalFlags.Select(x => new Flag { FlagType = x }));

                    deserialized[key].UsedRegisters = value.UsedRegisters;
                    deserialized[key].ChangedRegisters = value.ChangedRegisters;
                    deserialized[key].StackSpaceNeeded = value.StackSpaceNeeded;
                }
            }
        //}


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

        var mainElements = elements.ToDictionary(
            key => key.Key,
            value => new MainDocumentationElement
            {
                Detail = value.Value.Detail,
                Description = value.Value.Description,
                Prefix = value.Value.Prefix,
                Label = value.Value.Label,
                AffectedFlags = value.Value.AffectedFlags?.ToDictionary(
                    x => x.FlagType,
                    x => new FlagDescription 
                    {
                        WhenSet = x.WhenSet,
                        WhenUnset = x.WhenUnset
                    }),
            });

        var sharedElements = elements.ToDictionary(
            key => key.Key,
            val => new SharedDocumentationElement
            {
                ValidOperands = val.Value.ValidOperands,
                AffectedFlags = val.Value.AffectedFlags?.Select(x => x.FlagType).ToList(),
                DontGenerateSyntax = val.Value.DontGenerateSyntax,
                DontDuplicate = val.Value.DontDuplicate,
                PrefixRequired = val.Value.PrefixRequired,
                AddressingModes = val.Value.AddressingModes,
                UsedRegisters = val.Value.UsedRegisters,
                ChangedRegisters = val.Value.ChangedRegisters,
                StackSpaceNeeded = val.Value.StackSpaceNeeded,
            });

        var mainContent = JsonSerializer.Serialize(mainElements, _options);
        File.WriteAllText(_file.MainPath, mainContent);

        var sharedContent = JsonSerializer.Serialize(sharedElements, _options);
        File.WriteAllText(_file.SharedPath, sharedContent);
        LoadJson();

        Saved?.Invoke(this, EventArgs.Empty);

        foreach (var element in Elements)
        {
            element.IsDirty = false;
        }
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
