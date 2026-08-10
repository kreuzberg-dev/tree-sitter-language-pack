```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("--- a/file\n+++ b/file\n@@ -1 +1 @@\n-old\n+new", new ProcessConfig { Language = "diff" });

```
