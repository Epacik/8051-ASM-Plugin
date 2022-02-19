using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;
using System;
using System.Linq;

namespace QaD8051JDE.Views
{
    public partial class Operand : UserControl
    {
        private ComboBox When;
        private ComboBox ValidOperands;

        public Operand()
        {
            InitializeComponent();
            var operands = ValidOperand.Labels
                .OrderBy(x => x.Value)
                .ToDictionary(x => x.Key, x => x.Value);

            ValidOperands.Items = operands.Select(x => new TextBlock { Text = x.Value, Tag = x.Key});
            When.Items = operands.Select(x => new TextBlock { Text = x.Value, Tag = x.Key });
        }

        private void InitializeComponent()
        {
            AvaloniaXamlLoader.Load(this);
            When = this.FindControl<ComboBox>("When");
            ValidOperands = this.FindControl<ComboBox>("ValidOperands");
        }

        internal void Set(ValidOperand item)
        {
            ValidOperands.SelectedIndex = ValidOperands.Items
                .Cast<TextBlock>()
                .Select((element, position) => (element, position))
                .FirstOrDefault(x => ((PossibleOperands)x.element.Tag) == item.PossibleOperand)
                .position;

            When.SelectedIndex = ValidOperands.Items
                .Cast<TextBlock>()
                .Select((element, position) => (element, position))
                .FirstOrDefault(x => ((PossibleOperands)x.element.Tag) == item.WhenFirstIs)
                .position;
        }

        internal ValidOperand Get()
        {
            var operand = ValidOperands.SelectedIndex > -1 ? (PossibleOperands)((TextBlock)ValidOperands.SelectedItem).Tag : PossibleOperands.Accumulator;
            var when = When.SelectedIndex > -1 ? (PossibleOperands)((TextBlock)When.SelectedItem).Tag : PossibleOperands.Any;

            return new()
            {
                PossibleOperand = operand,
                WhenFirstIs = when,
            };
        }
    }
}
