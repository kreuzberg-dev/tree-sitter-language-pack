```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("body { color: red; }", new ProcessConfig { Language = "css" });

```
