---
id: fixture_csharp_smoke_php
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("<?php echo 'hello'; ?>", new ProcessConfig { Language = "php" });

```
