using ReactiveUI;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace QaD8051JDE.ViewModels
{
    internal class OldDocumentationElementEditorViewModel : ReactiveObject
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

        private string? syntax;
        public string? Syntax
        {
            get => syntax;
            set => this.RaiseAndSetIfChanged(ref syntax, value);
        }

        private string? affectedFlags;
        public string? AffectedFlags
        {
            get => affectedFlags;
            set => this.RaiseAndSetIfChanged(ref affectedFlags, value);
        }

        private string? validOperands;
        public string? ValidOperands
        {
            get => validOperands;
            set => this.RaiseAndSetIfChanged(ref validOperands, value);
        }
    }
}
