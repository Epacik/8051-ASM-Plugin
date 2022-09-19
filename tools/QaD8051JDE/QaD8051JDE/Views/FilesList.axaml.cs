using Avalonia;
using Avalonia.Controls;
using Avalonia.Controls.ApplicationLifetimes;
using Avalonia.Interactivity;
using Avalonia.Markup.Xaml;
using MessageBox.Avalonia.DTO;
using MessageBox.Avalonia.Enums;
using System;
using System.IO;
using System.Linq;

namespace QaD8051JDE.Views
{
    public partial class FilesList : UserControl
    {
        readonly ViewModels.FilesListViewModel viewModel = new ();
        private readonly string LanguageFolderPath;

        public FilesList()
        {
            DataContext = viewModel;
            InitializeComponent();
        }

        public FilesList(string lang) : this()
        {
            LanguageFolderPath = lang;

            LoadFiles();
        }

        public event EventHandler<string> SelectedNameChanged;

        private void LoadFiles()
        {
            if (viewModel.DocumentationElements is not null)
            {
                viewModel.DocumentationElements.RemoveEditor();
            }

            var files = Directory.GetFiles(LanguageFolderPath);
            viewModel.Categories = files.Select(x => new TextBlock { Text = Path.GetFileName(x), Tag = x })
                .ToArray();
        }

        internal void Save()
        {
            viewModel?.DocumentationElements?.Save();
        }

        private void CategoryList_SelectionChanged(object sender, SelectionChangedEventArgs e)
        {
            if (e.AddedItems.Count == 0)
                return;
            var file = ((e.AddedItems[0] as TextBlock)?.Tag as string ?? "");

            if(viewModel.DocumentationElements is not null)
            {
                viewModel.DocumentationElements.SelectedNameChanged -= List_SelectedNameChanged;
            }
            var list = new DocumentationElementList(file);
            viewModel.DocumentationElements = list;
            list.SelectedNameChanged += List_SelectedNameChanged;

            SelectedNameChanged?.Invoke(this, (e.AddedItems[0] as TextBlock)?.Text ?? "");
        }

        private void List_SelectedNameChanged(object? sender, string e)
        {
            var categoryName = viewModel.SelectedCategory?.Text ?? "";
            SelectedNameChanged?.Invoke(this, categoryName + " - " + e);
        }

        private async void AddFileButtonClick(object sender, RoutedEventArgs args)
        {
            if (Application.Current?.ApplicationLifetime is IClassicDesktopStyleApplicationLifetime desktop)
            {
                var messagebox = MessageBox.Avalonia.MessageBoxManager.GetMessageBoxInputWindow(new MessageBoxInputParams()
                {
                    Icon = Icon.Plus,
                    InputDefaultValue = null,
                    ContentTitle = "Input a Name",
                    WindowStartupLocation = WindowStartupLocation.CenterScreen,
                });

                var result = (await messagebox.ShowDialog(desktop.MainWindow));
                if (result.Button == "Cancel" || result.Message is null)
                    return;

                var name = result.Message.Trim().ToLowerInvariant();
                if (!name.EndsWith(".json"))
                    name += ".json";

                File.WriteAllText(Path.Combine(LanguageFolderPath, name), "{}");

                LoadFiles();
            }
        }

        private async void RemoveFileButtonClick(object sender, RoutedEventArgs args)
        {
            if (viewModel.SelectedCategory is null || viewModel.SelectedCategory.Tag is null)
                return;

            if (Application.Current?.ApplicationLifetime is IClassicDesktopStyleApplicationLifetime desktop)
            {
                var messagebox = MessageBox.Avalonia.MessageBoxManager.GetMessageBoxStandardWindow("Are you sure?", "Are you sure?", ButtonEnum.YesNo, Icon.Info);


                if (await messagebox.ShowDialog(desktop.MainWindow) == ButtonResult.No)
                    return;
            }

            var fileToRemove = (string)viewModel.SelectedCategory.Tag;

            File.Delete(fileToRemove);
            LoadFiles();
        }

        private async void EditFileNameButtonClick(object sender, RoutedEventArgs args)
        {
            if (viewModel.SelectedCategory is null || viewModel.SelectedCategory.Tag is null)
                return;

            if (Application.Current?.ApplicationLifetime is IClassicDesktopStyleApplicationLifetime desktop)
            {
                var fileToEdit = viewModel.SelectedCategory.Text;
                int lastindex = fileToEdit.LastIndexOf('.');
                if(lastindex != -1)
                {
                    fileToEdit = fileToEdit.Substring(0, lastindex);
                }

                var messagebox = MessageBox.Avalonia.MessageBoxManager.GetMessageBoxInputWindow(new MessageBoxInputParams()
                {
                    Icon = Icon.Plus,
                    InputDefaultValue = fileToEdit,
                    ContentTitle = "Input a Name",
                    WindowStartupLocation = WindowStartupLocation.CenterScreen,

                });

                var result = (await messagebox.ShowDialog(desktop.MainWindow));
                if (result.Button == "Cancel" || result.Message is null)
                    return;

                var newFileName = result.Message;
                if (!newFileName.EndsWith(".json"))
                    newFileName += ".json";

                var oldPath = Path.Combine(LanguageFolderPath, fileToEdit + ".json");
                var newPath = Path.Combine(LanguageFolderPath, newFileName);
                File.Move(oldPath, newPath);
                LoadFiles();
            }
        }

        private void InitializeComponent()
        {
            AvaloniaXamlLoader.Load(this);
        }
    }
}
