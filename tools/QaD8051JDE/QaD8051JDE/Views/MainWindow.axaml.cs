using Avalonia;
using Avalonia.Controls;
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
        }

        private async Task SelectFolder()
        {
            var dialog = new OpenFolderDialog()
            {
                Title = "Select folder containing documentation",
            };
            var path = await dialog.ShowAsync(this);
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
            viewModel.FilesList = list;
        }


        private void InitializeComponent()
        {
            AvaloniaXamlLoader.Load(this);
        }

    }
}
