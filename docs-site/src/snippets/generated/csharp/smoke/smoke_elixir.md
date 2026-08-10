```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("IO.puts(\"hello\")", new ProcessConfig { Language = "elixir" });

```
