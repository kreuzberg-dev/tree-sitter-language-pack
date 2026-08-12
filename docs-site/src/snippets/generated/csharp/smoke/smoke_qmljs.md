---
id: fixture_csharp_smoke_qmljs
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("import QtQuick 2.0\nItem {}", new ProcessConfig { Language = "qmljs" });

```
