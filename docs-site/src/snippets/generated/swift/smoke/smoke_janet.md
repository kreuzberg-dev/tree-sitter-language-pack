```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"janet\"}")
_ = try TreeSitterLanguagePack.process(source: "(print \"hello\")", config: configObj)

```
