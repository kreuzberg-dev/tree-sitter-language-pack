```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"slim\"}")
_ = try TreeSitterLanguagePack.process(source: "p hello\n", config: configObj)

```
