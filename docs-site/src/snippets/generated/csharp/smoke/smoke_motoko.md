```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("actor {\n}\n", new ProcessConfig { Language = "motoko" });

```
