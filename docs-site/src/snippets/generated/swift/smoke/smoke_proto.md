```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"proto\"}")
_ = try TreeSitterLanguagePack.process(source: "syntax = \"proto3\";", config: configObj)

```
