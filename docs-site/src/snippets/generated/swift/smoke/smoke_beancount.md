```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"beancount\"}")
_ = try TreeSitterLanguagePack.process(source: "2024-01-01 open Assets:Bank USD", config: configObj)

```
