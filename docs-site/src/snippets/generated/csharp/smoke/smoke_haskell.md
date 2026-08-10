```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("main = putStrLn \"hello\"", new ProcessConfig { Language = "haskell" });

```
