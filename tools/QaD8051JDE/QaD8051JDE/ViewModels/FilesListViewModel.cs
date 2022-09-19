using Avalonia.Controls;
using ReactiveUI;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace QaD8051JDE.ViewModels;
internal class FilesListViewModel : ReactiveObject
{
    private TextBlock[]? categories;
    public TextBlock[]? Categories
    {
        get => categories;
        set => this.RaiseAndSetIfChanged(ref categories, value);
    }

    private TextBlock? selectedCategory;
    public TextBlock? SelectedCategory
    {
        get => selectedCategory;
        set => this.RaiseAndSetIfChanged(ref selectedCategory, value);
    }

    private Views.DocumentationElementList? docElements;
    public Views.DocumentationElementList? DocumentationElements
    {
        get => docElements;
        set => this.RaiseAndSetIfChanged(ref docElements, value);
    }
}
