```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"reason\"}")
_ = try TreeSitterLanguagePack.process(source: "let x = 1;\n", config: configObj)

```
