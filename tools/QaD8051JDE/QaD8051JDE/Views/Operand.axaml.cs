using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;
using Avalonia.Styling;
using DynamicData;
using FluentAvalonia.Core;
using System;
using System.Collections.Generic;
using System.Linq;

namespace QaD8051JDE.Views;

public class Operand : UserControl
{
    //private ComboBox When;
    private ComboBox ValidOperands;
    private TextBlock Label;
    private DropDownButton WhenButton;
    private ListBox WhenListBox;
    private ToolTip WhenTooltip;
    private ToolTip ValidOperandsTooltip;

    public Operand() : this(false) { }

    public Operand(bool hideWhenFirstIs)
    {
        InitializeComponent();
        var operands = ValidOperand.Labels
            .OrderBy(x => x.Value)
            .ToDictionary(x => x.Key, x => x.Value);

        Label!.IsVisible = !hideWhenFirstIs;
        WhenButton!.IsVisible = !hideWhenFirstIs;

        ValidOperands!.Items = operands.Select(x => new TextBlock { Text = x.Value, Tag = x.Key});
        WhenListBox!.Items = operands
            .Where(x => x.Key != PossibleOperands.Any)
            .Select(x => new TextBlock { Text = x.Value, Tag = x.Key });

    }

    private void InitializeComponent()
    {
        AvaloniaXamlLoader.Load(this);
        //When          = this.FindControl<ComboBox>("When")             ?? throw new InvalidOperationException(nameof(When) + " not found");
        ValidOperands        = this.FindControl<ComboBox>("ValidOperands")
            ?? throw new InvalidOperationException(nameof(ValidOperands) + " not found");

        Label                = this.FindControl<TextBlock>("Label")
            ?? throw new InvalidOperationException(nameof(Label) + " not found");

        WhenButton           = this.FindControl<DropDownButton>("WhenButton")
            ?? throw new InvalidOperationException(nameof(WhenButton) + " not found");

        WhenListBox          = this.FindControl<ListBox>("WhenListBox")
            ?? throw new InvalidOperationException(nameof(WhenListBox) + " not found");
        WhenListBox.SelectionMode = SelectionMode.Multiple | SelectionMode.Toggle;

        WhenTooltip          = this.FindControl<ToolTip>("WhenTooltip")
            ?? throw new InvalidOperationException(nameof(WhenTooltip) + " not found");

        ValidOperandsTooltip = this.FindControl<ToolTip>("ValidOperandsTooltip")
            ?? throw new InvalidOperationException(nameof(ValidOperandsTooltip) + " not found");
    }

    public void ValidOperands_SelectionChanged(object sender, SelectionChangedEventArgs args)
    {
        ValidOperandsTooltip.Content = (ValidOperands.SelectedItem as TextBlock)?.Text ?? "";
    }

    public void WhenListBox_SelectionChanged(object sender, SelectionChangedEventArgs args)
    {
        if (WhenListBox.SelectedItems!.Count == 0)
        {
            WhenButton.Content  = "Any";
            WhenTooltip.Content = "Any";
        }
        else
        {
            WhenButton.Content  = string.Join(", ", WhenListBox.SelectedItems!.Cast<TextBlock>().Select(x => x.Text));
            WhenTooltip.Content = string.Join("\n", WhenListBox.SelectedItems!.Cast<TextBlock>().Select(x => x.Text));
        }
    }

    internal void Set(IGrouping<PossibleOperands, ValidOperand> item)
    {
        ValidOperands.SelectedIndex = ValidOperands.Items!
            .Cast<TextBlock>()
            .Select((element, position) => (element, position))
            .FirstOrDefault(x => (x.element.Tag as PossibleOperands?) == item.Key)
            .position;



        var first = item.Select(x => x.WhenFirstIs);
        if (first.Count(x => x != PossibleOperands.Any) == 0)
            return;

        int i = 0;
        foreach(TextBlock x in WhenListBox.Items!)
        {
            if (first.Contains((PossibleOperands)x.Tag!))
                WhenListBox.Selection.Select(i);
            i++;
        }
    }

    internal IEnumerable<ValidOperand> Get()
    {
        List<ValidOperand> operands = new();
        var operand = ValidOperands.SelectedIndex > -1 ? (PossibleOperands)((TextBlock)ValidOperands.SelectedItem!).Tag! : PossibleOperands.Accumulator;
        //var when = When.SelectedIndex > -1 ? (PossibleOperands)((TextBlock)When.SelectedItem!).Tag! : PossibleOperands.Any;

        foreach(TextBlock selected in WhenListBox.SelectedItems!)
        {
            operands.Add(new()
            {
                PossibleOperand = operand,
                WhenFirstIs = (PossibleOperands)selected.Tag!,
            });
        }

        if(operands.Count == 0)
        {
            operands.Add(new()
            {
                PossibleOperand = operand,
                WhenFirstIs = PossibleOperands.Any,
            });
        }

        return operands;
    }
}
