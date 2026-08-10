```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"doxygen\"}")
_ = try TreeSitterLanguagePack.process(source: "/** @brief A function */", config: configObj)

```
