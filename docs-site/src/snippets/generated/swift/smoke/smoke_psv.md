```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"psv\"}")
_ = try TreeSitterLanguagePack.process(source: "a|b|c\n1|2|3", config: configObj)

```
