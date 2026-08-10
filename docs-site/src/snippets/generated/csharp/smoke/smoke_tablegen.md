```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("def Hello : Base {}", new ProcessConfig { Language = "tablegen" });

```
