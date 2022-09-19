using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;
using System;
using System.IO;
using System.Text.Json;
using System.Collections.Generic;
using System.Linq;
using Avalonia.Interactivity;
using Avalonia.Controls.ApplicationLifetimes;
using MessageBox.Avalonia.Enums;
using MessageBox.Avalonia.DTO;

namespace QaD8051JDE.Views
{
    public partial class DocumentationElementList : UserControl
    {
        readonly ViewModels.DocumentationElementListViewModel viewModel = new();
        private readonly string DocumentationFilePath;
        private Dictionary<string, DocumentationElement>? Elements;

        public DocumentationElementList()
        {
            DataContext = viewModel;
            InitializeComponent();
        }

        public DocumentationElementList(string lang) : this()
        {
            this.DocumentationFilePath = lang;
            LoadJson();
        }

        private void LoadJson()
        {
            var content = File.ReadAllText(DocumentationFilePath);
            var elements = JsonSerializer.Deserialize<Dictionary<string, DocumentationElement>>(content);
            Elements = elements;
            viewModel.Elements = elements?.Select(x => new TextBlock { Text = x.Key, Tag = x })?.ToArray();
        }

        internal void Save()
        {
            viewModel?.Editor?.Save();
        }

        public event EventHandler<string> SelectedNameChanged;

        private void ElementList_SelectionChanged(object sender, SelectionChangedEventArgs e)
        {
            if (e.AddedItems.Count == 0)
                return;

            var selected = (e.AddedItems[0] as TextBlock);
            if (selected is null || selected.Tag is null)
                return;


            if(viewModel.Editor is not null)
            {
                viewModel.Editor.SaveRequested -= Editor_SaveRequested;
            }
            var element = (KeyValuePair<string, DocumentationElement>)selected.Tag;
            var editor = new DocumentationElementEditor(element);
            editor.SaveRequested += Editor_SaveRequested;
            viewModel.Editor = editor;

            SelectedNameChanged?.Invoke(this, element.Key);
        }

        private void Editor_SaveRequested(object? sender, SaveRequestedEventArgs e)
        {
            if (Elements is null)
                return;

            var elements = Elements;

            elements[e.Mnemonic ?? "LOST"] = e.NewValue;



            var content = JsonSerializer.Serialize(elements, new JsonSerializerOptions { WriteIndented = true, });
            File.WriteAllText(DocumentationFilePath, content);

            LoadJson();
        }


        private async void AddMnemonicButtonClick(object sender, RoutedEventArgs args)
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

                Editor_SaveRequested(this, new SaveRequestedEventArgs(result.Message.ToUpperInvariant(), new DocumentationElement()
                {
                    AffectedFlags = new(),
                    Description = "",
                    Detail = "",
                    //Syntax = "",
                    ValidOperands = new(),
                }));

            }
        }

        private async void RemoveMnemonicButtonClick(object sender, RoutedEventArgs args)
        {
            if (Elements is null || viewModel.SelectedElement is null || viewModel.SelectedElement.Tag is null)
                return;

            if (Application.Current?.ApplicationLifetime is IClassicDesktopStyleApplicationLifetime desktop)
            {
                var messagebox = MessageBox.Avalonia.MessageBoxManager.GetMessageBoxStandardWindow("Are you sure?", "Are you sure?", ButtonEnum.YesNo, Icon.Info);


                if (await messagebox.ShowDialog(desktop.MainWindow) == ButtonResult.No)
                    return;
            }

            var mnemonicToRemove = (KeyValuePair<string, DocumentationElement>)viewModel.SelectedElement.Tag;

            var elements = Elements;
            elements.Remove(mnemonicToRemove.Key);

            var content = JsonSerializer.Serialize(elements, new JsonSerializerOptions { WriteIndented = true, });
            File.WriteAllText(DocumentationFilePath, content);

            RemoveEditor();

            LoadJson();
        }

        private async void EditMnemonicButtonClick(object sender, RoutedEventArgs args)
        {
            if (Elements is null || viewModel.SelectedElement is null || viewModel.SelectedElement.Tag is null)
                return;

            if (Application.Current?.ApplicationLifetime is IClassicDesktopStyleApplicationLifetime desktop)
            {
                var mnemonicToEdit = (KeyValuePair<string, DocumentationElement>)viewModel.SelectedElement.Tag;

                var messagebox = MessageBox.Avalonia.MessageBoxManager.GetMessageBoxInputWindow(new MessageBoxInputParams()
                {
                    Icon = Icon.Plus,
                    InputDefaultValue = mnemonicToEdit.Key,
                    ContentTitle = "Input a Name",
                    WindowStartupLocation = WindowStartupLocation.CenterScreen,
                    
                });

                var result = (await messagebox.ShowDialog(desktop.MainWindow));
                if (result.Button == "Cancel" || result.Message is null)
                    return;

                var elements = Elements;
                var element = elements[mnemonicToEdit.Key];
                elements.Remove(mnemonicToEdit.Key);
                elements[result.Message.ToUpper()] = element;

                var content = JsonSerializer.Serialize(elements, new JsonSerializerOptions { WriteIndented = true, });
                File.WriteAllText(DocumentationFilePath, content);

                RemoveEditor();

                LoadJson();
            }
        }

        public void RemoveEditor()
        {
            if (viewModel.Editor is not null)
            {
                viewModel.Editor.SaveRequested -= Editor_SaveRequested;
            }
            viewModel.Editor = null;
        }

        private void InitializeComponent()
        {
            AvaloniaXamlLoader.Load(this);
        }
    }
}
