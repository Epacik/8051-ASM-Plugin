using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;
using System;
using System.Linq;

namespace QaD8051JDE.Views
{
    public partial class FlagEditorView : UserControl
    {
        public FlagEditorView()
        {
            InitializeComponent();
        }

        //public void Set(Flag flag)
        //{
        //    Flags.SelectedIndex = (int)flag.FlagType;
        //    WhenSet.Text = flag.WhenSet;
        //    WhenUnset.Text = flag.WhenUnset;
        //}

        //public Flag Get()
        //{
        //    return new Flag
        //    {
        //        FlagType = (FlagType)Flags.SelectedIndex,
        //        WhenSet = WhenSet.Text ?? "",
        //        WhenUnset = WhenUnset.Text ?? "",
        //    };
        //}

    }
}
