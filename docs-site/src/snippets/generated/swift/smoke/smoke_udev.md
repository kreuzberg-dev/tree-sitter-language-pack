```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"udev\"}")
_ = try TreeSitterLanguagePack.process(source: "ACTION==\"add\", KERNEL==\"sd*\"", config: configObj)

```
