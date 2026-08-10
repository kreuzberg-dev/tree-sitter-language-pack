```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"twig\"}")
_ = try TreeSitterLanguagePack.process(source: "{{ variable }}", config: configObj)

```
