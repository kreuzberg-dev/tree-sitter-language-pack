```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("SECTIONS { .text : { *(.text) } }", new ProcessConfig { Language = "linkerscript" });

```
