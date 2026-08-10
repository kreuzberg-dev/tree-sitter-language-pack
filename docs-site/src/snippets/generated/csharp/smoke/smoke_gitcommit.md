```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("feat: add feature\n\nBody text", new ProcessConfig { Language = "gitcommit" });

```
