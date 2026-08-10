```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("config FOO\n\tbool \"Enable foo\"", new ProcessConfig { Language = "kconfig" });

```
