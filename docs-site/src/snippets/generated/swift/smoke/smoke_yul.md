```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"yul\"}")
_ = try TreeSitterLanguagePack.process(source: "object \"C\" {\n  code {\n  }\n}\n", config: configObj)

```
