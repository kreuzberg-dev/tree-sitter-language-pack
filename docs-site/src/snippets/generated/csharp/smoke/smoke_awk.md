```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("BEGIN { print \"hello\" }", new ProcessConfig { Language = "awk" });

```
