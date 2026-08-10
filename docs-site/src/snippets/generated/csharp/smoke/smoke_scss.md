```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("$color: red;\nbody { color: $color; }", new ProcessConfig { Language = "scss" });

```
