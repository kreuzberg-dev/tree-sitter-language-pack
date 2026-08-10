```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"wgsl_bevy\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
