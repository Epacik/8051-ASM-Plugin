﻿using ReactiveUI;
using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace QaD8051JDE.ViewModels;

internal class DocumentationElementEditorViewModel : ReactiveObject
{
    private string? detail;
    public string? Detail
    {
        get => detail;
        set => this.RaiseAndSetIfChanged(ref detail, value);
    }

    private string? description;
    public string? Description
    {
        get => description;
        set => this.RaiseAndSetIfChanged(ref description, value);
    }

    private ObservableCollection<Flag>? affectedFlags;
    public ObservableCollection<Flag>? AffectedFlags
    {
        get => affectedFlags;
        set => this.RaiseAndSetIfChanged(ref affectedFlags, value);
    }

    private ObservableCollection<List<string>>? validOperands;
    public ObservableCollection<List<string>>? ValidOperands
    {
        get => validOperands;
        set => this.RaiseAndSetIfChanged(ref validOperands, value);
    }
}
