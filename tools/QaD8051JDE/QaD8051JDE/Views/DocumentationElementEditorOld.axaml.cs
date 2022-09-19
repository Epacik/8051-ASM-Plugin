using Avalonia;
using Avalonia.Controls;
using Avalonia.Controls.ApplicationLifetimes;
using Avalonia.Interactivity;
using Avalonia.Markup.Xaml;
using MessageBox.Avalonia.Enums;
using System;
using System.Collections.Generic;

namespace QaD8051JDE.Views;
public partial class DocumentationElementEditorOld : UserControl
{
    readonly ViewModels.OldDocumentationElementEditorViewModel viewModel = new();
    private KeyValuePair<string, DocumentationElement> Element;

    public DocumentationElementEditorOld()
    {
        DataContext = viewModel;
        InitializeComponent();
    }

    public DocumentationElementEditorOld(KeyValuePair<string, DocumentationElement> element) : this()
    {
        this.Element = element;
        LoadElement();
    }

    private void LoadElement()
    {
        viewModel.Detail = Element.Value.Detail;
        viewModel.Description = Element.Value.Description;
        //viewModel.Syntax = Element.Value.Syntax;
        //viewModel.AffectedFlags = Element.Value.AffectedFlags;
        //viewModel.ValidOperands = Element.Value.ValidOperands;
    }

    private async void RefreshButtonClick(object sender, RoutedEventArgs args)
    {
        if (Application.Current?.ApplicationLifetime is IClassicDesktopStyleApplicationLifetime desktop)
        {
            var messagebox = MessageBox.Avalonia.MessageBoxManager.GetMessageBoxStandardWindow("Are you sure?", "Are you sure?", ButtonEnum.YesNo, Icon.Info);


            if (await messagebox.ShowDialog(desktop.MainWindow) == ButtonResult.No)
                return;
        }
            
        LoadElement();
    }

    public event EventHandler<SaveRequestedEventArgs>? SaveRequested;

    private void SaveButtonClick(object sender, RoutedEventArgs args)
    {
        //SaveRequested?.Invoke(this, new SaveRequestedEventArgs(Element.Key, new DocumentationElement
        //{
        //    Detail = viewModel.Detail,
        //    Description = viewModel.Description,
        //    Syntax = viewModel.Syntax,
        //    AffectedFlags = viewModel.AffectedFlags,
        //    ValidOperands = viewModel.ValidOperands,
        //}));
    }

    private void InitializeComponent()
    {
        AvaloniaXamlLoader.Load(this);
    }
}

