---
id: fixture_swift_smoke_smali
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"smali\"}")
_ = try TreeSitterLanguagePack.process(source: ".class public LMain;\n.super Ljava/lang/Object;", config: configObj)

```
