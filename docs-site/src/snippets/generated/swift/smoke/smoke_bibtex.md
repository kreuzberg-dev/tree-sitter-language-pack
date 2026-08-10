```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"bibtex\"}")
_ = try TreeSitterLanguagePack.process(source: "@article{key, title={A}}", config: configObj)

```
