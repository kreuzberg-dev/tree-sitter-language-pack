```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"bpftrace\"}")
_ = try TreeSitterLanguagePack.process(source: "BEGIN { }\n", config: configObj)

```
