```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("general { border_size = 1 }", new ProcessConfig { Language = "hyprlang" });

```
