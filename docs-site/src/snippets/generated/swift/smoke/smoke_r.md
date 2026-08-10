```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"r\"}")
_ = try TreeSitterLanguagePack.process(source: "print('hello')", config: configObj)

```
