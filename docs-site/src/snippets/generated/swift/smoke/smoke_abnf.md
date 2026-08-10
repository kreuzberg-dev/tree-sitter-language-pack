```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"abnf\"}")
_ = try TreeSitterLanguagePack.process(source: "a = \"b\"\r\n", config: configObj)

```
