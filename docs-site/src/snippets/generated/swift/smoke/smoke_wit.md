```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"wit\"}")
_ = try TreeSitterLanguagePack.process(source: "package example:pkg;", config: configObj)

```
