```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("probe begin {}\n", new ProcessConfig { Language = "systemtap" });

```
