```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"kitty\"}")
_ = try TreeSitterLanguagePack.process(source: "font_size 12\n", config: configObj)

```
