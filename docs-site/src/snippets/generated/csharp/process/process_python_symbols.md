```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("MY_CONST = 42\ndef helper(): pass\nclass Widget: pass\n", new ProcessConfig { Language = "python", Symbols = true });

```
