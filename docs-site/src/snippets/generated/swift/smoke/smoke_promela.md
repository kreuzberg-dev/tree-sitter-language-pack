```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"promela\"}")
_ = try TreeSitterLanguagePack.process(source: "init {\n}\n", config: configObj)

```
