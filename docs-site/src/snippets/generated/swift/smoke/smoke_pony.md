```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"pony\"}")
_ = try TreeSitterLanguagePack.process(source: "actor Main\n  new create(env: Env) => None", config: configObj)

```
