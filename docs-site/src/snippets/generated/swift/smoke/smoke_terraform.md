```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"terraform\"}")
_ = try TreeSitterLanguagePack.process(source: "resource \"null_resource\" \"main\" {}", config: configObj)

```
