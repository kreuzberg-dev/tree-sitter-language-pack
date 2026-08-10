```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("val x : int", new ProcessConfig { Language = "ocaml_interface" });

```
