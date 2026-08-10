```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"dot\"}")
_ = try TreeSitterLanguagePack.process(source: "digraph G { A -> B; }", config: configObj)

```
