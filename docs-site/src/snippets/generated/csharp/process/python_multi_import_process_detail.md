```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("import os\nimport sys\nfrom pathlib import Path\n\ndef main():\n    pass\n", new ProcessConfig { Language = "python" });

```
