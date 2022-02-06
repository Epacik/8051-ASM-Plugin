using Avalonia.Controls;
using ReactiveUI;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace QaD8051JDE.ViewModels;
internal class DocumentationElementListViewModel : ReactiveObject
{
    private TextBlock[]? elements;
    public TextBlock[]? Elements
    {
        get => elements;
        set => this.RaiseAndSetIfChanged(ref elements, value);
    }

    private TextBlock? selectedElement;
    public TextBlock? SelectedElement
    {
        get => selectedElement;
        set => this.RaiseAndSetIfChanged(ref selectedElement, value);
    }

    private Views.DocumentationElementEditor? editor;
    public Views.DocumentationElementEditor? Editor
    {
        get => editor;
        set => this.RaiseAndSetIfChanged(ref editor, value);
    }
}

