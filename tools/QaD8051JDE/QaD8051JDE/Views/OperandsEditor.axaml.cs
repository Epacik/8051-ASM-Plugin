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
        public OperandsEditor(int operandNumber)
        {
            InitializeComponent();
            SetOperandNumber(operandNumber);
        }

        public void SetOperandNumber(int operandNumber)
        {
            Title.Text = $"Operand {operandNumber}";
            this.operandNumber = operandNumber;
        }

        public bool IsChecked => Selection.IsChecked == true;

        public void Set(List<ValidOperand> values)
        {
            ValidValues.BeginBatchUpdate();

            var items = (ObservableCollection<Operand>)(ValidValues.Items!);
            
            foreach(IGrouping<PossibleOperands, ValidOperand> item in values.GroupBy(x => x.Operand))
            {
                var operand = new Operand(operandNumber == 0);
                operand.Set(item);
                items.Add(operand);
            }

            //foreach (ValidOperand item in values)
            //{
            //    var operand = new Operand(operandNumber == 0);
            //    operand.Set(item);
            //    items.Add(operand);
            //}

            ValidValues.EndBatchUpdate();
        }

        public List<ValidOperand> Get()
        {
            var list = new List<ValidOperand>();
            var items = (ObservableCollection<Operand>)(ValidValues.Items!);

            foreach (Operand item in items)
            {
                list.AddRange(item.Get());
            }
            return list;
        }

        private void AddValueButton_Click(object sender, RoutedEventArgs args)
        {
            ValidValues.BeginBatchUpdate();

            var items = (ObservableCollection<Operand>)(ValidValues.Items!);
            items.Add(new Operand(operandNumber == 0));

            ValidValues.EndBatchUpdate();
        }

        private void RemoveValueButton_Click(object sender, RoutedEventArgs args)
        {
            var index = ValidValues.SelectedIndex;
            if (index == -1)
                return;

            ValidValues.BeginBatchUpdate();

            var items = (ObservableCollection<Operand>)(ValidValues.Items!);
            var item = items.ElementAt(index);
            items.Remove(item);

            ValidValues.EndBatchUpdate();
            
        }
        private int operandNumber;
    }
}
