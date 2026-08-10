```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("function main() end", new ProcessConfig { Language = "julia" });

```
