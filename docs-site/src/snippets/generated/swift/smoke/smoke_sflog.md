```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"sflog\"}")
_ = try TreeSitterLanguagePack.process(source: "37.0 APEX_CODE,DEBUG\n16:06:58.18 (1)|EXECUTION_STARTED\n", config: configObj)

```
