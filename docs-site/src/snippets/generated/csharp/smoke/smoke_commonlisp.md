```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("(defun hello () (print \"hello\"))", new ProcessConfig { Language = "commonlisp" });

```
