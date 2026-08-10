```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"avro\"}")
_ = try TreeSitterLanguagePack.process(source: "protocol P {\n}\n", config: configObj)

```
