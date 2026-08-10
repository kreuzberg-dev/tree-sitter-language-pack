```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("group(\"hello\") {}", new ProcessConfig { Language = "gn" });

```
