```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("export fn main() void = void;", new ProcessConfig { Language = "hare" });

```
