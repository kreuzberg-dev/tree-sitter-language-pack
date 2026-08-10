```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"astro\"}")
_ = try TreeSitterLanguagePack.process(source: "---\n---\n<p>hello</p>", config: configObj)

```
