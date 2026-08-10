```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("(defwidget main [] (label :text \"hi\"))", new ProcessConfig { Language = "yuck" });

```
