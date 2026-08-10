```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("fn main {\n}\n", new ProcessConfig { Language = "moonbit" });

```
