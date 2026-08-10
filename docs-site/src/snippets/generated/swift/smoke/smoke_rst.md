```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"rst\"}")
_ = try TreeSitterLanguagePack.process(source: "Hello\n=====\n\nWorld", config: configObj)

```
