```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"java\"}")
_ = try TreeSitterLanguagePack.process(source: "package com.example.widget;\n\npublic class Widget {\n    public String name() { return \"w\"; }\n}\n", config: configObj)

```
