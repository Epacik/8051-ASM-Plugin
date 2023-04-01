using Avalonia;
using Avalonia.Controls;
using Avalonia.Controls.ApplicationLifetimes;
using Avalonia.Interactivity;
using Avalonia.Markup.Xaml;
using Avalonia.Platform.Storage;
using QaD8051JDE.ViewModels;
using System;
using System.IO;
using System.Linq;
using System.Threading.Tasks;

namespace QaD8051JDE.Views
{
    public partial class MainWindow : Window
    {
        private ViewModels.MainWindowViewModel? ViewModel => DataContext as ViewModels.MainWindowViewModel;
        public MainWindow()
        {
            DataContext = new ViewModels.MainWindowViewModel();
            InitializeComponent();

            _ = SelectFolder();

            KeyDown += MainWindow_KeyDown;
        }

        private void MainWindow_KeyDown(object? sender, Avalonia.Input.KeyEventArgs e)
        {
            if(e.Key == Avalonia.Input.Key.S &&
               e.KeyModifiers == Avalonia.Input.KeyModifiers.Control)
            {
                ViewModel?.FileList?.DocumentationElements?.SaveJson();
            }
        }

        private async Task SelectFolder()
        {
            string? path = null;
            if (Application.Current!.ApplicationLifetime is IClassicDesktopStyleApplicationLifetime lifetime && lifetime.Args!.Length > 0)
            {
                path = lifetime.Args[0];
            }
            else 
            {
                var dialog = await StorageProvider.OpenFolderPickerAsync(new FolderPickerOpenOptions
                {
                    Title = "Select folder containing documentation",
                });
                if (dialog?.FirstOrDefault()?.TryGetUri(out Uri? uri) ?? false)
                {
                    path = uri.LocalPath;
                }
                
            }
            if(path is null || !Directory.Exists(path))
            {
                Environment.Exit(0);
                return;
            }

            ViewModel!.LanguageDirectories = Directory.GetDirectories(path)
                .Select(x => new NamedItemViewModel<string>(Path.GetFileName(x), x))
                .Where(x => x.Name != ".shared")
                .ToArray();

            ViewModel!.SharedPath = Path.Combine(path, ".shared");

        }

        //private void DirectoryContents_SelectionChanged(object sender, SelectionChangedEventArgs e)
        //{
        //    var lang = ((e.AddedItems[0] as NamedItemViewModel)?.Path as string ?? "");
        //    var list = new FilesListViewModel(lang);

        //    if(ViewModel?.FilesList is not null)
        //    {
        //       // ViewModel.FilesList.SelectedNameChanged -= List_SelectedNameChanged;
        //    }

        //    ViewModel!.FilesList = list;

        //    //list.SelectedNameChanged += List_SelectedNameChanged;
        //    ViewModel?.SetTitle((e.AddedItems[0] as TextBlock)?.Text ?? "");
        //}

        private void List_SelectedNameChanged(object? sender, string e)
        {
            //ViewModel?.SetTitle((ViewModel?.SelectedLanguage?.Name ?? "") + " - " + e);
        }
    }
}
