```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"fennel\"}")
_ = try TreeSitterLanguagePack.process(source: "(fn hello [] (print :hello))", config: configObj)

```
