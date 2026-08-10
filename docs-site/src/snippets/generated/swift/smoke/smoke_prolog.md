```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"prolog\"}")
_ = try TreeSitterLanguagePack.process(source: "hello :- write('hello'), nl.", config: configObj)

```
