```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"po\"}")
_ = try TreeSitterLanguagePack.process(source: "msgid \"hello\"\nmsgstr \"world\"", config: configObj)

```
