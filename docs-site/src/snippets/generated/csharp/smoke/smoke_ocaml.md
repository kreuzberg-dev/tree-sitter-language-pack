```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("let () = print_endline \"hello\"", new ProcessConfig { Language = "ocaml" });

```
