```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("program main\nend program main", new ProcessConfig { Language = "fortran" });

```
