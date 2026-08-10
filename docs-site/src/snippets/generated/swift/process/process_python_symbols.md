```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"python\",\"symbols\":true}")
_ = try TreeSitterLanguagePack.process(source: "MY_CONST = 42\ndef helper(): pass\nclass Widget: pass\n", config: configObj)

```
