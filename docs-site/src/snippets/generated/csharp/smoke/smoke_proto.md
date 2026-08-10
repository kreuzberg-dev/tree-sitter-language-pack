```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("syntax = \"proto3\";", new ProcessConfig { Language = "proto" });

```
