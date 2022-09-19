using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;
using System;
using System.Linq;

namespace QaD8051JDE.Views
{
    public partial class FlagEditor : UserControl
    {
        public FlagEditor()
        {
            InitializeComponent();

            Flags.Items = ((FlagType[])Enum.GetValues(typeof(FlagType)))
                .Select(x => new TextBlock { Text = $"{(int)x}: {x}", Tag = x });
        }

        public void Set(Flag flag)
        {
            Flags.SelectedIndex = (int)flag.FlagType;
            WhenSet.Text = flag.WhenSet;
            WhenUnset.Text = flag.WhenUnset;
        }

        public Flag Get()
        {
            return new Flag
            {
                FlagType = (FlagType)Flags.SelectedIndex,
                WhenSet = WhenSet.Text ?? "",
                WhenUnset = WhenUnset.Text ?? "",
            };
        }

        public bool IsChecked => Selection.IsChecked == true;

        private RadioButton Selection;
        private ComboBox Flags;
        private TextBox WhenSet;
        private TextBox WhenUnset;
        private void InitializeComponent()
        {
            AvaloniaXamlLoader.Load(this);

            Selection = this.FindControl<RadioButton>("Selection");
            Flags = this.FindControl<ComboBox>("Flags");
            WhenSet = this.FindControl<TextBox>("WhenSet");
            WhenUnset = this.FindControl<TextBox>("WhenUnset");
        }
    }
}
