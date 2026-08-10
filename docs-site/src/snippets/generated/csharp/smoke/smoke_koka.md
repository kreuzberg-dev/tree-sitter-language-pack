```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("fun main()\n  1\n", new ProcessConfig { Language = "koka" });

```
