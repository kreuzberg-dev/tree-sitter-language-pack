```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"bsl\"}")
_ = try TreeSitterLanguagePack.process(source: "Procedure Main() EndProcedure", config: configObj)

```
