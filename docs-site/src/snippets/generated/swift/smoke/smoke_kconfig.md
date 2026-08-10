```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"kconfig\"}")
_ = try TreeSitterLanguagePack.process(source: "config FOO\n\tbool \"Enable foo\"", config: configObj)

```
