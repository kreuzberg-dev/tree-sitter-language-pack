```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"erlang\"}")
_ = try TreeSitterLanguagePack.process(source: "main() -> ok.", config: configObj)

```
