```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"clarity\"}")
_ = try TreeSitterLanguagePack.process(source: "(define-public (hello) (ok true))", config: configObj)

```
