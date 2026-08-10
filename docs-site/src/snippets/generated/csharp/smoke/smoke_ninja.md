```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("rule cc\n  command = cc $in -o $out", new ProcessConfig { Language = "ninja" });

```
