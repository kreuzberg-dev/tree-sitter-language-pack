```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"po\"}")
_ = try TreeSitterLanguagePack.process(source: "msgid \"Hello\"\nmsgstr \"Hallo\"\n", config: configObj)

```
