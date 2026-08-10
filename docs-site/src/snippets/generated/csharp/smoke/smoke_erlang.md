```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("main() -> ok.", new ProcessConfig { Language = "erlang" });

```
