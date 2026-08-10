```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"gstlaunch\"}")
_ = try TreeSitterLanguagePack.process(source: "fakesrc ! fakesink", config: configObj)

```
