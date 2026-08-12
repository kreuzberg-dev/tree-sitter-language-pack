---
id: fixture_swift_kotlin_package_class_process
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"kotlin\"}")
_ = try TreeSitterLanguagePack.process(source: "package foo.bar\n\nclass Widget {\n    fun greet(): String = \"hi\"\n}\n", config: configObj)

```
