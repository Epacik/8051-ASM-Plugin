using Avalonia.Controls;
using Avalonia.Controls.Templates;
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

    public bool Match(object data)
    {
        return data is ObservableObject;
    }
}

