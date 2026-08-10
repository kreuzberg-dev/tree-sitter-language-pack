```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("module Main where", new ProcessConfig { Language = "purescript" });

```
