```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"heex\"}")
_ = try TreeSitterLanguagePack.process(source: "<%= @greeting %>", config: configObj)

```
