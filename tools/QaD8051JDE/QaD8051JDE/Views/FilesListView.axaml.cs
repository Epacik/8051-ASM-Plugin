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
    public partial class FilesListView : UserControl
    {
        private ViewModels.FilesListViewModel? ViewModel => DataContext as ViewModels.FilesListViewModel;
        private readonly string? _languageFolderPath;

        public FilesListView()
        {
            InitializeComponent();
        }

        public FilesListView(string lang) : this()
        {
            _languageFolderPath = lang;

            //LoadFiles();
        }

        //public event EventHandler<string> SelectedNameChanged;

        //private void LoadFiles()
        //{
        //    if (ViewModel?.DocumentationElements is not null)
        //    {
        //        //ViewModel?.DocumentationElements.RemoveEditor();
        //    }

        //    var files = Directory.GetFiles(_languageFolderPath!);
        //    //ViewModel?.Categories = files.Select(x => new TextBlock { Text = Path.GetFileName(x), Tag = x })
        //    //    .ToArray();
        //}

        //internal void Save()
        //{
        //    //ViewModel?.DocumentationElements?.Save();
        //}

        ////private void CategoryList_SelectionChanged(object sender, SelectionChangedEventArgs e)
        ////{
        ////    if (e.AddedItems.Count == 0)
        ////        return;
        ////    var file = ((e.AddedItems[0] as TextBlock)?.Tag as string ?? "");

        ////    if(ViewModel.DocumentationElements is not null)
        ////    {
        ////        //ViewModel.DocumentationElements.SelectedNameChanged -= List_SelectedNameChanged;
        ////    }
        ////    var list = new DocumentationElementListView(file);
        ////    ViewModel.DocumentationElements = list;
        ////    list.SelectedNameChanged += List_SelectedNameChanged;

        ////    SelectedNameChanged?.Invoke(this, (e.AddedItems[0] as TextBlock)?.Text ?? "");
        ////}

        //private void List_SelectedNameChanged(object? sender, string e)
        //{
        //    //var categoryName = ViewModel.SelectedCategory?.Text ?? "";
        //    //SelectedNameChanged?.Invoke(this, categoryName + " - " + e);
        //}

        //private async void AddFileButtonClick(object sender, RoutedEventArgs args)
        //{
        //    if (Application.Current?.ApplicationLifetime is IClassicDesktopStyleApplicationLifetime desktop)
        //    {
        //        var messagebox = MessageBox.Avalonia.MessageBoxManager.GetMessageBoxInputWindow(new MessageBoxInputParams()
        //        {
        //            Icon = Icon.Plus,
        //            InputDefaultValue = null,
        //            ContentTitle = "Input a Name",
        //            WindowStartupLocation = WindowStartupLocation.CenterScreen,
        //        });

        //        var result = (await messagebox.ShowDialog(desktop.MainWindow));
        //        if (result.Button == "Cancel" || result.Message is null)
        //            return;

        //        var name = result.Message.Trim().ToLowerInvariant();
        //        if (!name.EndsWith(".json"))
        //            name += ".json";

        //        File.WriteAllText(Path.Combine(_languageFolderPath!, name), "{}");

        //        LoadFiles();
        //    }
        //}

        //private async void RemoveFileButtonClick(object sender, RoutedEventArgs args)
        //{
        //    //if (ViewModel.SelectedCategory is null || ViewModel.SelectedCategory.Tag is null)
        //    //    return;

        //    //if (Application.Current?.ApplicationLifetime is IClassicDesktopStyleApplicationLifetime desktop)
        //    //{
        //    //    var messagebox = MessageBox.Avalonia.MessageBoxManager.GetMessageBoxStandardWindow("Are you sure?", "Are you sure?", ButtonEnum.YesNo, Icon.Info);


        //    //    if (await messagebox.ShowDialog(desktop.MainWindow) != ButtonResult.Yes)
        //    //        return;
        //    //}

        //    //var fileToRemove = (string)ViewModel.SelectedCategory.Tag;

        //    //File.Delete(fileToRemove);
        //    //LoadFiles();
        //}

        //private async void EditFileNameButtonClick(object sender, RoutedEventArgs args)
        //{
        //    //if (ViewModel.SelectedCategory is null || ViewModel.SelectedCategory.Tag is null)
        //    //    return;

        //    //if (Application.Current?.ApplicationLifetime is IClassicDesktopStyleApplicationLifetime desktop)
        //    //{
        //    //    var fileToEdit = ViewModel.SelectedCategory.Text;
        //    //    int lastindex = fileToEdit.LastIndexOf('.');
        //    //    if(lastindex != -1)
        //    //    {
        //    //        fileToEdit = fileToEdit.Substring(0, lastindex);
        //    //    }

        //    //    var messagebox = MessageBox.Avalonia.MessageBoxManager.GetMessageBoxInputWindow(new MessageBoxInputParams()
        //    //    {
        //    //        Icon = Icon.Plus,
        //    //        InputDefaultValue = fileToEdit,
        //    //        ContentTitle = "Input a Name",
        //    //        WindowStartupLocation = WindowStartupLocation.CenterScreen,

        //    //    });

        //    //    var result = (await messagebox.ShowDialog(desktop.MainWindow));
        //    //    if (result.Button == "Cancel" || result.Message is null)
        //    //        return;

        //    //    var newFileName = result.Message;
        //    //    if (!newFileName.EndsWith(".json"))
        //    //        newFileName += ".json";

        //    //    var oldPath = Path.Combine(_languageFolderPath!, fileToEdit + ".json");
        //    //    var newPath = Path.Combine(_languageFolderPath!, newFileName);
        //    //    File.Move(oldPath, newPath);
        //    //    LoadFiles();
        //    //}
        //}

        private void InitializeComponent()
        {
            AvaloniaXamlLoader.Load(this);
        }
    }
}
