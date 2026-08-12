---
id: fixture_csharp_smoke_plantuml
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("@startuml\n@enduml\n", new ProcessConfig { Language = "plantuml" });

```
