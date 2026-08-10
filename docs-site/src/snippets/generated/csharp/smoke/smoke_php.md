```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("<?php echo 'hello'; ?>", new ProcessConfig { Language = "php" });

```
