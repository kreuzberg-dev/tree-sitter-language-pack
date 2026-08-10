```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("SELECT 1;\n", new ProcessConfig { Language = "tsql" });

```
