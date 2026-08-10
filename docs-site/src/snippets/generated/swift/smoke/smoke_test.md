```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"test\"}")
_ = try TreeSitterLanguagePack.process(source: "===========\nTest\n===========\n---\n(node)", config: configObj)

```
