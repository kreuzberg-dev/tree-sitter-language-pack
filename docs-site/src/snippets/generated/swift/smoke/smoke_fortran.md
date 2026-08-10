```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"fortran\"}")
_ = try TreeSitterLanguagePack.process(source: "program main\nend program main", config: configObj)

```
