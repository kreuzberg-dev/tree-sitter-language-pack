```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("hello :- write('hello'), nl.", new ProcessConfig { Language = "prolog" });

```
