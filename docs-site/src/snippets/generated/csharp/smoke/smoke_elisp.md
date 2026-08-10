```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("(defun hello () (message \"hello\"))", new ProcessConfig { Language = "elisp" });

```
