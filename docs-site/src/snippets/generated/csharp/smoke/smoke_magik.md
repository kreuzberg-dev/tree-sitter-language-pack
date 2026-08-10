```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("_method object.hello\n_endmethod", new ProcessConfig { Language = "magik" });

```
