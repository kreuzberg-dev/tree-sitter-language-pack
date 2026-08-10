```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"yuck\"}")
_ = try TreeSitterLanguagePack.process(source: "(defwidget main [] (label :text \"hi\"))", config: configObj)

```
