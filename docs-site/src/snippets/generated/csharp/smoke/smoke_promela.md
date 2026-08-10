```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("init {\n}\n", new ProcessConfig { Language = "promela" });

```
