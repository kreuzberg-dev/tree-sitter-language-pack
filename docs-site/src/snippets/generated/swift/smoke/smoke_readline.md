```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"readline\"}")
_ = try TreeSitterLanguagePack.process(source: "set editing-mode vi", config: configObj)

```
