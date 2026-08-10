```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"markdown\"}")
_ = try TreeSitterLanguagePack.process(source: "# Hello\n\nWorld", config: configObj)

```
