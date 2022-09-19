using Avalonia.Controls;
using ReactiveUI;

namespace QaD8051JDE.ViewModels;
public class MainWindowViewModel : ReactiveObject
{
    private TextBlock[]? languageDirectories;
    public TextBlock[]? LanguageDirectories
    {
        get => languageDirectories;
        set => this.RaiseAndSetIfChanged(ref languageDirectories, value);
    }

    private TextBlock? selectedLanguage;
    public TextBlock? SelectedLanguage
    {
        get => selectedLanguage;
        set => this.RaiseAndSetIfChanged(ref selectedLanguage, value);
    }

    private Views.FilesList? filesList;
    public Views.FilesList? FilesList
    {
        get => filesList;
        set => this.RaiseAndSetIfChanged(ref filesList, value);
    }

    private string? title;
    public string? Title
    {
        get => "QaD8051JDE: " + title;
        set => this.RaiseAndSetIfChanged(ref title, value);
    }

}