```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"editorconfig\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
