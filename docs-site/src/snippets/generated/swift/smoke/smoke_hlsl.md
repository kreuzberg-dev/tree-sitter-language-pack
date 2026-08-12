---
id: fixture_swift_smoke_hlsl
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"hlsl\"}")
_ = try TreeSitterLanguagePack.process(source: "float4 main() : SV_Target { return 0; }", config: configObj)

```
