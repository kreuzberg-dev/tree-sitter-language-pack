```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("def main : IO Unit := pure ()", new ProcessConfig { Language = "lean" });

```
