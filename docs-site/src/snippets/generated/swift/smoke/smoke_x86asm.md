```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"x86asm\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
