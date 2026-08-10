```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("@echo off\necho hello", new ProcessConfig { Language = "batch" });

```
