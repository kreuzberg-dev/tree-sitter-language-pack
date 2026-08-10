```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("actor Main\n  new create(env: Env) => None", new ProcessConfig { Language = "pony" });

```
