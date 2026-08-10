```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"vue\"}")
_ = try TreeSitterLanguagePack.process(source: "<template><div>hello</div></template>", config: configObj)

```
