---
id: fixture_csharp_smoke_solidity
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("pragma solidity ^0.8.0;\ncontract Main {}", new ProcessConfig { Language = "solidity" });

```
