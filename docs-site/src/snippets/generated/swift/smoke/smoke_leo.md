```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"leo\"}")
_ = try TreeSitterLanguagePack.process(source: "program test.aleo {\n}\n", config: configObj)

```
