```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("project('hello', 'c')", new ProcessConfig { Language = "meson" });

```
