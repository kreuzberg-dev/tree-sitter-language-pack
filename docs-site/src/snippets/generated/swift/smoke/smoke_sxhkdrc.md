```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"sxhkdrc\"}")
_ = try TreeSitterLanguagePack.process(source: "super + a\n\techo hi\n", config: configObj)

```
