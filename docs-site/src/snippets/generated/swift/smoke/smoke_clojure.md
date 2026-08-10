```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"clojure\"}")
_ = try TreeSitterLanguagePack.process(source: "(def x 1)", config: configObj)

```
