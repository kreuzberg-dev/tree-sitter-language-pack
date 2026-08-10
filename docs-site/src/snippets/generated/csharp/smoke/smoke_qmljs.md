```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("import QtQuick 2.0\nItem {}", new ProcessConfig { Language = "qmljs" });

```
