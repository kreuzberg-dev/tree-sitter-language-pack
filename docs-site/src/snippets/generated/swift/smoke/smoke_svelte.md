```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"svelte\"}")
_ = try TreeSitterLanguagePack.process(source: "<script>let x = 1;</script>", config: configObj)

```
