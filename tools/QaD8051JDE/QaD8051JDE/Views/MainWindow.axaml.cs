using Avalonia;
using Avalonia.Controls;
using Avalonia.Controls.ApplicationLifetimes;
using Avalonia.Interactivity;
using Avalonia.Markup.Xaml;
using System;
using System.IO;
using System.Linq;
using System.Threading.Tasks;

namespace QaD8051JDE.Views
{
    public partial class MainWindow : Window
    {
        readonly ViewModels.MainWindowViewModel viewModel = new();
        public MainWindow()
        {
            DataContext = viewModel;
            InitializeComponent();
#if DEBUG
            this.AttachDevTools();
#endif

            _ = SelectFolder();

            KeyDown += MainWindow_KeyDown;
        }

        private void MainWindow_KeyDown(object? sender, Avalonia.Input.KeyEventArgs e)
        {
            if(e.Key == Avalonia.Input.Key.S &&
               e.Modifiers == Avalonia.Input.InputModifiers.Control)
            {
                viewModel?.FilesList?.Save();
            }
        }

        private async Task SelectFolder()
        {
            string? path;
            if (Application.Current.ApplicationLifetime is IClassicDesktopStyleApplicationLifetime lifetime && lifetime.Args.Length > 0)
            {
                path = lifetime.Args[0];
            }
            else 
            {
                var dialog = new OpenFolderDialog()
                {
                    Title = "Select folder containing documentation",
                };
                path = await dialog.ShowAsync(this);
            }
            if(path is null || !Directory.Exists(path))
            {
                Environment.Exit(0);
                return;
            }

            viewModel.LanguageDirectories = Directory.GetDirectories(path)
                .Select(x => new TextBlock { Text = Path.GetFileName(x), Tag = x})
                .ToArray();

        }

        private void DirectoryContents_SelectionChanged(object sender, SelectionChangedEventArgs e)
        {
            var lang = ((e.AddedItems[0] as TextBlock)?.Tag as string ?? "");
            var list = new FilesList(lang);

            if(viewModel.FilesList is not null)
            {
                viewModel.FilesList.SelectedNameChanged -= List_SelectedNameChanged;
            }

            viewModel.FilesList = list;

            list.SelectedNameChanged += List_SelectedNameChanged;
            viewModel.Title = (e.AddedItems[0] as TextBlock)?.Text ?? "";
        }

        private void List_SelectedNameChanged(object? sender, string e)
        {
            viewModel.Title = (viewModel.SelectedLanguage?.Text ?? "") + " - " + e;
        }

        private void InitializeComponent()
        {
            AvaloniaXamlLoader.Load(this);
        }

    }
}
