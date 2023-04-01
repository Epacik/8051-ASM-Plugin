using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace QaD8051JDE.Models;

public class PartialDocumentationListItemModel
{
    public PartialDocumentationListItemModel(string filename, string sharedPath, string mainPath)
    {
        _filename = filename;
        _sharedPath = sharedPath;
        _mainPath = mainPath;
    }


    private readonly string _filename;
    private readonly string _sharedPath;
    private readonly string _mainPath;

    public string SharedPath => Path.Combine(_sharedPath, _filename);
    public string MainPath => Path.Combine(_mainPath, _filename);
}
