```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"rego\"}")
_ = try TreeSitterLanguagePack.process(source: "package main\ndefault allow = false", config: configObj)

```
