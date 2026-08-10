```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"odin\"}")
_ = try TreeSitterLanguagePack.process(source: "package main", config: configObj)

```
