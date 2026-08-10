```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("mov eax, 1", new ProcessConfig { Language = "asm" });

```
