using Avalonia.Controls;
using CommunityToolkit.Mvvm.ComponentModel;
using QaD8051JDE.ViewModels;
using System.ComponentModel;

namespace QaD8051JDE.ViewModels;
public partial class MainWindowViewModel : BaseViewModel
{
    [ObservableProperty]
    private NamedItemViewModel<string>[]? _languageDirectories;

    [ObservableProperty]
    private NamedItemViewModel<string>? _selectedLanguage;

    [ObservableProperty]
    private FilesListViewModel? _fileList;

    public string Title => 
        ("QaD8051JDE: " + 
        (SelectedLanguage?.Name ?? "") + "/" +
        (FileList?.SelectedCategory?.Name ?? "") + "/" + 
        (FileList?.DocumentationElements?.SelectedElement?.Name ?? ""))
        .Trim('/', ':', ' ');

    partial void OnSelectedLanguageChanged(NamedItemViewModel<string>? value)
    {
        if (FileList is not null)
        {
            FileList.PropertyChanged -= FilesList_PropertyChanged;
        }

        if (value is null)
        {
            FileList = null;
            OnPropertyChanged(nameof(Title));
            return;
        }

        FileList = new(value.Item!);
        FileList.PropertyChanged += FilesList_PropertyChanged;

        OnPropertyChanged(nameof(Title));
    }

    private void FilesList_PropertyChanged(object? sender, PropertyChangedEventArgs e)
    {
        switch (e.PropertyName)
        {
            case nameof(FilesListViewModel.SelectedCategory):
            case nameof(FilesListViewModel.DocumentationElements):
                OnPropertyChanged(nameof(Title));
                break;
        }
    }
}