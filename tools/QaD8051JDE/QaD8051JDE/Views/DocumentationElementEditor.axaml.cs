using Avalonia;
using Avalonia.Controls;
using Avalonia.Controls.ApplicationLifetimes;
using Avalonia.Interactivity;
using Avalonia.Markup.Xaml;
using MessageBox.Avalonia.Enums;
using System;
using System.Collections.Generic;
using System.Linq;

namespace QaD8051JDE.Views;
public partial class DocumentationElementEditor : UserControl
{
    readonly ViewModels.DocumentationElementEditorViewModel viewModel = new();
    private KeyValuePair<string, DocumentationElement> Element;

    public DocumentationElementEditor()
    {
        DataContext = viewModel;
        InitializeComponent();
    }

    public DocumentationElementEditor(KeyValuePair<string, DocumentationElement> element) : this()
    {
        this.Element = element;
        LoadElement();
    }

    private void LoadElement()
    {
        viewModel.Detail = Element.Value.Detail;
        viewModel.Description = Element.Value.Description;
        //viewModel.Syntax = Element.Value.Syntax;
        viewModel.AffectedFlags = Element.Value.AffectedFlags is not null ? new(Element.Value.AffectedFlags) : new();
        //viewModel.ValidOperands = Element.Value.ValidOperands is not null ? new (Element.Value.ValidOperands) : new();

        foreach(Flag flag in Element.Value.AffectedFlags)
        {
            var editor = new FlagEditor();
            editor.Set(flag);
            AffectedFlagsPanel.Children.Add(editor);
        }

        for (int i = 0; i < Element.Value.ValidOperands.Count; i++)
        {
            var operand = Element.Value.ValidOperands[i];
            var editor = new OperandsEditor(i);
            editor.Set(operand);
            OperandsPanel.Children.Add(editor);
        }

        DontGenerateSyntax.IsChecked = Element.Value.DontGenerateSyntax;
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
        var affectedFlags = AffectedFlagsPanel.Children.Select(x => ((FlagEditor)x).Get())
            .OrderBy(x => x.FlagType).ToList();

        var operands = OperandsPanel.Children
            .Select(
                x => ((OperandsEditor)x)
                    .Get()
                    .OrderBy(x => x.PossibleOperand.ToString())
                    .ToList())
            .ToList();

        SaveRequested?.Invoke(this, new SaveRequestedEventArgs(Element.Key, new DocumentationElement
        {
            Detail = viewModel.Detail,
            Description = viewModel.Description,
            //Syntax = viewModel.Syntax,
            AffectedFlags = affectedFlags,
            ValidOperands = operands,
            DontGenerateSyntax = DontGenerateSyntax.IsChecked == true,
        }));
    }

    private void AddFlagButton_Click(object sender, RoutedEventArgs args)
    {
        AffectedFlagsPanel.Children.Add(new FlagEditor());
    }

    private void RemoveFlagButton_Click(object sender, RoutedEventArgs args)
    {
        var editor = AffectedFlagsPanel.Children.FirstOrDefault(x => ((FlagEditor)x).IsChecked);
        if (editor is null)
            return;

        AffectedFlagsPanel.Children.Remove(editor);
    }

    private void AddOperandButton_Click(object sender, RoutedEventArgs args)
    {
        OperandsPanel.Children.Add(new OperandsEditor(OperandsPanel.Children.Count));
    }

    private void RemoveOperandButton_Click(object sender, RoutedEventArgs args)
    {
        var editor = OperandsPanel.Children.FirstOrDefault(x => ((OperandsEditor)x).IsChecked);
        if (editor is null)
            return;

        OperandsPanel.Children.Remove(editor);

        for(int i = 0; i < OperandsPanel.Children.Count; i++)
        {
            var child = (OperandsEditor)OperandsPanel.Children[i];
            child.SetOperatorNumber(i);
        }
    }

    private StackPanel AffectedFlagsPanel;
    private StackPanel OperandsPanel;
    private ToggleSwitch DontGenerateSyntax;

    private void InitializeComponent()
    {
        AvaloniaXamlLoader.Load(this);
        AffectedFlagsPanel = this.FindControl<StackPanel>("AffectedFlagsPanel");
        OperandsPanel = this.FindControl<StackPanel>("OperandsPanel");
        DontGenerateSyntax = this.FindControl<ToggleSwitch>("DontGenerateSyntax");
    }
}
