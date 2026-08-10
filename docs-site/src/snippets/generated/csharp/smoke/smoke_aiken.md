```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("fn main() {\n  1\n}\n", new ProcessConfig { Language = "aiken" });

```
