```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"rust\"}")
_ = try TreeSitterLanguagePack.process(source: "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n", config: configObj)

```
