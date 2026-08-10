```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("FROM alpine", new ProcessConfig { Language = "dockerfile" });

```
