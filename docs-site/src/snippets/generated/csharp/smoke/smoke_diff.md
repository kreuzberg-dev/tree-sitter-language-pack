---
id: fixture_csharp_smoke_diff
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("--- a/file\n+++ b/file\n@@ -1 +1 @@\n-old\n+new", new ProcessConfig { Language = "diff" });

```
