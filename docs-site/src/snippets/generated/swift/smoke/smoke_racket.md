```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"racket\"}")
_ = try TreeSitterLanguagePack.process(source: "#lang racket\n(define x 1)", config: configObj)

```
