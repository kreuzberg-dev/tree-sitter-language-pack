```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"jsdoc\"}")
_ = try TreeSitterLanguagePack.process(source: "/** @param {string} name */", config: configObj)

```
