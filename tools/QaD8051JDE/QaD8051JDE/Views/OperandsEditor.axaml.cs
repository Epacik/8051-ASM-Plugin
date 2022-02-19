using Avalonia;
using Avalonia.Controls;
using Avalonia.Interactivity;
using Avalonia.Markup.Xaml;
using System;
using System.Collections;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;

namespace QaD8051JDE.Views
{
    public partial class OperandsEditor : UserControl
    {
        public OperandsEditor() : this(0)
        {

        }
        public OperandsEditor(int operatorNumber)
        {
            InitializeComponent();
            SetOperatorNumber(operatorNumber);
        }

        public void SetOperatorNumber(int operatorNumber)
        {
            Title.Text = $"Operand {operatorNumber}";
        }

        public bool IsChecked => Selection.IsChecked == true;

        public void Set(List<ValidOperand> values)
        {
            ValidValues.BeginBatchUpdate();

            var items = (ObservableCollection<Operand>)(ValidValues.Items);
            
            foreach (ValidOperand item in values)
            {
                var operand = new Operand();
                operand.Set(item);
                items.Add(operand);
            }

            ValidValues.EndBatchUpdate();
        }

        public List<ValidOperand> Get()
        {
            var list = new List<ValidOperand>();
            var items = (ObservableCollection<Operand>)(ValidValues.Items);

            foreach (Operand item in items)
            {
                list.Add(item.Get());
            }
            return list;

        }

        private void AddValueButton_Click(object sender, RoutedEventArgs args)
        {
            ValidValues.BeginBatchUpdate();

            var items = (ObservableCollection<Operand>)(ValidValues.Items);
            items.Add(new Operand());

            ValidValues.EndBatchUpdate();
        }

        private void RemoveValueButton_Click(object sender, RoutedEventArgs args)
        {
            var index = ValidValues.SelectedIndex;
            if (index == -1)
                return;

            ValidValues.BeginBatchUpdate();

            var items = (ObservableCollection<Operand>)(ValidValues.Items);
            var item = items.ElementAt(index);
            items.Remove(item);

            ValidValues.EndBatchUpdate();
            
        }



        private TextBlock Title;
        private RadioButton Selection;
        private ListBox ValidValues;

        private void InitializeComponent()
        {
            AvaloniaXamlLoader.Load(this);

            Title = this.FindControl<TextBlock>("Title");
            Selection = this.FindControl<RadioButton>("Selection");
            ValidValues = this.FindControl<ListBox>("ValidValues");
            ValidValues.Items = new ObservableCollection<Operand>();
        }
    }
}
