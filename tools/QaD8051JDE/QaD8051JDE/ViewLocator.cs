using Avalonia.Controls;
using Avalonia.Controls.Templates;
using Avalonia.Media;
using CommunityToolkit.Mvvm.ComponentModel;
using QaD8051JDE.ViewModels;
using System;
using System.Linq;
using System.Reflection;

namespace QaD8051JDE;

public class ViewLocator : IDataTemplate
{
    public IControl Build(object data)
    {
        try
        {
            var dataType = data.GetType();

            var name = dataType.FullName!.Replace("ViewModel", "View");
            var type = Type.GetType(name);

            if (type is null && name.Contains("`"))
            {
                name = name.Split("`")[0];
                type = Type.GetType(name);
            }

            if (type != null)
            {
                return (Control)Activator.CreateInstance(type)!;
            }
            else
            {
                return new TextBlock { Text = "Not Found: " + name };
            }
        }
        catch (Exception ex)
        {
            var stack = new StackPanel();
            stack.Children.Add(new TextBlock 
            { 
                Text = "An exception occured while locating the view", 
                Foreground = Brushes.DarkRed,
                FontWeight = FontWeight.SemiBold,
                FontSize = 20,
            });
            stack.Children.Add(new TextBox { Text = ex.ToString(), IsReadOnly = true });
            return stack;
        }
    }

    public bool Match(object? data)
    {
        return data is BaseViewModel;
    }
}

