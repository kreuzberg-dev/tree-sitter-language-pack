```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("extends Node\nfunc _ready():\n\tpass", new ProcessConfig { Language = "gdscript" });

```
